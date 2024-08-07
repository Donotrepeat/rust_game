use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use tokio::time::{self, interval};

mod ui;
use crate::ui::{ui, Ground, Level, Midde, Player};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut player = Player {
        x: -120.0,
        y: 12.0,
        dy: -5.0,
    };

    let _res = run_app(&mut terminal, &mut player).await;
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, player: &mut Player) -> io::Result<bool> {
    //test ground level vector
    let mut v = std::iter::repeat_with(|| Ground { x: 1.0, hight: 3 })
        .take(10)
        .collect::<Vec<_>>();

    for i in 0..10 {
        v[i].x = -180.0 + 10.0 * v[i].x * i as f64;
        v[i].hight = i as u8;
    }

    //test mid level
    let m = std::iter::repeat_with(|| Midde { x: 1.0, level: 6 })
        .take(30)
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut jump = true;
    let mut jump_count = 0;
    let mut level_index = 0;

    let mut level = Level {
        ground: v,
        middle: m,
    };
    let array: [Level; 3] = [level.clone(), level.clone(), level.clone()];

    let tick_rate = Duration::from_millis(20);
    let mut last_tick = Instant::now();

    let mut interval = interval(tick_rate);

    loop {
        terminal.draw(|f| ui(f, player, &mut level.ground, &mut level.middle))?;

        let current_level = &array[level_index];

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Up => {
                        if !player.on_block(&current_level.ground)
                            && !player.on_block_m(&current_level.middle)
                        {
                            player.dy = 2.0;
                            jump = true;
                            jump_count = count;
                        }
                    }
                    KeyCode::Down => {
                        if !player.on_block(&current_level.ground)
                            && !player.on_block_m(&current_level.middle)
                        {
                            player.dy = -2.0;
                        }
                    }
                    KeyCode::Right => {
                        if !player.nextto_block(&current_level.ground)
                            && !player.nextto_block_m(&current_level.middle)
                        {
                            player.x += 1.0;
                        }
                    }
                    KeyCode::Left => {
                        if !player.nextto_block(&current_level.ground)
                            && !player.nextto_block_m(&current_level.middle)
                        {
                            player.x -= 1.0;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        interval.tick().await;

        if jump == true && (count - jump_count) >= 15 {
            jump = false;
            player.dy = -2.0;
        }

        if player.x <= -180.0 {
            return Ok(true);
        }

        if player.x >= 180.0 {
            level_index += 1;
            player.x = -170.0;
        }

        // if !on_block(player, &current_level.ground) && !on_block_m(player, &current_level.middle) {
        //     player.drop_down();
        // }
        //
        player.drop_down(&current_level.ground, &current_level.middle);
        count += 1;
    }
}

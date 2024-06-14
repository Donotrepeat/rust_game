use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod ui;
use crate::ui::{ui, Ground, Level, Midde, Player};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut player = Player {
        x: -120.0,
        y: 12.0,
        dy: -0.0,
    };

    let _res = run_app(&mut terminal, &mut player);
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, player: &mut Player) -> io::Result<bool> {
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

    loop {
        terminal.draw(|f| ui(f, player, &mut level.ground, &mut level.middle))?;

        let current_level = &array[level_index];

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                KeyCode::Right => {
                    if !nextto_block(player, &current_level.ground)
                        && !nextto_block_m(player, &current_level.middle)
                    {
                        player.x += 1.0;
                    }
                }
                KeyCode::Left => {
                    if !nextto_block(player, &current_level.ground)
                        && !nextto_block_m(player, &current_level.middle)
                    {
                        player.x -= 1.0;
                    }
                }
                KeyCode::Up => {
                    if !on_block(player, &current_level.ground)
                        && !on_block_m(player, &current_level.middle)
                        && jump == false
                    {
                        player.dy = 2.0;
                        jump = true;
                        jump_count = count;
                    } else {
                        player.dy = 0.0;
                    }
                }
                KeyCode::Down => {
                    if !on_block(player, &current_level.ground)
                        && !on_block_m(player, &current_level.middle)
                    {
                        player.dy = -2.0;
                    } else {
                        player.dy = 0.0;
                    }
                }
                _ => {}
            }
            if jump == true && (count - jump_count) >= 5 {
                jump = false;
                player.dy = -2.0;
            }
        }
        if player.x <= -180.0 {
            return Ok(true);
        }

        if player.x >= 180.0 {
            level_index += 1;
            player.x = -170.0;
        }

        if !on_block(player, &current_level.ground) && !on_block_m(player, &current_level.middle) {
            player.drop_down();
        }

        count += 1;
    }
}

fn on_block(player: &mut Player, ground: &[Ground]) -> bool {
    for i in ground {
        if ((player.x) >= i.x && player.x <= (i.x + 15.0))
            && (player.y >= -85.0 && player.y <= (i.hight as f64 * 10.0 - 85.0))
        {
            player.y = i.hight as f64 * 10.0 - 84.0;
            player.dy = 0.0;
            return true;
        }
    }
    return false;
}

fn nextto_block(player: &mut Player, ground: &[Ground]) -> bool {
    for i in ground {
        if ((player.x) >= (i.x - 5.0) && player.x <= (i.x + 15.0))
            && (player.y >= -85.0 && player.y <= (i.hight as f64 * 10.0 - 85.0))
        {
            if (player.x - (i.x - 5.0)) < ((i.x + 15.0) - player.x) {
                player.x = i.x - 6.0;
            } else if (player.x - (i.x - 5.0)) >= ((i.x + 15.0) - player.x) {
                player.x = i.x + 16.0;
            }
            return true;
        }
    }
    return false;
}
fn on_block_m(player: &mut Player, ground: &[Midde]) -> bool {
    for i in ground {
        if ((player.x) >= i.x && player.x <= (i.x + 15.0))
            && (player.y >= (i.level as f64 * 10.0 - 95.0)
                && player.y <= (i.level as f64 * 10.0 - 75.0))
        {
            if player.y > (i.level as f64 * 10.0 - 90.0) {
                player.y += 1.0;
            } else if player.y <= (i.level as f64 * 10.0 - 90.0) {
                player.y -= 1.0;
            }

            player.dy = 0.0;
            return true;
        }
    }
    return false;
}

fn nextto_block_m(player: &mut Player, ground: &[Midde]) -> bool {
    for i in ground {
        if ((player.x) >= (i.x - 5.0) && player.x <= (i.x + 15.0))
            && (player.y >= (i.level as f64 * 10.0 - 85.0)
                && player.y <= (i.level as f64 * 10.0 - 75.0))
        {
            if (player.x - (i.x - 5.0)) < ((i.x + 15.0) - player.x) {
                player.x = i.x - 6.0;
            } else if (player.x - (i.x - 5.0)) >= ((i.x + 15.0) - player.x) {
                player.x = i.x + 16.0;
            }
            return true;
        }
    }
    return false;
}

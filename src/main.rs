use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod ui;
use crate::ui::{ui, Player};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut player = Player { x: 12.0, y: 12.0 };

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
    loop {
        terminal.draw(|f| ui(f, player))?;

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
                    player.x += 1.0;
                }
                KeyCode::Left => {
                    player.x -= 1.0;
                }
                KeyCode::Up => {
                    player.y += 1.0;
                }
                KeyCode::Down => {
                    player.y -= 1.0;
                }
                _ => {}
            }
        }
    }
}

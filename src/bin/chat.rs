mod game_of_life;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game_of_life::GameOfLife;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = GameOfLife::new(800, 400);
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let mut grid_display = String::new();
            for row in game.get_grid() {
                for &cell in row {
                    grid_display.push(if cell { '█' } else { ' ' });
                }
                grid_display.push('\n');
            }

            let paragraph = Paragraph::new(grid_display)
                .block(Block::default().borders(Borders::ALL).title("Game of Life"))
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, chunks[0]);
        })?;

        if crossterm::event::poll(tick_rate - last_tick.elapsed())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => game.next_generation(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            game.next_generation();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap, canvas::*},
    Frame,
};

pub fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constrains([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(5),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Game test", Style::default().fg(Color::Gray)))
        .block(title_block);

    f.render_widget(title, chunks[0]);


    let ground = Canvas::default()
        .background_color(White)
        .block(
            Block::new(),
        )

    f.render_widget(ground, chunks[2]);
}

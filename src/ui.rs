use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{canvas::*, Block, Borders, Paragraph},
    Frame,
};
#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub dy: f64,
}

impl Player {
    pub fn drop_down(&mut self) {
        self.y += self.dy;
    }
}

#[derive(Copy, Clone)]
pub struct Ground {
    pub x: f64,
    pub hight: u8,
}

#[derive(Copy, Clone)]
pub struct Midde {
    pub x: f64,
    pub level: u8,
}

#[derive(Clone)]
pub struct Level {
    pub ground: Vec<Ground>,
    pub middle: Vec<Midde>,
}

pub fn ui(f: &mut Frame, player: &Player, ground: &mut [Ground], middel: &mut [Midde]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(5),
        ])
        .split(f.size());

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        player.x.to_string().yellow(),
        player.y.to_string().yellow(),
    ])]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(counter_text).centered().block(title_block);

    f.render_widget(title, chunks[0]);

    let screen = Canvas::default()
        .block(Block::bordered().title("World"))
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: player.x,
                y: player.y,
                radius: 5.0,
                color: Color::Red,
            });
            ctx.draw(&Rectangle {
                x: -180.0,
                y: -90.0,
                width: 10.0,
                height: 10.0,
                color: Color::Cyan,
            });

            for i in &*ground {
                ctx.draw(&Rectangle {
                    x: i.x,
                    y: -90.0,
                    width: 10.0,
                    height: 10.0 * i.hight as f64,
                    color: Color::Cyan,
                })
            }

            for i in &*middel {
                ctx.draw(&Rectangle {
                    x: i.x,
                    y: -90.0 + i.level as f64 * 10.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Cyan,
                })
            }
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(screen, chunks[1]);

    let ground = Canvas::default()
        .block(Block::bordered().title("World"))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::Green,
                resolution: MapResolution::High,
            });
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);

    f.render_widget(ground, chunks[2]);
}

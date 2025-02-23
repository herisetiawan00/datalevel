use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{data::context::global_context::GlobalContext, presentation::Screen};

pub fn screen() -> Screen {
    Screen { build, key }
}

fn build(context: &mut GlobalContext, frame: &mut Frame) {
    let connections = [
        "MongoDB - Jago App - LFS - Dev",
        "MongoDB - Jago App - LP - Dev",
        "MongoDB - Jago App - Loan Account - Dev",
        "Create Connection...",
    ];

    let area = frame.area();
    let block = Block::default().borders(Borders::ALL);
    frame.render_widget(block, area);

    let constraints: Vec<Constraint> = std::iter::once(Constraint::Min(0))
        .chain(connections.iter().map(|_| Constraint::Length(1)))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let _canvas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .horizontal_margin(1)
        .split(area);

    for (i, &connection) in connections.iter().enumerate() {
        //let index = i.clone() as i32;
        let paragraph = Paragraph::new(connection)
            .alignment(Alignment::Center)
            .style(
                Style::default(), //.fg(if index == selected {
                                  //    Color::Black
                                  //} else {
                                  //    Color::White
                                  //})
                                  //.bg(if index == selected {
                                  //    Color::White
                                  //} else {
                                  //    Color::Reset
                                  //}),
            );

        frame.render_widget(paragraph, _canvas[i + 1]); // Skipping top padding
    }
}

fn key(context: &mut GlobalContext, event: &KeyEvent) {
    match event.code {
        KeyCode::Char('q') => {
            context.route_clear();
        }
        _ => {}
    }
}

use std::{io, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{data::context::global_context::GlobalContext, presentation::routes::Routes};

pub fn app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut context = GlobalContext::new();
    context.route_push(Routes::Connections);

    loop {
        if let Some(route) = context.route() {
            let screen = route.get();
            terminal.draw(|frame| {
                screen.build(&mut context, frame);
            })?;

            if event::poll(Duration::from_secs(1))? {
                if let Event::Key(key) = event::read()? {
                    screen.key(&mut context, &key);
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}

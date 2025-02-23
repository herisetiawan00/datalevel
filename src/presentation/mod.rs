mod journey;
pub mod routes;

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::data::context::global_context::GlobalContext;

pub struct Screen {
    build: fn(&mut GlobalContext, &mut Frame),
    key: fn(&mut GlobalContext, &KeyEvent),
}

impl Screen {
    pub fn build(&self, context: &mut GlobalContext, frame: &mut Frame) {
        (self.build)(context, frame);
    }

    pub fn key(&self, context: &mut GlobalContext, key: &KeyEvent) {
        (self.key)(context, key);
    }
}

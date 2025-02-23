use super::{journey::connections, Screen};

pub enum Routes {
    Connections,
}

impl Routes {
    pub fn get(&self) -> Screen {
        match self {
            Self::Connections => connections::screen(),
        }
    }
}

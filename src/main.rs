mod app;
mod data;
mod presentation;
mod wrapper;

use app::app;
use std::io;
use wrapper::app_wrapper;

fn main() -> io::Result<()> {
    app_wrapper(app)
}

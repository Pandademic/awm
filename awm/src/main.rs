#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_winow_manager,
    Backward, Forward, Less, More, Selector
};

use simplelog::{LevelFilter, SimpleLogger};

fn main() -> penrose::Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };
    loop{}
}


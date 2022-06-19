#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        manager::WindowManager,
        layout::{
            LayoutConf,
            side_stack,
        },
    },
    draw::{
        Color,
    },
    logging_error_handler,
    //xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector
};

use penrose::new_xcb_backed_window_manager;
use simplelog::{LevelFilter, SimpleLogger};

pub const TERMINAL: &str = "st";
pub const LAUNCHER: &str = "dmenu_run";

fn main() -> penrose::Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };
    let config = Config::default()
        .builder()
        .show_bar(false)
        .top_bar(false)
        .layouts(vec![side_stack])
        .focused_border(Color::PURPLE)?
        .gap_px(5)
        .build()
        .expect("could not build config!!!");
    

    let key_bindings = gen_keybindings! {
        // Program launchers
        "C-space" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit Penrose (important to remember this one!)
        "C-M" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "C-M-c" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "C-A-Up" => run_internal!(cycle_workspace, Forward);
        "C-A-Down" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-h" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
             "M-{}" => focus_workspace (REF);
             "M-S-{}" => client_to_workspace (REF);
         };
    };

    let mut wm = penrose::new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})


}


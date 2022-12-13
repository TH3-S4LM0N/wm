use penrose::core::WindowManager;

use {
    penrose::{
        builtin::actions::{exit, modify_with, spawn},
        core::{
            bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
            Config,
        },
        map,
        x11rb::RustConn,
        Result,
    },
    std::collections::HashMap,
    tracing_subscriber::{self, prelude::*},
};

mod logger;
mod theme;

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        // focus and swap keys
        "A-l" => modify_with(|cs| cs.focus_up()),
        "A-h" => modify_with(|cs| cs.focus_down()),
        "A-k" => modify_with(|cs| cs.swap_up()),
        "A-j" => modify_with(|cs| cs.swap_down()),

        "A-S-a" => spawn("alacritty"),
        "A-p" => spawn("rofi -show drun -theme ~/.config/rofi/theme.rasi"),

        "A-S-q" => exit(),
    };

    //for tag in &["", "", "﬏", "", ""] {
    for tag in &["1", "2", "3", "4", "5"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }
    return raw_bindings;
}

fn main() -> Result<()> {
    // xdg compliant directories for items
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("penrose_wms").expect("Failed to create xdg dirs");
    
    // start the simple logger
    logger::init_logger(&xdg_dirs.get_cache_file("penrose_wm.log"));

    // print penrose errs to stdout
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(Config::default(), bindings, HashMap::new(), conn)?;

    wm.run()
}

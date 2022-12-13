use {
    std::{
        path::Path,
        process::Command
    },
    crate::logger::log_err,
    xdg::BaseDirectories
};

pub fn load_theme<P>(xdg_dirs: BaseDirectories) // config_path: "~/.config/penrose_wm"
{
    let cmd = Command::new("sh")
            .arg("-c")
            .arg(&format!("{:?}/up", &xdg_dirs.get_config_home()))
            .spawn();

    match cmd {
        Ok(_) => (),
        Err(e) => log_err("Failed to run up command", &xdg_dirs.get_cache_file("penrose_wm.log"), &e),
    }
}

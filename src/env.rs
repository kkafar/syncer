use core::panic;

use std::path::Path;
use log::trace;



pub const APP_PREFIX: &str = "syncer";


pub fn ensure_data_files_exist(dirs: &xdg::BaseDirectories) {
    match dirs.create_data_directory(Path::new(APP_PREFIX)) {
        Err(err) => {
            panic!("Error while creating data directory {err:?}");
        }
        Ok(ref path) => {
            trace!("Created data directory: {path:?}");
        }
    };
}


pub fn ensure_state_files_exist(dirs: &xdg::BaseDirectories) {
    match dirs.create_state_directory(Path::new(APP_PREFIX)) {
        Err(err) => {
            panic!("Error while creating state directory {err:?}");
        }
        Ok(ref path) => {
            trace!("Created state directory: {path:?}");
        }
    };
}


pub fn ensure_file_structure_exists() -> xdg::BaseDirectories {
    trace!("Creating file structure");
    let Ok(xdg_dirs) = xdg::BaseDirectories::new() else {
        panic!("Could not create xdg::BaseDirectories, do you have HOME environment variable set?");
    };

    ensure_data_files_exist(&xdg_dirs);
    ensure_state_files_exist(&xdg_dirs);

    xdg_dirs
}


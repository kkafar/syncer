use core::panic;
use log::trace;
use std::path::{Path, PathBuf};

pub const APP_PREFIX: &str = "syncer";

pub struct AppDirectories {
    data_dir: PathBuf,
    state_dir: PathBuf,
}

impl AppDirectories {
    pub fn new() -> Self {
        let xdg_dirs = AppDirectories::ensure_file_structure_exists();
        let data_dir = xdg_dirs.get_data_home().join(APP_PREFIX);
        let state_dir = xdg_dirs.get_state_home().join(APP_PREFIX);
        Self {
            data_dir,
            state_dir,
        }
    }

    pub fn get_data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn get_state_dir(&self) -> &Path {
        &self.state_dir
    }

    fn ensure_data_files_exist(dirs: &xdg::BaseDirectories) {
        match dirs.create_data_directory(Path::new(APP_PREFIX)) {
            Err(err) => {
                panic!("Error while creating data directory {err:?}");
            }
            Ok(ref path) => {
                trace!("Created data directory: {path:?}");
            }
        };
    }

    fn ensure_state_files_exist(dirs: &xdg::BaseDirectories) {
        match dirs.create_state_directory(Path::new(APP_PREFIX)) {
            Err(err) => {
                panic!("Error while creating state directory {err:?}");
            }
            Ok(ref path) => {
                trace!("Created state directory: {path:?}");
            }
        };
    }

    fn ensure_file_structure_exists() -> xdg::BaseDirectories {
        trace!("Creating file structure");
        let Ok(xdg_dirs) = xdg::BaseDirectories::new() else {
            panic!("Could not create xdg::BaseDirectories, do you have HOME environment variable set?");
        };

        AppDirectories::ensure_data_files_exist(&xdg_dirs);
        AppDirectories::ensure_state_files_exist(&xdg_dirs);

        xdg_dirs
    }
}


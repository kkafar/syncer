pub mod path {
    use std::{
        io,
        path::{Path, PathBuf},
    };

    use path_clean::PathClean;

    pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
        let path = path.as_ref();
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        }
        .clean();

        Ok(abs_path)
    }
}

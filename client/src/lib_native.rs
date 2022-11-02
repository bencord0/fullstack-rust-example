use std::path::Path;
use include_dir::{include_dir, Dir, File};

static STATIC_DIR: Dir<'_> = include_dir!("$OUT_DIR/pkg");

pub fn get_file<S: AsRef<Path>>(path: S) -> Option<&'static File<'static>> {
    STATIC_DIR.get_file(path)
}

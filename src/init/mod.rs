use std::fs::File;

pub fn init_simple() {
    create_simple_directory();
}

fn create_simple_directory() {
    let path = std::path::Path::new("./.simple/simple.toml");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let _ = File::create(path);
}

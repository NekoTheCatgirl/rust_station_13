use rusty_engine::fileutils::get_home_path;

pub fn client_dirs_exist() -> bool {
    let home = get_home_path();
    home.join("./client_config").exists()
        && home.join("client_saves").exists()
}

pub fn server_dirs_exist() -> bool {
    let home = get_home_path();
    home.join("./server_config").exists()
        && home.join("client_saves").exists()
}

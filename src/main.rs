mod config;
mod ui;

fn main() {
    let menu = config::Menu::read_config();
    let _ = ui::main(&menu.unwrap());
}

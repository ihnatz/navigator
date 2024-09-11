mod config;
mod ui;

fn main() {
    let menu = config::Menu::read_config();
    ui::main(&menu.unwrap());
}

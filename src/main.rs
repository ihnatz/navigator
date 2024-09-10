mod config;
mod ui;

fn main() {
    let _configuration = config::Menu::read_config();
    let _ = ui::main();
}

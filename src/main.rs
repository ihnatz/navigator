mod config;
mod ui;

fn main() {
    let menu = config::Menu::read_config();
    if let Some(command) = ui::main(&menu.unwrap()) {
        println!("{command}");
    }
}

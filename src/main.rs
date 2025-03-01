use std::fs::File;
use std::io::Write;
use std::process;

mod config;
mod ui;

fn main() -> std::io::Result<()> {
    let menu = config::Menu::read_config();

    if let Err(err) = menu {
        eprintln!("[ERROR] Can't read configuration file:\n{err}");
        process::exit(1);
    }

    let output_path = "/tmp/navigator_output.txt";

    let mut file = File::create(output_path)?;

    if let Some(cmd) = ui::main(&menu.unwrap()) {
        file.write_all(cmd.as_bytes())?;
    } else {
        file.write_all(b"")?;
    }

    Ok(())
}

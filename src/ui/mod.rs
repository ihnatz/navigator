pub mod state;

use crate::config::Menu;
use crate::ui::state::State;

use std::io::{stdin, stdout, Result, Stdout, Write};
use termion::{clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode, style};

#[derive(Debug)]
enum Command {
    Quit,
    MoveUp,
    MoveDown,
    GoInside,
    GoOutside,
}

const ITEMS_PER_LIST: usize = 5;

pub fn main(menu: &Menu) -> Option<String> {
    let mut state = State {
        current_cursor: 0,
        current_item_id: 0,
        menu,
    };

    let response = with_terminal(|stdout| loop {
        let items = state
            .next_level()
            .map(|item| &*item.title)
            .collect::<Vec<&str>>();
        render_list(&items, state.current_cursor, stdout);
        match handle_events() {
            Ok(Some(Command::Quit)) => break None,
            Ok(Some(Command::MoveUp)) => state.move_up(),
            Ok(Some(Command::MoveDown)) => state.move_down(),
            Ok(Some(Command::GoOutside)) => state.go_outside(),
            Ok(Some(Command::GoInside)) => {
                if state.is_terminating() {
                    break Some(state.pressed_item().value.clone().unwrap());
                } else {
                    state.go_inside();
                }
            },
            _ => (),
        }
    })
    .unwrap();

    response
}

fn handle_events() -> Result<Option<Command>> {
    let stdin = stdin();
    for key in stdin.keys() {
        match key? {
            Key::Char('q') => return Ok(Some(Command::Quit)),
            Key::Up => return Ok(Some(Command::MoveUp)),
            Key::Down => return Ok(Some(Command::MoveDown)),
            Key::Char('\n') => return Ok(Some(Command::GoInside)),
            Key::Esc => return Ok(Some(Command::GoOutside)),
            _ => (),
        }
    }
    Ok(None)
}

fn with_terminal<F, T>(f: F) -> Result<T>
where
    F: FnOnce(&mut Stdout) -> T,
{
    let mut raw_stdout = stdout().into_raw_mode()?;

    writeln!(raw_stdout)?;
    write!(raw_stdout, "{}", cursor::Hide)?;
    write!(raw_stdout, "{}", cursor::Save)?;
    write!(raw_stdout, "{}", clear::AfterCursor)?;
    raw_stdout.flush()?;

    let result = f(&mut raw_stdout);

    write!(raw_stdout, "{}", cursor::Restore)?;
    write!(raw_stdout, "{}", cursor::Show)?;
    raw_stdout.flush()?;

    Ok(result)
}

fn render_list(items: &[&str], selected_index: usize, stdout: &mut impl Write) {
    write!(stdout, "{}", clear::AfterCursor).unwrap();
    let skip_count = calculate_window_start(selected_index, items.len(), ITEMS_PER_LIST);

    for (i, item) in items
        .iter()
        .enumerate()
        .skip(skip_count)
        .take(ITEMS_PER_LIST)
    {
        if i == selected_index {
            write!(
                stdout,
                " {}{}>{}{} {}\r\n",
                style::Bold,
                color::Fg(color::Blue),
                color::Fg(color::Reset),
                style::Reset,
                item
            )
            .unwrap();
        } else {
            write!(stdout, "   {}\r\n", item).unwrap();
        }
    }
    if items.len() < ITEMS_PER_LIST {
        for _i in items.len()..ITEMS_PER_LIST {
            writeln!(stdout).unwrap();
        }
    }

    write!(stdout, "{}", cursor::Up(ITEMS_PER_LIST as u16)).unwrap();
    stdout.flush().unwrap();
}

fn calculate_window_start(selected_index: usize, total_items: usize, window_size: usize) -> usize {
    if selected_index < window_size {
        0
    } else if selected_index > total_items - window_size {
        total_items - window_size
    } else {
        selected_index - window_size / 2
    }
}

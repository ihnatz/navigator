pub mod state;

use crate::config::Menu;
use crate::ui::state::State;

use std::io::{stdin, stdout, BufWriter, Result, Stdout, Write};
use termion::{clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode, raw::RawTerminal, style};

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

    let response = with_terminal(|buf_writer| loop {
        let items = state
            .next_level()
            .map(|item| &*item.title)
            .collect::<Vec<&str>>();
        render_list(&items, state.current_cursor, buf_writer);
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
    F: FnOnce(&mut BufWriter<RawTerminal<Stdout>>) -> T,
{
    let raw_stdout = stdout().into_raw_mode()?;
    let mut buf_writer = BufWriter::new(raw_stdout);

    writeln!(buf_writer)?;
    write!(buf_writer, "{}", cursor::Hide)?;
    write!(buf_writer, "{}", cursor::Save)?;
    write!(buf_writer, "{}", clear::AfterCursor)?;
    buf_writer.flush()?;

    let result = f(&mut buf_writer);

    write!(buf_writer, "{}", cursor::Restore)?;
    write!(buf_writer, "{}", cursor::Show)?;
    buf_writer.flush()?;

    Ok(result)
}

fn render_list(items: &[&str], selected_index: usize, buf_writer: &mut BufWriter<impl Write>) {
    write!(buf_writer, "{}", clear::AfterCursor).unwrap();
    let skip_count = calculate_window_start(selected_index, items.len(), ITEMS_PER_LIST);

    for (i, item) in items
        .iter()
        .enumerate()
        .skip(skip_count)
        .take(ITEMS_PER_LIST)
    {
        if i == selected_index {
            write!(
                buf_writer,
                " {}{}>{}{} {}\r\n",
                style::Bold,
                color::Fg(color::Blue),
                color::Fg(color::Reset),
                style::Reset,
                item
            )
            .unwrap();
        } else {
            write!(buf_writer, "   {}\r\n", item).unwrap();
        }
    }
    if items.len() < ITEMS_PER_LIST {
        for _i in items.len()..ITEMS_PER_LIST {
            writeln!(buf_writer).unwrap();
        }
    }

    write!(buf_writer, "{}", cursor::Up(ITEMS_PER_LIST as u16)).unwrap();
    buf_writer.flush().unwrap();
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

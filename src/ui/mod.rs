pub mod state;

use crate::config::Menu;
use crate::ui::state::State;

use std::io::{stdout, Result, Stdout, Write};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
    Frame, Terminal, TerminalOptions, Viewport,
};

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

    let response = with_terminal(|terminal| loop {
        terminal.draw(|f| ui(f, &state)).unwrap();
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
            }
            _ => (),
        }
    })
    .unwrap();

    response
}

fn handle_events() -> Result<Option<Command>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Command::Quit)),
                KeyCode::Up => return Ok(Some(Command::MoveUp)),
                KeyCode::Down => return Ok(Some(Command::MoveDown)),
                KeyCode::Enter => return Ok(Some(Command::GoInside)),
                KeyCode::Esc => return Ok(Some(Command::GoOutside)),
                _ => (),
            }
        }
    }
    Ok(None)
}

fn ui(frame: &mut Frame, state: &State) {
    let areas = Layout::vertical([Constraint::Length(1); ITEMS_PER_LIST]).split(frame.area());
    let skip_count = state
        .current_cursor
        .saturating_sub(ITEMS_PER_LIST.saturating_sub(1));

    for (id, subitem) in state
        .next_level()
        .enumerate()
        .skip(skip_count)
        .take(ITEMS_PER_LIST)
    {
        let mut line = Paragraph::new(&*subitem.title);
        if id == state.current_cursor {
            line = line.black().on_white();
        }
        frame.render_widget(line, areas[id - skip_count]);
    }
}

fn with_terminal<F, T>(f: F) -> Result<T>
where
    F: FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> T,
{
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(ITEMS_PER_LIST as u16),
    });

    enable_raw_mode()?;
    let result = f(&mut terminal);
    disable_raw_mode()?;

    terminal.clear().unwrap();
    terminal.draw(|_f| {}).unwrap();

    stdout().flush()?;

    Ok(result)
}

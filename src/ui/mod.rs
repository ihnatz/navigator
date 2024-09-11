pub mod state;

use crate::config::Menu;
use crate::ui::state::State;
use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
    Frame, Terminal,
};

enum Command {
    Quit,
    MoveUp,
    MoveDown,
}

pub fn main(menu: &Menu) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut state = State {
        current_cursor: 0,
        current_item_id: 0,
        menu: menu,
    };

    loop {
        terminal.draw(|f| ui(f, &state))?;
        match handle_events() {
            Ok(Some(Command::Quit)) => break,
            Ok(Some(Command::MoveUp)) => state.move_up(),
            Ok(Some(Command::MoveDown)) => state.move_down(),
            _ => (),
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> Result<Option<Command>, std::io::Error> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Command::Quit)),
                KeyCode::Up => return Ok(Some(Command::MoveUp)),
                KeyCode::Down => return Ok(Some(Command::MoveDown)),
                _ => (),
            }
        }
    }
    return Ok(None);
}

fn ui(frame: &mut Frame, state: &State) {
    let area = frame.area();
    let areas = Layout::vertical([Constraint::Length(1); 10]).split(area);

    for (id, subitem) in state.next_level().enumerate() {
        let mut line = Paragraph::new(&*subitem.title);
        if id == state.current_cursor {
            line = line.black().on_white();
        }
        frame.render_widget(line, areas[id]);
    }
}

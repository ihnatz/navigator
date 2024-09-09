use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

enum Command {
    Quit,
    MoveUp,
    MoveDown,
}

struct State {
    current: usize,
}

pub fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut state = State { current: 0 };

    loop {
        terminal.draw(ui)?;
        match handle_events() {
            Ok(Some(Command::Quit)) => break,
            Ok(Some(Command::MoveUp)) => state.current = state.current.saturating_sub(1),
            Ok(Some(Command::MoveDown)) => state.current = state.current.saturating_add(1),
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

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}

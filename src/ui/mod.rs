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

enum Commands {
    QUIT,
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
        match handle_events(&mut state) {
            Some(Commands::QUIT) => break,
            _ => (),
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(state: &mut State) -> Option<Commands> {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind != event::KeyEventKind::Press {
                return None;
            }
            match key.code {
                KeyCode::Char('q') => return Some(Commands::QUIT),
                KeyCode::Up => state.current = state.current.saturating_sub(1),
                KeyCode::Down => state.current = state.current.saturating_add(1),
                _ => (),
            }
        }
    }
    return None;
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}

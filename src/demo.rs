use crate::ui::{ui, AppState};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

pub fn run_demo() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::default();
    
    // Add some demo messages to show the fixed bubbles
    state.add_message("Hello Gemini! How are you today?".to_string(), true);
    state.add_message("Hello! I'm doing great, thank you for asking! I'm here to help you with any questions or tasks you might have. The weather has been lovely lately, and I've been enjoying our conversations. How has your day been going so far?".to_string(), false);
    state.add_message("That's wonderful to hear! I've been working on a TUI chat application in Rust.".to_string(), true);
    state.add_message("That sounds like an exciting project! Rust is an excellent choice for building TUI applications. The combination of performance, memory safety, and the rich ecosystem of crates like ratatui makes it perfect for creating responsive and beautiful terminal interfaces. Are you finding the development process enjoyable?".to_string(), false);
    state.add_message("Yes, very much! The bubble design looks much better now.".to_string(), true);

    state.status_message = "Demo Mode - Press any key to continue, Ctrl+C to exit".to_string();

    // Animation timer
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    // Main event loop
    loop {
        terminal.draw(|f| ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Handle events
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            break;
                        }
                        _ => {
                            break; // Exit on any other key press
                        }
                    }
                }
            }
        }

        // Handle animation ticks
        if last_tick.elapsed() >= tick_rate {
            state.increment_animation();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
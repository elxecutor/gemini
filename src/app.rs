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
use tokio::sync::mpsc;

use crate::{
    gemini::GeminiClient,
    ui::{ui, AppState},
};

pub struct App {
    state: AppState,
    client: GeminiClient,
}

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    GeminiResponse(String),
    GeminiError(String),
}

impl App {
    pub fn new(api_key: String) -> Self {
        Self {
            state: AppState::default(),
            client: GeminiClient::new(api_key),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create channels for async communication
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Animation timer
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(100);

        // Main event loop
        loop {
            terminal.draw(|f| ui(f, &self.state))?;

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
                            KeyCode::Enter => {
                                if !self.state.input.trim().is_empty() && !self.state.is_loading {
                                    let message = self.state.input.clone();
                                    self.state.add_message(message.clone(), true);
                                    self.state.clear_input();
                                    self.state.is_loading = true;
                                    self.state.status_message = "Sending message to Gemini...".to_string();

                                    // Send message to Gemini in background
                                    let client = self.client.clone();
                                    let tx_clone = tx.clone();
                                    tokio::spawn(async move {
                                        match client.send_message(&message).await {
                                            Ok(response) => {
                                                let _ = tx_clone.send(AppEvent::GeminiResponse(response));
                                            }
                                            Err(e) => {
                                                let _ = tx_clone.send(AppEvent::GeminiError(e.to_string()));
                                            }
                                        }
                                    });
                                }
                            }
                            KeyCode::Char(c) => {
                                self.state.insert_char(c);
                            }
                            KeyCode::Backspace => {
                                self.state.delete_char();
                            }
                            KeyCode::Left => {
                                self.state.move_cursor_left();
                            }
                            KeyCode::Right => {
                                self.state.move_cursor_right();
                            }
                            KeyCode::Esc => {
                                if self.state.is_loading {
                                    self.state.is_loading = false;
                                    self.state.status_message = "Message cancelled".to_string();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Handle async messages
            while let Ok(event) = rx.try_recv() {
                match event {
                    AppEvent::GeminiResponse(response) => {
                        self.state.add_message(response, false);
                        self.state.is_loading = false;
                        self.state.status_message = "Response received! 🎉".to_string();
                    }
                    AppEvent::GeminiError(error) => {
                        self.state.add_message(
                            format!("❌ Error: {}", error),
                            false,
                        );
                        self.state.is_loading = false;
                        self.state.status_message = "Error occurred 😞".to_string();
                    }
                    AppEvent::Tick => {
                        self.state.increment_animation();
                    }
                }
            }

            // Handle animation ticks
            if last_tick.elapsed() >= tick_rate {
                self.state.increment_animation();
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
}

impl Clone for GeminiClient {
    fn clone(&self) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
        }
    }
}
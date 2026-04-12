//! Terminal UI for polytunnel
//!
//! Provides an interactive ratatui-based dashboard for managing
//! Java project dependencies, viewing build configuration, and
//! resolving dependency trees.

#![warn(missing_docs)]

mod app;
mod event;
mod ui;

use std::path::PathBuf;
use std::time::Duration;

use crossterm::ExecutableCommand;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{App, InputMode};

/// Run the interactive TUI.
///
/// Initializes the terminal, enters the event loop, and restores
/// the terminal on exit.
pub async fn run_tui(config_path: PathBuf) -> color_eyre::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(config_path)?;
    let tick_rate = Duration::from_millis(250);

    // Event loop
    while app.running {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        match event::poll(tick_rate)? {
            event::Event::Key(key) => {
                // Ignore key release events on Windows
                if key.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }
                handle_key(&mut app, key.code, key.modifiers).await;
            }
            event::Event::Tick | event::Event::Resize => {}
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    // Ctrl+C always quits
    if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
        app.running = false;
        return;
    }

    match &app.input_mode {
        InputMode::AddingCoord => match code {
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Enter => app.confirm_coord(),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                app.input_buffer.push(c);
            }
            _ => {}
        },
        InputMode::AddingScope => match code {
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Enter => app.confirm_add(),
            KeyCode::Up | KeyCode::Char('k') => app.scope_up(),
            KeyCode::Down | KeyCode::Char('j') => app.scope_down(),
            _ => {}
        },
        InputMode::ConfirmDelete => match code {
            KeyCode::Char('y') => app.confirm_delete(),
            KeyCode::Char('n') | KeyCode::Esc => app.cancel_input(),
            _ => {}
        },
        InputMode::Normal => match code {
            KeyCode::Char('q') | KeyCode::Esc => app.running = false,
            KeyCode::Char('1') => app.switch_tab(app::Tab::Dashboard),
            KeyCode::Char('2') => app.switch_tab(app::Tab::Dependencies),
            KeyCode::Char('3') => app.switch_tab(app::Tab::Tree),
            KeyCode::Tab => app.next_tab(),
            // Dependencies tab keys
            KeyCode::Up | KeyCode::Char('k') if app.tab == app::Tab::Dependencies => {
                app.select_up();
            }
            KeyCode::Down | KeyCode::Char('j') if app.tab == app::Tab::Dependencies => {
                app.select_down();
            }
            KeyCode::Char('a') if app.tab == app::Tab::Dependencies => {
                app.enter_add_mode();
            }
            KeyCode::Char('d') if app.tab == app::Tab::Dependencies => {
                app.enter_delete_mode();
            }
            // Tree tab keys
            KeyCode::Char('r') if app.tab == app::Tab::Tree => {
                app.resolve_tree().await;
            }
            _ => {}
        },
    }
}

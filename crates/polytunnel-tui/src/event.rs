use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

/// TUI event types
pub enum Event {
    /// Keyboard input
    Key(KeyEvent),
    /// Terminal resize
    Resize,
    /// Periodic tick for UI refresh
    Tick,
}

/// Polls for crossterm events with a tick interval.
pub fn poll(tick_rate: Duration) -> color_eyre::Result<Event> {
    if event::poll(tick_rate)? {
        match event::read()? {
            CrosstermEvent::Key(key) => Ok(Event::Key(key)),
            CrosstermEvent::Resize(_, _) => Ok(Event::Resize),
            _ => Ok(Event::Tick),
        }
    } else {
        Ok(Event::Tick)
    }
}

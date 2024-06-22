use crate::app::{App, AppResult};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Handles the key events and updates the state of [`App`].

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if let Event::Key(key_event) = event::read()? {
        if key_event.kind == KeyEventKind::Press {
            use KeyCode::*;
            match key_event.code {
                Char('q') | Esc => app.quit(),
                Char('h') | Left => app.items.unselect(),
                Char('j') | Down => app.items.next(),
                Char('k') | Up => app.items.previous(),
                // Char('l') | Right | Enter => self.change_status(),
                Char('g') => app.go_top(),
                Char('G') => app.go_bottom(),
                _ => {}
            }
        }
    }

    Ok(())
}

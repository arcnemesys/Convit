use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui;
use color_eyre::config::HookBuilder;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::panic;

/// Representation of a terminal user interface
/// 
/// It is responsible for setting up the terminal,
/// initializing the interface, and handling the draw events.

#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the terminal
    terminal: Terminal<B>,
    /// Terminal event handler
    pub events: EventHandler
}

impl<B: Backend> Tui<B> {
    /// Construct new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events}
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        let (panic, error) = HookBuilder::default().into_hooks();
        let panic = panic.into_panic_hook();
        let error = error.into_eyre_hook();

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    fn restore_terminal(&self) -> color_eyre::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
       }
}
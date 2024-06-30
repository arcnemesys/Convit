use crate::app::{App, AppResult};
use crate::ui;
// use crate::event::EventHandler;
use ratatui::backend::Backend;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use std::io;
use std::panic;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    // pub events: EventHandler
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        // Add missing event handler
        Self { terminal }
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), 
        EnterAlternateScreen, 
        EnableMouseCapture)?;
        // Define a custom panic hook to reset the terminal properties.
      // This way, you won't have your terminal messed up if an unexpected error happens.
        
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
      }

      pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| frame.render_widget(app, frame.size()))?;
        Ok(())
    }

    fn reset(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
          io::stderr(),
          LeaveAlternateScreen,
          DisableMouseCapture
      )?;
      Ok(())
      }

      pub fn exit(&mut self) -> AppResult<()> {
        self.reset()?;
        self.terminal.show_cursor()?;
        Ok(())
      }
      }

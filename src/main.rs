use crossterm::{event::DisableMouseCapture, execute, terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen}};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {


        // Footer type list
        let footer_types = vec!["BREAKING CHANGE", "Co-authored-by"];
        let footer_items: Vec<ListItem> = footer_types.iter().map(|i| ListItem::new(*i)).collect();
        let footer_list = List::new(footer_items)
            .block(Block::default().borders(Borders::ALL).title("Footer Type"));

        // f.render_widget(commit_list, horizontal_chunks[0]);
        // f.render_widget(footer_list, horizontal_chunks[1]);

        // Description text area
        let description = Paragraph::new(Text::from("Description"))
            .block(Block::default().borders(Borders::ALL).title("Description"));
        // f.render_widget(description, chunks[1]);

        // Body text area
        let body = Paragraph::new(Text::from("Body"))
            .block(Block::default().borders(Borders::ALL).title("Body"));
        // f.render_widget(body, chunks[2]);

        // Full commit display area
        let full_commit = Paragraph::new(Text::from("Full Commit:"))
            .block(Block::default().borders(Borders::ALL).title("Composed Commit"));
        // f.render_widget(full_commit, chunks[3]);
    })?;

    Ok(())
}

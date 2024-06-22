use ratatui::{
 layout::{Alignment, Constraint, Direction, Layout, Rect},
 style::{Color, Modifier, Style, Stylize},
 text::{Line, Span, Text},
 widgets::{Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph, Wrap},
 Frame,
};

use crate::app::{App, CommitType, NORMAL_ROW_COLOR, SELECTED_STYLE_FG, TEXT_COLOR, TODO_HEADER_BG};

pub fn render(app:&mut App, frame: &mut Frame) {
     let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
                   Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(2),
      ]).split(frame.size());
    


      let title_block = Block::default()
      .borders(Borders::ALL)
      .style(Style::default());
    
      let title = Paragraph::new(Text::styled(
        "Convit",
        Style::default().fg(Color::Green),
    ))
    .block(title_block.clone());
     frame.render_widget(title, chunks[0]);

     let items: Vec<ListItem> = app
         .items
         .commit_items
         .iter()
         .enumerate()
         .map(|(i, commit_type)| commit_type.to_list_item(i))
         .collect();

     // Create a List from all list items and highlight the currently selected one
     let items = List::new(items)
         .block(title_block)
         .highlight_style(
             Style::default()
                 .add_modifier(Modifier::BOLD)
                 .add_modifier(Modifier::REVERSED)
                 .fg(SELECTED_STYLE_FG),
         )
         .highlight_symbol(">")
         .highlight_spacing(HighlightSpacing::Always);

         let info = if let Some(i) = app.items.state.selected() {
            match app.items.commit_items[i] {
                CommitType::Fix => "✓ DONE: ".to_string(),
                CommitType::Feat => "TODO: ".to_string(),
                _ => "Not impl'd yet".to_string(),
            }
        } else {
            "Nothing to see here...".to_string()
        };
   
        frame.render_widget(items, chunks[1]);

        let info = if let Some(i) = app.items.state.selected() {
            match app.items.commit_items[i] {
                CommitType::Fix => "✓ DONE: ".to_string(),
                CommitType::Feat => "TODO: ".to_string(),
                _ => "Not impl'd yet".to_string(),
            }
        } else {
            "Nothing to see here...".to_string()
        };
   
        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title("Conventional Commit")
            .fg(TEXT_COLOR)
            .bg(TODO_HEADER_BG);
        let inner_info_block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(1))
            .bg(NORMAL_ROW_COLOR);
   

}
// pub fn ui(f: &mut Frame, app: &App) {
// }
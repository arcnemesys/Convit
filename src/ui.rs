use ratatui::{
 layout::{Constraint, Direction, Layout, Rect},
 style::{Color, Style},
 text::{Line, Span, Text},
 widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
 Frame,
};

// pub fn ui(f: &mut Frame, app: &App) {
//  let chunks = Layout::default()
//   .direction(Direction::Vertical)
//   .constraints(vec![
//                Constraint::Length(2),
//             Constraint::Min(0),
//             Constraint::Length(2),
//   ]).split(f.size());

//   let title_block = Block::default()
//    .borders(Borders::NONE)
//    .style(Style::default());

//   let title = Paragraph::new(Text::styled(
//    "Convit",
//    Style::default().fg(Color::Green)
//   ))
//   .block(title_block);


//  f.render_widget(title, chunks[0])
// }
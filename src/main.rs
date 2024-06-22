//! # [Ratatui] List example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

// TODO
#![allow(clippy::enum_glob_use, clippy::wildcard_imports)]

use std::{error::Error, io, io::stdout};

use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use convit::{app::{App, AppResult}, event::{Event, EventHandler}, handler::handle_key_events, tui::Tui};
const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

// pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum CurrentlyEditing {
    CommitType,
    CommitScope,
    CommitDescription,
    CommitBody,
    CommitFooters,
}

#[derive(Debug, Clone)]
struct StatefulList {
    state: ListState,
    commit_items: Vec<CommitType>,
    last_selected: Option<usize>,
}
impl StatefulList {
 fn with_items(commit_items: Vec<CommitType>) -> StatefulList {
     StatefulList {
         state: ListState::default(),
         commit_items,
         last_selected: None,
     }
 }

 fn next(&mut self) {
     let i = match self.state.selected() {
         Some(i) => {
             if i >= self.commit_items.len() - 1 {
                 0
             } else {
                 i + 1
             }
         }
         None => self.last_selected.unwrap_or(0),
     };
     self.state.select(Some(i));
 }

 fn previous(&mut self) {
     let i = match self.state.selected() {
         Some(i) => {
             if i == 0 {
                 self.commit_items.len() - 1
             } else {
                 i - 1
             }
         }
         None => self.last_selected.unwrap_or(0),
     };
     self.state.select(Some(i));
 }

 fn unselect(&mut self) {
     let offset = self.state.offset();
     self.last_selected = self.state.selected();
     self.state.select(None);
     *self.state.offset_mut() = offset;
 }
}

// #[derive(Debug, Clone)]
// struct App {
//     items: StatefulList,
// }

// impl App {
//  fn new() -> Self {
//      Self {
//          items: StatefulList::with_items(vec![
//              CommitType::Fix,
//              CommitType::Feat,
//              CommitType::Build,
//              CommitType::Chore,
//              CommitType::Ci,
//              CommitType::Docs,
//              CommitType::Style,
//              CommitType::Refactor,
//              CommitType::Perf,
//              CommitType::Test,
//          ]),
//      }
//  }

//  fn go_top(&mut self) {
//      self.items.state.select(Some(0));
//  }

//  fn go_bottom(&mut self) {
//      self.items.state.select(Some(self.items.commit_items.len() - 1));
//  }
// }

// impl App {
//  fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
//      loop {
//          self.draw(&mut terminal)?;

//          if let Event::Key(key) = event::read()? {
//              if key.kind == KeyEventKind::Press {e
//                  use KeyCode::*;
//                  match key.code {
//                      Char('q') | Esc => return Ok(()),
//                      Char('h') | Left => self.items.unselect(),
//                      Char('j') | Down => self.items.next(),
//                      Char('k') | Up => self.items.previous(),
//                      // Char('l') | Right | Enter => self.change_status(),
//                      Char('g') => self.go_top(),
//                      Char('G') => self.go_bottom(),
//                      _ => {}
//                  }
//              }
//          }
//      }
//  }

//  fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
//      terminal.draw(|f| f.render_widget(self, f.size()))?;
//      Ok(())
//  }
// }

// impl Widget for &mut App {
//  fn render(self, area: Rect, buf: &mut Buffer) {
//      // Create a space for header, todo list and the footer.

//      let vertical = Layout::vertical([
//          Constraint::Length(2),
//          Constraint::Min(0),
//          Constraint::Length(2),
//      ]);
//      let [header_area, rest_area, footer_area] = vertical.areas(area);
     

//      // Create two chunks with equal vertical screen space. One for the list and the other for
//      // the info block.
//      let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
//      let [upper_item_list_area, lower_item_list_area] = vertical.areas(rest_area);

//      render_title(header_area, buf);
//      self.render_todo(upper_item_list_area, buf);
//      self.render_info(lower_item_list_area, buf);
//      render_footer(footer_area, buf);
//  }
// }

// impl App {
//  fn render_todo(&mut self, area: Rect, buf: &mut Buffer) {
//      // We create two blocks, one is for the header (outer) and the other is for list (inner).
//      let outer_block = Block::new()
//          .borders(Borders::NONE)
//          .title_alignment(Alignment::Center)
//          .title("Commit Type")
//          .fg(TEXT_COLOR)
//          .bg(TODO_HEADER_BG);
//      let inner_block = Block::new()
//          .borders(Borders::NONE)
//          .fg(TEXT_COLOR)
//          .bg(NORMAL_ROW_COLOR);

//      // We get the inner area from outer_block. We'll use this area later to render the table.
//      let outer_area = area;
//      let inner_area = outer_block.inner(outer_area);

//      // We can render the header in outer_area.
//      outer_block.render(outer_area, buf);

//      // Iterate through all elements in the `items` and stylize them.
//      let items: Vec<ListItem> = self
//          .items
//          .commit_items
//          .iter()
//          .enumerate()
//          .map(|(i, commit_type)| commit_type.to_list_item(i))
//          .collect();

//      // Create a List from all list items and highlight the currently selected one
//      let items = List::new(items)
//          .block(inner_block)
//          .highlight_style(
//              Style::default()
//                  .add_modifier(Modifier::BOLD)
//                  .add_modifier(Modifier::REVERSED)
//                  .fg(SELECTED_STYLE_FG),
//          )
//          .highlight_symbol(">")
//          .highlight_spacing(HighlightSpacing::Always);

//      // We can now render the item list
//      // (look careful we are using StatefulWidget's render.)
//      // ratatui::widgets::StatefulWidget::render as stateful_render
//      StatefulWidget::render(items, inner_area, buf, &mut self.items.state);
//  }

//  fn render_info(&self, area: Rect, buf: &mut Buffer) {
//      // We get the info depending on the item's state.
//      let info = if let Some(i) = self.items.state.selected() {
//          match self.items.commit_items[i] {
//              CommitType::Fix => "✓ DONE: ".to_string(),
//              CommitType::Feat => "TODO: ".to_string(),
//              _ => "Not impl'd yet".to_string(),
//          }
//      } else {
//          "Nothing to see here...".to_string()
//      };

//      // We show the list item's info under the list in this paragraph
//      let outer_info_block = Block::new()
//          .borders(Borders::NONE)
//          .title_alignment(Alignment::Center)
//          .title("Conventional Commit")
//          .fg(TEXT_COLOR)
//          .bg(TODO_HEADER_BG);
//      let inner_info_block = Block::new()
//          .borders(Borders::NONE)
//          .padding(Padding::horizontal(1))
//          .bg(NORMAL_ROW_COLOR);

//      // This is a similar process to what we did for list. outer_info_area will be used for
//      // header inner_info_area will be used for the list info.
//      let outer_info_area = area;
//      let inner_info_area = outer_info_block.inner(outer_info_area);

//      // We can render the header. Inner info will be rendered later
//      outer_info_block.render(outer_info_area, buf);

//      let info_paragraph = Paragraph::new(info)
//          .block(inner_info_block)
//          .fg(TEXT_COLOR)
//          .wrap(Wrap { trim: false });

//      // We can now render the item info
//      info_paragraph.render(inner_info_area, buf);
//  }
// }

fn render_title(area: Rect, buf: &mut Buffer) {
 Paragraph::new("Convit").bold().centered().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
 Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
     .centered()
     .render(area, buf);
}

#[derive(Debug, Clone)]
pub enum CurrentScreen {
    Main,
    Editing,
    // EditingCommitType,
    // EditingCommitScope,
    // EditingCommitDescription,
    // EditingCommitBody,
    // EditingCommitFooters,
    Exiting,
}


// Users should be able to select from a list of commit types,
// which will spare us having to mutate a string to match it
// against the enum, to make sure the commit type input
// is valid.



impl<'a> Default for ConventionalCommit<'a> {
 fn default() -> Self {
     Self {
         commit_type: "",
         scope: None,
         description: "",
         body: None,
         footers: None,
         commit_status: CommitStatus::Unready,
     }
 }
}

impl<'a> ConventionalCommit<'a> {
 fn new() -> Self {
     ConventionalCommit::default()
 }
}

// #[derive(Debug, Clone)]
// pub struct Convit<'a> {
//     pub conventional_commit: ConventionalCommit<'a>,
//     pub commit_type_input: &'a str,
//     pub commit_scope: Option<&'a str>,
//     pub commit_description: &'a str,
//     pub commit_body: Option<&'a str>,
//     pub commit_footers: Option<HashMap<CommitFooter, &'a str>>,
//     pub current_screen: CurrentScreen,
//     pub currently_editing: Option<CurrentlyEditing>,
// }


// impl<'a> Default for Convit<'a> {
//     fn default() -> Self {
//         Self {
//             conventional_commit: ConventionalCommit::default(),
//             commit_type_input: "",
//             commit_scope: None,
//             commit_description: "",
//             commit_body: None,
//             commit_footers: None,
//             current_screen: CurrentScreen::Main,
//             currently_editing: None,
//         }
//     }
// }



impl From<CommitType> for String {
    fn from(value: CommitType) -> Self {
        let commit_type = match value {
            CommitType::Fix => String::from("fix"),
            CommitType::Feat => String::from("feat"),
            CommitType::Build => String::from("build"),
            CommitType::Chore => String::from("chore"),
            CommitType::Ci => String::from("ci"),
            CommitType::Docs => String::from("docs"),
            CommitType::Style => String::from("style"),
            CommitType::Refactor => String::from("refactor"),
            CommitType::Revert => String::from("revert"),
            CommitType::Perf => String::from("perf"),
            CommitType::Test => String::from("test")
        };

        commit_type
    }
}

impl From<&CommitType> for String {
    fn from(value: &CommitType) -> Self {
        let commit_type = match value {
            CommitType::Fix => String::from("fix"),
            CommitType::Feat => String::from("feat"),
            CommitType::Build => String::from("build"),
            CommitType::Chore => String::from("chore"),
            CommitType::Ci => String::from("ci"),
            CommitType::Docs => String::from("docs"),
            CommitType::Style => String::from("style"),
            CommitType::Refactor => String::from("refactor"),
            CommitType::Revert => String::from("revert"),
            CommitType::Perf => String::from("perf"),
            CommitType::Test => String::from("test")
        };

        commit_type
    }
}

impl From<String> for CommitType {
    fn from(value: String) -> Self {
        let commit_type = match value.to_lowercase().as_str() {
            "fix" => CommitType::Fix,
            "feat" => CommitType::Feat,
            "build" => CommitType::Build,
            "chore" => CommitType::Chore,
            "ci" => CommitType::Ci,
            "docs" => CommitType::Docs,
            "style" => CommitType::Style,
            "refactor" => CommitType::Refactor,
            "perf" => CommitType::Perf,
            "test" => CommitType::Test,
            &_ => unreachable!()
        };

        commit_type
        
    }
}
// impl<'a> Convit<'a> {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn save_commit_type(&mut self) {
//         self.conventional_commit.commit_type = self.commit_type_input.clone();
//     }
// }

#[derive(PartialEq, Debug, Clone)]
pub enum CommitType {
    Fix,
    Feat,
    Build,
    Chore,
    Ci,
    Docs,
    Style,
    Refactor,
    Revert,
    Perf,
    Test,
}

impl CommitType {
 fn to_list_item(&self, index: usize) -> ListItem {
     let bg_color = NORMAL_ROW_COLOR;

     let line = match self {
         &CommitType::Fix => Line::styled(
             format!("{:?}: Use when making changes to patch a bug.", self),
             TEXT_COLOR,
         ),
         &CommitType::Feat => Line::styled(
             format!("{:?}: Use when adding a new feature.", self),
             (COMPLETED_TEXT_COLOR, bg_color),
         ),
         &CommitType::Build => Line::styled(
             format!(
                 "{:?}: Use when changing the build system or external dependencies.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Chore => Line::styled(
             format!(
                 "{:?}: Use when making non-functional changes that don't concern the codebase.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Ci => Line::styled(
             format!(
                 "{:?}: Use when changing CI configurations or scripts.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Docs => Line::styled(
             format!("{:?}: Use when making changes to documentation.", self),
             TEXT_COLOR,
         ),
         &CommitType::Style => Line::styled(
             format!(
                 "{:?}: Use when making non-semantic changes, such as formatting.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Refactor => Line::styled(
             format!(
                 "{:?}: Use when making changes that don't fix a bug or add a feature.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Revert => Line::styled(
             format!("{:?}: Use when reverting a previous/prior commit.", self),
             TEXT_COLOR,
         ),
         &CommitType::Perf => Line::styled(
             format!(
                 "{:?}: Use when making changes to improve performance.",
                 self
             ),
             TEXT_COLOR,
         ),
         &CommitType::Test => Line::styled(
             format!(
                 "{:?}: Use when adding tests or editing existing ones.",
                 self
             ),
             TEXT_COLOR,
         ),
     };

     ListItem::new(line).bg(bg_color)
 }
}
#[derive(Debug, Clone)]
pub enum CommitFooter {
    BreakingChange,
    SignedOffBy,
    AckedBy,
    HelpedBy,
    ReferenceTo,
    SeeAlso,
    Fixes,
    Cc,
    ReviewedBy
}

#[derive(Copy, Clone, Debug)]
pub enum CommitStatus {
    Ready,
    Unready,
}


#[derive(Debug, Clone)]
pub struct ConventionalCommit<'a> {
    pub commit_type: &'a str,
    pub scope: Option<&'a str>,
    pub description: &'a str,
    pub body: Option<&'a str>,
    pub footers: Option<Vec<&'a str>>,
    pub commit_status: CommitStatus,
}


fn main() -> AppResult<()> {
 // setup terminal

 // Create the application.
 let mut app = App::new();

 // Initialize terminal user interface
 let backend = CrosstermBackend::new(io::stderr());

 // let terminal = init_terminal()?;

 let terminal = Terminal::new(backend)?;
 let events = EventHandler::new(250);
 let mut tui = Tui::new(terminal, events);
 tui.init()?;

 // Start the main loop.
 while app.running {
  tui.draw(&mut app)?;

  // Handle events.
  match tui.events.next()? {
   Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
   Event::Mouse(_) => {},
   Event::Resize(_, _) => {}
  }
 }

 app.quit();
 restore_terminal()?;

 // create app and run it


 Ok(())
}

fn init_error_hooks() -> color_eyre::Result<()> {
 let (panic, error) = HookBuilder::default().into_hooks();
 let panic = panic.into_panic_hook();
 let error = error.into_eyre_hook();
 color_eyre::eyre::set_hook(Box::new(move |e| {
     let _ = restore_terminal();
     error(e)
 }))?;
 std::panic::set_hook(Box::new(move |info| {
     let _ = restore_terminal();
     panic(info);
 }));
 Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
 enable_raw_mode()?;
 stdout().execute(EnterAlternateScreen)?;
 let backend = CrosstermBackend::new(stdout());
 let terminal = Terminal::new(backend)?;
 Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
 disable_raw_mode()?;
 stdout().execute(LeaveAlternateScreen)?;
 Ok(())
}
// const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
// const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
// const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
// const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
// const TEXT_COLOR: Color = tailwind::SLATE.c200;
// const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

// #[derive(Copy, Clone)]
// enum Status {
//     Todo,
//     Completed,
// }

// struct TodoItem<'a> {
//     todo: &'a str,
//     info: &'a str,
//     status: Status,
// }

// struct StatefulList<'a> {
//     state: ListState,
//     items: Vec<TodoItem<'a>>,
//     last_selected: Option<usize>,
// }

// /// This struct holds the current state of the app. In particular, it has the `items` field which is
// /// a wrapper around `ListState`. Keeping track of the items state let us render the associated
// /// widget with its state and have access to features such as natural scrolling.
// ///
// /// Check the event handling at the bottom to see how to change the state on incoming events.
// /// Check the drawing logic for items on how to specify the highlighting style for selected items.
// struct App<'a> {
//     items: StatefulList<'a>,
// }

// impl<'a> App<'a> {
//     fn new() -> Self {
//         Self {
//             items: StatefulList::with_items([
//                 ("Rewrite everything with Rust!", "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust", Status::Todo),
//                 ("Rewrite all of your tui apps with Ratatui", "Yes, you heard that right. Go and replace your tui with Ratatui.", Status::Completed),
//                 ("Pet your cat", "Minnak loves to be pet by you! Don't forget to pet and give some treats!", Status::Todo),
//                 ("Walk with your dog", "Max is bored, go walk with him!", Status::Todo),
//                 ("Pay the bills", "Pay the train subscription!!!", Status::Completed),
//                 ("Refactor list example", "If you see this info that means I completed this task!", Status::Completed),
//             ]),
//         }
//     }

//     /// Changes the status of the selected list item
//     fn change_status(&mut self) {
//         if let Some(i) = self.items.state.selected() {
//             self.items.items[i].status = match self.items.items[i].status {
//                 Status::Completed => Status::Todo,
//                 Status::Todo => Status::Completed,
//             }
//         }
//     }

//     fn go_top(&mut self) {
//         self.items.state.select(Some(0));
//     }

//     fn go_bottom(&mut self) {
//         self.items.state.select(Some(self.items.items.len() - 1));
//     }
// }

// impl App<'_> {
//     fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
//         loop {
//             self.draw(&mut terminal)?;

//             if let Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press {
//                     use KeyCode::*;
//                     match key.code {
//                         Char('q') | Esc => return Ok(()),
//                         Char('h') | Left => self.items.unselect(),
//                         Char('j') | Down => self.items.next(),
//                         Char('k') | Up => self.items.previous(),
//                         Char('l') | Right | Enter => self.change_status(),
//                         Char('g') => self.go_top(),
//                         Char('G') => self.go_bottom(),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }

//     fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
//         terminal.draw(|f| f.render_widget(self, f.size()))?;
//         Ok(())
//     }
// }

// impl Widget for &mut App<'_> {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         // Create a space for header, todo list and the footer.
//         let vertical = Layout::vertical([
//             Constraint::Length(2),
//             Constraint::Min(0),
//             Constraint::Length(2),
//         ]);
//         let [header_area, rest_area, footer_area] = vertical.areas(area);

//         // Create two chunks with equal vertical screen space. One for the list and the other for
//         // the info block.
//         let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
//         let [upper_item_list_area, lower_item_list_area] = vertical.areas(rest_area);

//         render_title(header_area, buf);
//         self.render_todo(upper_item_list_area, buf);
//         self.render_info(lower_item_list_area, buf);
//         render_footer(footer_area, buf);
//     }
// }

// impl App<'_> {
//     fn render_todo(&mut self, area: Rect, buf: &mut Buffer) {
//         // We create two blocks, one is for the header (outer) and the other is for list (inner).
//         let outer_block = Block::new()
//             .borders(Borders::NONE)
//             .title_alignment(Alignment::Center)
//             .title("TODO List")
//             .fg(TEXT_COLOR)
//             .bg(TODO_HEADER_BG);
//         let inner_block = Block::new()
//             .borders(Borders::NONE)
//             .fg(TEXT_COLOR)
//             .bg(NORMAL_ROW_COLOR);

//         // We get the inner area from outer_block. We'll use this area later to render the table.
//         let outer_area = area;
//         let inner_area = outer_block.inner(outer_area);

//         // We can render the header in outer_area.
//         outer_block.render(outer_area, buf);

//         // Iterate through all elements in the `items` and stylize them.
//         let items: Vec<ListItem> = self
//             .items
//             .items
//             .iter()
//             .enumerate()
//             .map(|(i, todo_item)| todo_item.to_list_item(i))
//             .collect();

//         // Create a List from all list items and highlight the currently selected one
//         let items = List::new(items)
//             .block(inner_block)
//             .highlight_style(
//                 Style::default()
//                     .add_modifier(Modifier::BOLD)
//                     .add_modifier(Modifier::REVERSED)
//                     .fg(SELECTED_STYLE_FG),
//             )
//             .highlight_symbol(">")
//             .highlight_spacing(HighlightSpacing::Always);

//         // We can now render the item list
//         // (look careful we are using StatefulWidget's render.)
//         // ratatui::widgets::StatefulWidget::render as stateful_render
//         StatefulWidget::render(items, inner_area, buf, &mut self.items.state);
//     }

//     fn render_info(&self, area: Rect, buf: &mut Buffer) {
//         // We get the info depending on the item's state.
//         let info = if let Some(i) = self.items.state.selected() {
//             match self.items.items[i].status {
//                 Status::Completed => "✓ DONE: ".to_string() + self.items.items[i].info,
//                 Status::Todo => "TODO: ".to_string() + self.items.items[i].info,
//             }
//         } else {
//             "Nothing to see here...".to_string()
//         };

//         // We show the list item's info under the list in this paragraph
//         let outer_info_block = Block::new()
//             .borders(Borders::NONE)
//             .title_alignment(Alignment::Center)
//             .title("TODO Info")
//             .fg(TEXT_COLOR)
//             .bg(TODO_HEADER_BG);
//         let inner_info_block = Block::new()
//             .borders(Borders::NONE)
//             .padding(Padding::horizontal(1))
//             .bg(NORMAL_ROW_COLOR);

//         // This is a similar process to what we did for list. outer_info_area will be used for
//         // header inner_info_area will be used for the list info.
//         let outer_info_area = area;
//         let inner_info_area = outer_info_block.inner(outer_info_area);

//         // We can render the header. Inner info will be rendered later
//         outer_info_block.render(outer_info_area, buf);

//         let info_paragraph = Paragraph::new(info)
//             .block(inner_info_block)
//             .fg(TEXT_COLOR)
//             .wrap(Wrap { trim: false });

//         // We can now render the item info
//         info_paragraph.render(inner_info_area, buf);
//     }
// }

// fn render_title(area: Rect, buf: &mut Buffer) {
//     Paragraph::new("Ratatui List Example")
//         .bold()
//         .centered()
//         .render(area, buf);
// }

// fn render_footer(area: Rect, buf: &mut Buffer) {
//     Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
//         .centered()
//         .render(area, buf);
// }

// impl StatefulList<'_> {
//     fn with_items<'a>(items: [(&'a str, &'a str, Status); 6]) -> StatefulList<'a> {
//         StatefulList {
//             state: ListState::default(),
//             items: items.iter().map(TodoItem::from).collect(),
//             last_selected: None,
//         }
//     }

//     fn next(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => self.last_selected.unwrap_or(0),
//         };
//         self.state.select(Some(i));
//     }

//     fn previous(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => self.last_selected.unwrap_or(0),
//         };
//         self.state.select(Some(i));
//     }

//     fn unselect(&mut self) {
//         let offset = self.state.offset();
//         self.last_selected = self.state.selected();
//         self.state.select(None);
//         *self.state.offset_mut() = offset;
//     }
// }

// impl TodoItem<'_> {
//     fn to_list_item(&self, index: usize) -> ListItem {
//         let bg_color = match index % 2 {
//             0 => NORMAL_ROW_COLOR,
//             _ => ALT_ROW_COLOR,
//         };
//         let line = match self.status {
//             Status::Todo => Line::styled(format!(" ☐ {}", self.todo), TEXT_COLOR),
//             Status::Completed => Line::styled(
//                 format!(" ✓ {}", self.todo),
//                 (COMPLETED_TEXT_COLOR, bg_color),
//             ),
//         };

//         ListItem::new(line).bg(bg_color)
//     }
// }

// impl<'a> From<&(&'a str, &'a str, Status)> for TodoItem<'a> {
//     fn from((todo, info, status): &(&'a str, &'a str, Status)) -> Self {
//         Self {
//             todo,
//             info,
//             status: *status,
//         }
//     }
// }

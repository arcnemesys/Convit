use std::error;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Clone)]
pub enum CurrentlyEditing {
    CommitType,
    CommitFooters,
    CommitDescription,
    CommitBody,
    CommitScope,
}

#[derive(Copy, Clone, Debug)]
pub enum CommitStatus {
    Ready,
    Unready,
}

#[derive(Debug, Clone)]
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
    ReviewedBy,
}

#[derive(Debug, Clone)]
pub struct ConventionalCommit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub footers: Option<Vec<String>>,
}
impl Default for ConventionalCommit {
    fn default() -> Self {
        Self {
            commit_type: String::new(),
            scope: None,
            description: String::new(),
            body: None,
            footers: None,
        }
    }
}

impl ConventionalCommit {
    pub fn new() -> Self {
        ConventionalCommit::default()
    }
}
pub struct App {
    pub commit_type: CommitType,
    pub commit_footers: Option<Vec<CommitFooter>>,
    pub commit_description: String,
    pub commit_scope: Option<String>,
    pub commit_body: Option<String>,
    pub commit_status: CommitStatus,
    pub convit: ConventionalCommit,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            commit_type: CommitType::Fix,
            commit_footers: None,
            commit_description: String::new(),
            commit_scope: None,
            commit_body: None,
            commit_status: CommitStatus::Unready,
            convit: ConventionalCommit::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn save_commit_type(&mut self) {
        let commit_type = match self.commit_type {
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
            CommitType::Test => String::from("test"),
        };

        self.convit.commit_type = commit_type;
    }

    fn render_commit_types(&mut self, area: Rect, but: &mut Buffer) {
        let commit_types = vec!["feat", "fix", "docs", "style", "refactor", "test", "chore"];
        let commit_items: Vec<ListItem> = commit_types.iter().map(|i| ListItem::new(*i)).collect();
        let commit_list = List::new(commit_items)
            .block(Block::default().borders(Borders::ALL).title("Commit Type"));
    }
    fn render_commit_footers(&mut self, area: Rect, but: &mut Buffer) { todo!() }
    fn render_commit_description(&mut self, area: Rect, but: &mut Buffer) { todo!() }
    fn render_commit_body(&mut self, area: Rect, but: &mut Buffer) { todo!() }

    fn render_commit(&mut self, area: Rect, but: &mut Buffer) { todo!() }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]);
    }
}

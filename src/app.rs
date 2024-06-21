use std::collections::HashMap;

use serde_json::Result;

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

pub enum CurrentlyEditing {
    CommitType,
    CommitScope,
    CommitDescription,
    CommitBody,
    CommitFooters,
}

// Users should be able to select from a list of commit types,
// which will spare us having to mutate a string to match it
// against the enum, to make sure the commit type input
// is valid.

#[derive(PartialEq)]
pub enum CommitType {
    Fix,
    Feat,
    Build,
    Chore,
    Ci,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
}

pub enum CommitFooter {
    BreakingChange,
}

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
            footers: None
        }
    }
}

impl ConventionalCommit {
    fn new() -> Self {
        ConventionalCommit::default()
    }
}
pub struct App {
    pub conventional_commit: ConventionalCommit,
    pub commit_type_input: String,
    pub commit_scope: Option<String>,
    pub commit_description: String,
    pub commit_body: Option<String>,
    pub commit_footers: Option<HashMap<CommitFooter, String>>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            conventional_commit: ConventionalCommit::default(),
            commit_type_input: String::new(),
            commit_scope: None,
            commit_description: String::new(),
            commit_body: None,
            commit_footers: None,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

// pub fn match_commit_type(commit_input:)


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
impl App {
    pub fn new() -> App {
        App::default()
    }
    //  pub fn toggle_edit
    pub fn save_commit_type(&mut self) {
        self.conventional_commit.commit_type = self.commit_type_input.clone();
    }

    pub fn update_commit_type(&mut self, commit_type: &CommitType) {
        if self.conventional_commit.commit_type != String::from(commit_type) {
            self.conventional_commit.commit_type = commit_type.into()
        }
    }
}

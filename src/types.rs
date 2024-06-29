use ratatui::{prelude::*, widgets::*};

use crate::ui::{TODO_HEADER_BG, NORMAL_ROW_COLOR, ALT_ROW_COLOR, SELECTED_STYLE_FG, TEXT_COLOR, COMPLETED_TEXT_COLOR};

#[derive(Debug, Clone)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
#[derive(Debug, Clone)]
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
    pub fn to_list_item(&self, index: usize) -> ListItem {
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
pub enum CommitFooters {
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

impl CommitFooters {
    pub fn to_list_item(&self, index: usize) -> ListItem {
        let bg_color = NORMAL_ROW_COLOR;

        let line = match self {
            &CommitFooters::BreakingChange => Line::styled(
                format!("{:?}: Use when making changes to patch a bug.", self),
                TEXT_COLOR,
            ),
            &CommitFooters::SignedOffBy => Line::styled(
                format!("{:?}: Use when adding a new feature.", self),
                (COMPLETED_TEXT_COLOR, bg_color),
            ),
            &CommitFooters::AckedBy => Line::styled(
                format!(
                    "{:?}: Use when changing the build system or external dependencies.",
                    self
                ),
                TEXT_COLOR,
            ),
            &CommitFooters::HelpedBy => Line::styled(
                format!(
                    "{:?}: Use when making non-functional changes that don't concern the codebase.",
                    self
                ),
                TEXT_COLOR,
            ),
            &CommitFooters::ReferenceTo => Line::styled(
                format!(
                    "{:?}: Use when changing CI configurations or scripts.",
                    self
                ),
                TEXT_COLOR,
            ),
            &CommitFooters::SeeAlso => Line::styled(
                format!("{:?}: Use when making changes to documentation.", self),
                TEXT_COLOR,
            ),
            &CommitFooters::Fixes => Line::styled(
                format!(
                    "{:?}: Use when making non-semantic changes, such as formatting.",
                    self
                ),
                TEXT_COLOR,
            ),
            &CommitFooters::Cc => Line::styled(
                format!(
                    "{:?}: Use when making changes that don't fix a bug or add a feature.",
                    self
                ),
                TEXT_COLOR,
            ),
            &CommitFooters::ReviewedBy => Line::styled(
                format!("{:?}: Use when reverting a previous/prior commit.", self),
                TEXT_COLOR,
            ),
        };

        ListItem::new(line).bg(bg_color)
    }
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
    pub fn new() -> Self {
        ConventionalCommit::default()
    }
}

#![allow(clippy::enum_glob_use, clippy::wildcard_imports)]

use std::{collections::HashMap, error::Error, io::{self, stdout}};

use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

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
    fn to_list_item(&self, index: usize) -> ListItem {
        let bg_color = NORMAL_ROW_COLOR;

        let line = match self {
            &CommitType::Fix => Line::styled(format!("{:?}: Use when making changes to patch a bug.", self), TEXT_COLOR),
            &CommitType::Feat => Line::styled(
                format!("{:?}: Use when adding a new feature.", self),
                (COMPLETED_TEXT_COLOR, bg_color)
            ),
            &CommitType::Build => Line::styled(format!("{:?}: Use when changing the build system or external dependencies.", self), TEXT_COLOR),
            &CommitType::Chore => Line::styled(format!("{:?}: Use when making non-functional changes that don't concern the codebase.", self), TEXT_COLOR),
            &CommitType::Ci => Line::styled(format!("{:?}: Use when changing CI configurations or scripts.", self), TEXT_COLOR),
            &CommitType::Docs => Line::styled(format!("{:?}: Use when making changes to documentation.", self), TEXT_COLOR),
            &CommitType::Style => Line::styled(format!("{:?}: Use when making non-semantic changes, such as formatting.", self), TEXT_COLOR),
            &CommitType::Refactor => Line::styled(format!("{:?}: Use when making changes that don't fix a bug or add a feature.", self), TEXT_COLOR),
            &CommitType::Revert => Line::styled(format!("{:?}: Use when reverting a previous/prior commit.", self), TEXT_COLOR),
            &CommitType::Perf => Line::styled(format!("{:?}: Use when making changes to improve performance.", self), TEXT_COLOR),
            &CommitType::Test => Line::styled(format!("{:?}: Use when adding tests or editing existing ones.", self), TEXT_COLOR),
        };

        ListItem::new(line).bg(bg_color)
    }
}

#[derive(Debug, Clone)]
pub enum CommitFooter {
    BreakingChange,
}


impl<'a> Default for ConventionalCommit<'a> {
    fn default() -> Self {
        Self {
            commit_type: "",
            scope: None,
            description: "",
            body: None,
            footers: None,
            commit_status: CommitStatus::Unready
        }
    }
}

impl<'a> ConventionalCommit<'a> {
    fn new() -> Self {
        ConventionalCommit::default()
    }
}
#[derive(Debug, Clone)]
pub struct Convit<'a> {
    pub conventional_commit: ConventionalCommit<'a>,
    pub commit_type_input: &'a str,
    pub commit_scope: Option<&'a str>,
    pub commit_description: &'a str,
    pub commit_body: Option<&'a str>,
    pub commit_footers: Option<HashMap<CommitFooter, &'a str>>,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl<'a> Default for Convit<'a> {
    fn default() -> Self {
        Self {
            conventional_commit: ConventionalCommit::default(),
            commit_type_input: "",
            commit_scope: None,
            commit_description: "",
            commit_body: None,
            commit_footers: None,
            currently_editing: None,
        }
    }
}


const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    Completed,
}

#[derive(Copy, Clone, Debug)]
pub enum CommitStatus {
    Ready,
    Unready
}

struct TodoItem<'a> {
    todo: &'a str,
    info: &'a str,
    status: Status,
}

#[derive(Debug, Clone)]
pub struct ConventionalCommit<'a> {
    pub commit_type: &'a str,
    pub scope: Option<&'a str>,
    pub description: &'a str,
    pub body: Option<&'a str>,
    pub footers: Option<Vec<&'a str>>,
    pub commit_status: CommitStatus
}
#[derive(Debug, Clone)]
struct StatefulList<'a> {
    state: ListState,
    commit: ConventionalCommit<'a>,
    items: Vec<CommitType>,
    last_selected: Option<usize>,
}
#[derive(Debug, Clone)]
struct App<'a> {
    items: StatefulList<'a>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // create app and run it
    App::new().run(terminal)?;

    restore_terminal()?;

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

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            items: StatefulList::with_items(vec![
                CommitType::Fix,
                CommitType::Feat,
                CommitType::Build,
                CommitType::Chore,
                CommitType::Ci,
                CommitType::Docs,
                CommitType::Style,
                CommitType::Refactor,
                CommitType::Perf,
                CommitType::Test,
            ]),
        }
    }

    fn go_top(&mut self) {
        self.items.state.select(Some(0));
    }

    fn go_bottom(&mut self) {
        self.items.state.select(Some(self.items.items.len() - 1));
    }
}

impl App<'_> {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('h') | Left => self.items.unselect(),
                        Char('j') | Down => self.items.next(),
                        Char('k') | Up => self.items.previous(),
                        // Char('l') | Right | Enter => self.change_status(),
                        Char('g') => self.go_top(),
                        Char('G') => self.go_bottom(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a space for header, todo list and the footer.
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        // Create two chunks with equal vertical screen space. One for the list and the other for
        // the info block.
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [upper_item_list_area, lower_item_list_area] = vertical.areas(rest_area);

        render_title(header_area, buf);
        self.render_todo(upper_item_list_area, buf);
        self.render_info(lower_item_list_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App<'_> {
    fn render_todo(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title("Commit Type")
            .fg(TEXT_COLOR)
            .bg(TODO_HEADER_BG);
        let inner_block = Block::new()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .enumerate()
            .map(|(i, commit_type)| commit_type.to_list_item(i))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        StatefulWidget::render(items, inner_area, buf, &mut self.items.state);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.items.state.selected() {
            match self.items.items[i] {
                CommitType::Fix => "✓ DONE: ".to_string(),
                CommitType::Feat => "TODO: ".to_string(),
                _ => "Not impl'd yet".to_string()
            }
        } else {
            "Nothing to see here...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title("Composed Commit")
            .fg(TEXT_COLOR)
            .bg(TODO_HEADER_BG);
        let inner_info_block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(1))
            .bg(NORMAL_ROW_COLOR);

        // This is a similar process to what we did for list. outer_info_area will be used for
        // header inner_info_area will be used for the list info.
        let outer_info_area = area;
        let inner_info_area = outer_info_block.inner(outer_info_area);

        // We can render the header. Inner info will be rendered later
        outer_info_block.render(outer_info_area, buf);

        let info_paragraph = Paragraph::new(info)
            .block(inner_info_block)
            .fg(TEXT_COLOR)
            .wrap(Wrap { trim: false });

        // We can now render the item info
        info_paragraph.render(inner_info_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Convit")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}

impl StatefulList<'_> {
    fn with_items<'a>(items: Vec<CommitType>) -> StatefulList<'a> {
        StatefulList {
            state: ListState::default(),
            commit: ConventionalCommit::new(),
            items,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
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
                    self.items.len() - 1
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

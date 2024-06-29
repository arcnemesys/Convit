use std::collections::HashMap;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use crate::types::*;
use crate::ui::*;
use std::io;


#[derive(Debug, Clone, Copy, Default, Display, EnumIter, FromRepr, PartialEq, Eq)]
pub enum Tab {
    #[default]
    CommitType,
    CommitFooter
}

impl Tab {
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn prev(self) -> Self {
        let current_index = self as usize;
        let prev_index = current_index.saturating_sub(1);
        Self::from_repr(prev_index).unwrap_or(self)
    }

    fn title(self) -> String {
        match self {
            Self::CommitType => String::from("Commit Types"),
            Self::CommitFooter => String::from("Commit Footers")
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommitTypeTab {
    row: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommitFooterTab {
    row: usize,
}

#[derive(Debug, Clone)]
pub struct App<'a> {
    items: StatefulList<'a>,
    footer_list: StatefulFooterList<'a>,
    pub commit_type_tab: CommitTypeTab,
    pub commit_footer_tab: CommitFooterTab,
    pub tab: Tab,
    pub commit_type: CommitType,
    pub scope: Option<&'a str>,
    pub description: &'a str,
    pub body: Option<&'a str>,
    pub footers: Option<Vec<&'a str>>,
    pub commit_status: CommitStatus,
    pub currently_editing: CurrentlyEditing,
    pub current_screen: CurrentScreen,
}



impl<'a> App<'a> {
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
        let first_vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = first_vertical.areas(area);

        // Create two chunks with equal vertical screen space. One for the list and the other for
        // the info block.
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [upper_item_list_area, lower_item_list_area] = vertical.areas(rest_area);

        let upper_half = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    
        let [commit_types_area, commit_footers_area] = upper_half.areas(upper_item_list_area);

        let bottom_half = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [commit_type_info, commit_message] = bottom_half.areas(lower_item_list_area);


        render_title(header_area, buf);
        self.render_todo(commit_types_area, buf);
        self.render_commit_footers(commit_footers_area, buf);
        self.render_info(commit_type_info, buf);
        self.render_commit_message(commit_message, buf);
        render_footer(footer_area, buf);
    }
}

impl App<'_> {
    fn render_todo(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::new()
            .borders(Borders::ALL)
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
                _ => "Not impl'd yet".to_string(),
            }
        } else {
            "Nothing to see here...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .title("Commit Message")
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

    fn render_commit_message(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        // let info = if let Some(i) = self.items.state.selected() {
        //     match self.items.items[i] {
        //         CommitType::Fix => "✓ DONE: ".to_string(),
        //         CommitType::Feat => "TODO: ".to_string(),
        //         _ => "Not impl'd yet".to_string(),
        //     }
        // } else {
        //     "Nothing to see here...".to_string()
        // };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::ALL)
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

        let info_paragraph = Paragraph::new("Your commit here...")
            .block(inner_info_block)
            .fg(TEXT_COLOR)
            .wrap(Wrap { trim: false });

        // We can now render the item info
        info_paragraph.render(inner_info_area, buf);
    }

    fn render_commit_footers(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .title("Commit Footers")
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
            .footer_list
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
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Convit").bold().centered().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}

fn render_commit(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Commit Message")
        .bold()
        .centered()
        .render(area, buf);

    let outer_info_area = area;
}

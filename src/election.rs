use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{widgets::ListState, Frame};

use crate::{candidate::Candidate, Action, Screen};
use std::iter::once;

use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, List, Paragraph},
};
macro_rules! colored {
    ($color:ident) => {
        ratatui::style::Style::default().fg(ratatui::style::Color::$color)
    };
}

#[derive(Default)]
pub struct ElectionScreen {
    pub search: String,
    pub candidates: Vec<Candidate>,
    pub filtered: Vec<usize>,
    pub list_state: ListState,
    pub state: VoteState,
}

pub struct VoteState {
    pub include_empty: bool,
    pub second: bool,
    pub vote: u32,
    pub last: Option<usize>,
}

impl Default for VoteState {
    fn default() -> Self {
        Self {
            include_empty: true,
            second: false,
            vote: 1,
            last: None,
        }
    }
}

impl VoteState {
    fn next_candidate(&mut self) {
        self.include_empty = true;
        self.second = false;
        self.vote += 1;
        self.last = None;
    }

    fn select_candidate(&mut self, candidate: &mut Candidate, index: usize) {
        if self.second {
            self.next_candidate();
            candidate.second_vote();
        } else {
            self.second = true;
            candidate.first_vote();
            self.last = Some(index);
        }
    }

    fn select_empty(&mut self) {
        self.include_empty = false;
        if self.second {
            self.next_candidate();
        } else {
            self.second = true;
        }
    }
}

impl ElectionScreen {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut screen = Self {
            list_state: ListState::default().with_selected(Some(0)),
            candidates,
            ..Default::default()
        };
        screen.update_filter();
        screen
    }
    pub fn update_filter(&mut self) {
        self.filtered = (0..self.candidates.len())
            .filter(|i| {
                self.candidates[*i]
                    .name
                    .to_lowercase()
                    .starts_with(&self.search.to_lowercase())
                    && self.state.last != Some(*i)
            })
            .collect();
        if self.list_state.selected().is_none() {
            self.list_state.select_next();
        }
    }

    pub fn select(&mut self) {
        let Some(index) = self.list_state.selected() else {
            return;
        };

        let index = if self.state.include_empty && self.search.is_empty() {
            index.checked_sub(1)
        } else {
            Some(index)
        };

        match index {
            Some(index) => self
                .state
                .select_candidate(&mut self.candidates[self.filtered[index]], index),
            None => self.state.select_empty(),
        }
        self.search.clear();
        self.update_filter();
        self.list_state = ListState::default().with_selected(Some(0));
    }

    fn draw_voting(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);
        let input = Paragraph::new(&*self.search).block(
            Block::bordered()
                .title(Line::from("Search").centered())
                .title(
                    Line::from(format!(
                        "{:03}: {} Vote",
                        self.state.vote,
                        if self.state.second { "Second" } else { "First" }
                    ))
                    .left_aligned(),
                )
                .border_type(BorderType::Rounded)
                .border_style(colored!(LightCyan))
                .title_style(Style::default()),
        );
        let input_area = chunks[0];
        frame.render_widget(input, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + u16::try_from(self.search.len()).unwrap_or(input_area.width - 2) + 1,
            input_area.y + 1,
        ));

        let mut candidates = once("(Empty)".to_string()).chain(
            self.filtered
                .iter()
                .map(|index| format!("{:?}", &self.candidates[*index])),
        );
        if !(self.state.include_empty && self.search.is_empty()) {
            let _ = candidates.next();
        }

        let list = List::new(candidates)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(colored!(LightYellow)),
            )
            .highlight_style(colored!(Yellow).bold())
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
    }

    fn draw_ranking(&self, frame: &mut Frame, area: Rect) {
        let mut items = self.candidates.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| {
            if a.points == b.points {
                a.first_votes.cmp(&b.first_votes).reverse()
            } else {
                a.points.cmp(&b.points).reverse()
            }
        });
        let items = items
            .iter()
            .enumerate()
            .map(|c| format!("{:2}: {:#?}", c.0 + 1, c.1));
        let list = List::new(items).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Ranking")
                .border_style(colored!(LightGreen)),
        );

        frame.render_widget(list, area);
    }
}

impl Screen for ElectionScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::horizontal([Constraint::Fill(1); 2])
            .vertical_margin(2)
            .horizontal_margin(6)
            .split(frame.area());
        self.draw_voting(frame, chunks[0]);
        self.draw_ranking(frame, chunks[1]);
    }

    fn handle_input(&mut self, event: Event) -> Action {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Action::Nothing;
            }
            match key.code {
                KeyCode::Char(c) => {
                    self.search.push(c);
                    self.update_filter();
                }
                KeyCode::Backspace => {
                    self.search.pop();
                    self.update_filter();
                }
                KeyCode::Down => self.list_state.select_next(),
                KeyCode::Up => self.list_state.select_previous(),
                KeyCode::Enter => self.select(),
                KeyCode::Esc => return Action::Exit(Ok(())),
                _ => {}
            };
        }
        Action::Nothing
    }
}

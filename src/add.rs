use std::mem;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, List, Paragraph},
    Frame,
};

use crate::{candidate::Candidate, Action, Screen};

macro_rules! colored {
    ($color:ident) => {
        ratatui::style::Style::default().fg(ratatui::style::Color::$color)
    };
}

#[derive(Default)]
pub struct AddScreen {
    name: String,
    candidates: Vec<Candidate>,
}

impl AddScreen {
    pub fn get_cadidates(self) -> Vec<Candidate> {
        self.candidates
    }
}

impl Screen for AddScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .vertical_margin(3)
            .horizontal_margin(10)
            .split(frame.area());

        let input = Paragraph::new(&*self.name).block(
            Block::bordered()
                .title(Line::from("Enter candidate name").centered())
                .border_type(BorderType::Rounded)
                .border_style(colored!(LightCyan))
                .title_style(Style::default()),
        );
        let input_area = chunks[0];
        frame.render_widget(input, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + u16::try_from(self.name.len()).unwrap_or(0) + 1,
            input_area.y + 1,
        ));

        let candidates = self.candidates.iter().map(|c| format!("{c:?}"));
        let list = List::new(candidates).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(colored!(Green))
                .title_bottom("Ctrl-Enter to procceed to election"),
        );
        frame.render_widget(list, chunks[1]);
    }

    fn handle_input(&mut self, event: Event) -> Action {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Action::Nothing;
            }
            match key.code {
                KeyCode::Char(c) => {
                    self.name.push(c);
                }
                KeyCode::Backspace => {
                    self.name.pop();
                }
                KeyCode::Enter => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return Action::Continue;
                    }
                    self.candidates
                        .push(Candidate::new(mem::replace(&mut self.name, String::new())));
                }
                KeyCode::Esc => return Action::Exit(Ok(())),
                _ => {}
            };
        }
        Action::Nothing
    }
}

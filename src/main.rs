use std::{error::Error, io};

use add::AddScreen;
use crossterm::event::{self, Event};
use election::ElectionScreen;
use ratatui::{self, prelude::Backend, Frame, Terminal};

mod add;
mod candidate;
mod election;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App {};
    Ok(run_app(&mut app)?)
}

fn run_app(app: &mut App) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_screen<B: Backend, S: Screen>(
    terminal: &mut Terminal<B>,
    screen: &mut S,
) -> Result<(), io::Result<()>> {
    loop {
        if let Err(err) = terminal.draw(|frame| screen.draw(frame)) {
            return Err(Err(err));
        };
        let event = match event::read() {
            Ok(event) => event,
            Err(err) => return Err(Err(err)),
        };
        let result = screen.handle_input(event);
        match result {
            Action::Nothing => {}
            Action::Exit(res) => return Err(res),
            Action::Continue => return Ok(()),
        }
    }
}

struct App {}

impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Result<()>> {
        let mut add_screen = AddScreen::default();
        run_screen(terminal, &mut add_screen)?;

        let candidates = add_screen.get_cadidates();

        let mut election = ElectionScreen::new(candidates);
        run_screen(terminal, &mut election)?;

        Ok(())
    }
}

enum Action {
    Nothing,
    Exit(io::Result<()>),
    Continue,
}

trait Screen {
    fn draw(&mut self, frame: &mut Frame);

    fn handle_input(&mut self, event: Event) -> Action;
}

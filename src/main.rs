use std::io;

use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{
		disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};
use tui::{
	backend::{Backend, CrosstermBackend},
	Frame, Terminal,
};

mod widgets;

fn main() -> Result<(), io::Error> {
	// enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
	enable_raw_mode()?;

	run_app(&mut terminal)?;

	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;

	Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
	loop {
		terminal.draw(ui)?;

		if let Event::Key(k) = event::read()? {
			match k.code {
				KeyCode::Char('q') => return Ok(()),

				_ => {}
			}
		}
	}
}

fn ui(f: &mut Frame<impl Backend>) {
	let mltb = widgets::MultiLineTextBuffer::new("abcdefg");
	f.render_widget(mltb, f.size());
}

mod habit;
mod pages;
use std::{io::{self, Stdout}, thread, time::Duration, path::Path, fs::File, error::Error, str::FromStr};

use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, Clear, ClearType, enable_raw_mode, EnterAlternateScreen}, execute, event::{DisableMouseCapture, Event, KeyEvent, KeyCode, self, EnableMouseCapture}};
use pages::Pages;
use rusqlite::Connection;
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders}, layout::{Layout, Constraint}, Frame};

use crate::habit::Habit;
mod prelude {
	pub use tui::widgets::*;
	pub use tui::{backend::Backend, Frame};
}

const FILENAME: &str = "habits.db";

pub struct GlobalState {
	conn: Connection,
	input_state : InputState,
	name: Option<String>,
	current_page: Pages
}

#[derive(PartialEq)]
pub enum InputState {
	None,
	Reading {content: String},
	Submitted {tag: String, content: String},
}

impl Default for GlobalState {
	fn default() -> GlobalState {
		GlobalState { conn: rusqlite::Connection::open("habits.db").unwrap(), input_state: InputState::None, name: None, current_page: Pages::Login}
	}
}

pub fn read_input(state: &mut GlobalState, tag: &str) -> io::Result<()> {
	match &mut state.input_state {
		InputState::Reading { content } => {
			if let Event::Key(key) = event::read()? {
				match key.code {
					KeyCode::Char(c) => {
						content.push(c);
					},
					KeyCode::Backspace => {
						content.pop();
					},
					KeyCode::Enter => {
						state.input_state = InputState::Submitted { tag: String::from_str(tag).unwrap(), content: content.clone() }
					}
					_ => {}
				}
			}
		},
		InputState::None => {
			state.input_state = InputState::Reading { content: String::new() }
		},
		_ => {}
	}
	Ok(())
}
fn main() -> Result<(), io::Error>{
	generate_database_file();
	let mut stdout = io::stdout();
	let mut terminal = initiate_terminal(&mut stdout)?;
	let mut state = GlobalState::default();
	loop {
		terminal.draw(|f| {
			match state.current_page {
				Pages::Login => {
					pages::login_page::login_page(f, &mut state);
				},
				Pages::Home => {
					pages::home_page::home_page(f, &mut state);
				}
				_ => {}
			}
		})?;
	}
	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;
  // read_input(&mut input);
	Ok(())

}

fn generate_database_file(){
	let path = Path::new(FILENAME);
	if path.exists(){
		return;
	}
	File::create(FILENAME).expect("Could not create database file");
}

fn initiate_terminal(stdout: &mut Stdout) -> Result<Terminal<CrosstermBackend<&mut io::Stdout>>, io::Error>{
	enable_raw_mode()?;
	execute!(io::stdout(), Clear(ClearType::All), DisableMouseCapture, EnterAlternateScreen,)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal: Terminal<CrosstermBackend<&mut io::Stdout>> = Terminal::new(backend)?;
	Ok(terminal)
}


// fn main() -> Result<(), io::Error> {
// 	let mut stdout = io::stdout();
// 	let backend = CrosstermBackend::new(&mut stdout);
// 	let mut terminal = Terminal::new(backend)?;
// 	execute!(io::stdout(), Clear(ClearType::All))?;
// 	terminal.draw(|f| {
// 		ui(f);
// 	})?;
// 	Ok(())
// }


// fn ui<B: Backend>(f: &mut Frame<B>) {
// 	let chunks = Layout::default()
// 		.direction(tui::layout::Direction::Vertical)
// 		.margin(5)
// 		.constraints([
// 			Constraint::Percentage(20),
// 			Constraint::Percentage(70),
// 			Constraint::Percentage(10),
// 		].as_ref())
// 		.split(f.size());
// 	let block = Block::default().title("lele").borders(Borders::ALL);
// 	f.render_widget(block, chunks[0]);
// 	let block = Block::default().title("Block 2").borders(Borders::ALL);
// 	f.render_widget(block, chunks[1]);
// 	let block = Block::default().title("Block 3").borders(Borders::ALL);
// 	f.render_widget(block, chunks[2]);
// }

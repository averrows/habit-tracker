use std::{io, thread, time::Duration};

use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, Clear, ClearType}, execute, event::DisableMouseCapture};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders}, layout::{Layout, Constraint}, Frame};



fn main() -> Result<(), io::Error> {
	let mut stdout = io::stdout();
	let backend = CrosstermBackend::new(&mut stdout);
	let mut terminal = Terminal::new(backend)?;
	execute!(io::stdout(), Clear(ClearType::All))?;
	terminal.draw(|f| {
		ui(f);
	})?;
	Ok(())
}


fn ui<B: Backend>(f: &mut Frame<B>) {
	let chunks = Layout::default()
		.direction(tui::layout::Direction::Vertical)
		.margin(5)
		.constraints([
			Constraint::Percentage(20),
			Constraint::Percentage(70),
			Constraint::Percentage(10),
		].as_ref())
		.split(f.size());
	let block = Block::default().title("lele").borders(Borders::ALL);
	f.render_widget(block, chunks[0]);
	let block = Block::default().title("Block 2").borders(Borders::ALL);
	f.render_widget(block, chunks[1]);
	let block = Block::default().title("Block 3").borders(Borders::ALL);
	f.render_widget(block, chunks[2]);
}
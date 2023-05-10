use tui::{backend::Backend, Frame, widgets::{Paragraph, Block, Borders}};

use crate::GlobalState;

pub fn home_page<B: Backend>(f: &mut Frame<B>, state: &mut GlobalState) {
  if let Some(name) = & state.name {
    let name_block = Paragraph::new(name.clone()).block(Block::default().borders(Borders::ALL).title("homepage"));
    f.render_widget(name_block, f.size())
  }
}
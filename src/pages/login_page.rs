use std::io::Result;

use crossterm::event::{Event, self, KeyCode};
use tui::{layout::{Constraint, Layout}, backend::Backend, Frame, widgets::{Block, Borders, Paragraph}, text::Text, style::{Style, Color}};

use crate::{GlobalState};
use unicode_width::UnicodeWidthStr;
use crate::*;


pub fn login_page<B: Backend>(f: &mut Frame<B>, state: &mut GlobalState) -> Result<()> {
  let divs = Layout::default()
    .direction(tui::layout::Direction::Vertical)
    .margin(2)
    .constraints([
      Constraint::Length(1),
      Constraint::Length(3),
      Constraint::Min(1)
    ].as_ref())
    .split(f.size());

  let question_text = Text::from("You haven't logged in yet...");
  let question = Paragraph::new(question_text);
  f.render_widget(question, divs[0]);
  
  read_input(state, "login_name")?;
  match &mut state.input_state {
    InputState::Reading { content } => {
      f.set_cursor(
        divs[1].x + content.width() as u16 + 1, 
        divs[1].y + 1);
      let answer = Paragraph::new(content.as_ref()).style(Style::default()
      .fg(Color::Yellow))
      .block(Block::default().borders(Borders::ALL).title("What is your name?"));
      f.render_widget(answer, divs[1]);
    },
    InputState::Submitted { tag , content} => {
      if tag != "login_name" {
        return Ok(());
      }
      state.name = Some(content.clone());
      state.current_page = Pages::Home;
    },
    _ => {}
  }

  Ok(())
}
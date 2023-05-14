use tui::{widgets::{Widget, Tabs, Block, Borders}, text::Spans, style::{Style, Color}, symbols::DOT};

use crate::read_input;

pub struct TopNavbar{}

impl Widget for TopNavbar {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
      let titles = ["Homepage", "Add Habit"].iter().cloned().map(Spans::from).collect();
      let menu = Tabs::new(titles)
          .block(Block::default().title("Menu").borders(Borders::ALL))
          .style(Style::default().fg(Color::White))
          .highlight_style(Style::default().fg(Color::Yellow))
          .select(1)
          .divider(DOT);
      menu.render(area, buf);
    }
}


use tui::{backend::Backend, Frame, widgets::{Paragraph, Block, Borders, ListItem, List}, text::{Text, Span, Spans}, style::{Style, Modifier}, layout::{Layout, Constraint}};

use crate::{GlobalState, habit::Habit};

pub fn home_page<B: Backend>(f: &mut Frame<B>, state: &mut GlobalState) {
  let divs = Layout::default().direction(tui::layout::Direction::Vertical)
    .margin(2)
    .constraints([
      Constraint::Length(3),
      Constraint::Min(2)
    ].as_ref()).split(f.size());
  if let Some(name) = & state.name {
    let name_block = Paragraph::new(format!("Hi {}",name.clone())).block(Block::default().borders(Borders::ALL).title("homepage"));
    f.render_widget(name_block, divs[0])
  }
  let habit_list = match Habit::get_all(&mut state.conn) {
    Ok(habits) => {
      habits.iter().map(|h| habit_list_item(h)).collect()
    },
    Err(_) => {vec![]}
  };
  let list = List::new(habit_list)
    .block(Block::default().title("Habits").borders(Borders::ALL));
  f.render_widget(list, divs[1]);
}

fn habit_list_item(habit: &Habit) -> ListItem<'static> {
  let coll = vec![
    Spans::from(vec![
      Span::styled(habit.name.clone(), Style::default().add_modifier(Modifier::BOLD)),
      Span::raw(": "),
      Span::raw(habit.description.clone())
    ])
  ];

  ListItem::new(coll)
}
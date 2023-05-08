use std::str::FromStr;

use chrono::{DateTime, Local};
use rusqlite::Connection;

#[derive(Debug)]
pub struct Habit {
    name: String,
    start: DateTime<Local>,
    description: String,
    duration: i32,
    is_good: bool,
    daily_status: Vec<bool>,
}

fn parse_daily_status(s: String) -> Vec<bool> {
  s.split(",").map(|s| s == "1").collect::<Vec<bool>>()
}

fn parse_date(s: String) -> DateTime<Local> {
  DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Local)
}

fn parse_bool(s: String) -> bool {
  s == "1"
}

impl Habit {
  pub fn new(name: &str, description: &str, duration: i32, is_good: bool) -> Habit {
    Habit {
      name: String::from_str(name).unwrap(),
      start: Local::now(),
      description: String::from_str(description).unwrap(),
      duration,
      is_good,
      daily_status: vec![false; 1],
    }
  }

  pub fn save(&self, conn: &mut Connection) -> Result<(),rusqlite::Error> {
    self.create_table(conn)?;
    let sql = "INSERT INTO habits (name, start, description, duration, is_good, daily_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
    conn.execute(sql, [
      self.name.clone(),
      self.start.to_rfc3339(),
      self.description.clone(),
      self.duration.to_string(),
      self.is_good.to_string(),
      self.daily_status.iter().map(|b| if *b { String::from_str("1").unwrap()} else { String::from_str("0").unwrap()}).collect::<Vec<String>>().join(","),
    ])?;
    Ok(())
  }

  pub fn get_all(conn: &mut Connection) -> Result<Vec<Habit>, rusqlite::Error> {
    let sql = "SELECT name, start, description, duration, is_good, daily_status FROM habits";
    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    let mut habits = Vec::new();
    while let Some(row) = rows.next()? {
      habits.push(Habit {
        name: row.get(0)?,
        start: parse_date(row.get(1)?),
        description: row.get(2)?,
        duration: row.get(3)?,
        is_good: parse_bool(row.get(4)?),
        daily_status: parse_daily_status(row.get(5)?),
      });
    }
    Ok(habits)
  }

  fn create_table(&self, conn: &mut Connection) -> Result<(), rusqlite::Error>{
    let sql = "CREATE TABLE IF NOT EXISTS habits (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      start TEXT NOT NULL,
      description TEXT NOT NULL,
      duration INTEGER NOT NULL,
      is_good INTEGER NOT NULL,
      daily_status TEXT NOT NULL
    )";
    conn.execute(sql, [])?;
    Ok(()) 
  }
}
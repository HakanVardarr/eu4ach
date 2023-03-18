use crossterm::cursor::MoveTo;
use crossterm::style::{
    Color,
    Color::{Black, Grey, Reset},
    Print, SetBackgroundColor, SetForegroundColor,
};

use crossterm::{execute, Result};
use std::io::stdout;

pub struct Categories<'a> {
    list: Option<Vec<Category<'a>>>,
    pos_tracker: u16,
}

impl<'a> Categories<'a> {
    pub fn new() -> Self {
        Self {
            list: None,
            pos_tracker: 0,
        }
    }
    pub fn draw(&mut self, col: u16) -> Result<()> {
        if let Some(mut categories) = self.list.clone() {
            for category in categories.iter_mut() {
                category.draw(col)?;
            }
        }

        Ok(())
    }
    pub fn add_category(&self, text: &'a str) -> Self {
        if let Some(mut categories) = self.list.clone() {
            categories.push(Category::new(self.pos_tracker, text));
            let number = self.pos_tracker;
            Self {
                list: Some(categories),
                pos_tracker: number + 1,
            }
        } else {
            Self {
                list: Some(vec![Category::new(self.pos_tracker, text)]),
                pos_tracker: 1,
            }
        }
    }
}

#[derive(Clone)]
struct Category<'a> {
    pos: u16,
    color: Color,
    text_color: Color,
    text: &'a str,
}

impl<'a> Category<'a> {
    fn new(pos: u16, text: &'a str) -> Category {
        Self {
            pos,
            color: Reset,
            text_color: Reset,
            text,
        }
    }
    fn draw(&mut self, col: u16) -> Result<()> {
        if col == self.pos {
            self.color = Grey;
            self.text_color = Black;
        } else {
            self.color = Reset;
            self.text_color = Reset;
        }

        execute!(
            stdout(),
            MoveTo(0, self.pos),
            SetBackgroundColor(self.color),
            SetForegroundColor(self.text_color),
            Print(self.text)
        )
    }
}

use crossterm::event::{read, Event};
use crossterm::{execute, Result};
use eu4ach::draw_achievementt;
use std::io::stdout;

struct Row<'a> {
    pos: u16,
    color: crossterm::style::Color,
    text_color: crossterm::style::Color,
    text: &'a str,
}

impl<'a> Row<'a> {
    fn draw(&mut self, col: u16) -> Result<()> {
        if col == self.pos {
            self.color = crossterm::style::Color::Grey;
            self.text_color = crossterm::style::Color::Black;
        } else {
            self.color = crossterm::style::Color::Reset;
            self.text_color = crossterm::style::Color::Reset;
        }

        execute!(
            stdout(),
            crossterm::cursor::MoveTo(0, self.pos),
            crossterm::style::SetBackgroundColor(self.color),
            crossterm::style::SetForegroundColor(self.text_color),
            crossterm::style::Print(self.text)
        )
    }
}

fn draw_achievement(text: &str) -> Result<()> {
    execute!(
        stdout(),
        crossterm::cursor::MoveTo(10, 0),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
        crossterm::style::SetForegroundColor(crossterm::style::Color::Reset),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
        crossterm::cursor::MoveTo(10, 1),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
        crossterm::cursor::MoveTo(10, 2),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
        crossterm::cursor::MoveTo(10, 0),
    )?;
    draw_achievementt(text)?;

    Ok(())
}

fn main() -> Result<()> {
    let mut row = 0;
    let mut col = 0;

    crossterm::terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorUp),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::cursor::Hide,
        crossterm::cursor::MoveTo(row, col),
    )?;

    let mut very_hard = Row {
        pos: 0,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "Very Hard",
    };

    let mut hard = Row {
        pos: 1,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "     Hard",
    };
    let mut medium = Row {
        pos: 2,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "   Medium",
    };
    let mut easy = Row {
        pos: 3,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "     Easy",
    };
    let mut very_easy = Row {
        pos: 4,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "Very Easy",
    };
    let mut random = Row {
        pos: 5,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "   Random",
    };
    let mut complete = Row {
        pos: 6,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: " Complete",
    };
    let mut track = Row {
        pos: 7,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "    Track",
    };
    let mut current = Row {
        pos: 8,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "  Current",
    };
    let mut clear = Row {
        pos: 9,
        color: crossterm::style::Color::Reset,
        text_color: crossterm::style::Color::Reset,
        text: "    Clear",
    };

    loop {
        very_hard.draw(col)?;
        hard.draw(col)?;
        medium.draw(col)?;
        easy.draw(col)?;
        very_easy.draw(col)?;
        random.draw(col)?;
        complete.draw(col)?;
        track.draw(col)?;
        current.draw(col)?;
        clear.draw(col)?;

        match read()? {
            Event::Key(event) => match (event.code, event.kind) {
                (crossterm::event::KeyCode::Esc, crossterm::event::KeyEventKind::Press) => {
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(0, 0),
                        crossterm::style::SetForegroundColor(crossterm::style::Color::Reset),
                        crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset),
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
                        crossterm::cursor::Show,
                    )?;

                    crossterm::terminal::disable_raw_mode()?;
                    break;
                }
                (crossterm::event::KeyCode::Right, crossterm::event::KeyEventKind::Press) => {
                    row += 1;

                    execute!(stdout(), crossterm::cursor::MoveTo(row, col))?;
                }
                (crossterm::event::KeyCode::Left, crossterm::event::KeyEventKind::Press) => {
                    if row > 0 {
                        row -= 1;
                    }

                    execute!(stdout(), crossterm::cursor::MoveTo(row, col))?;
                }
                (crossterm::event::KeyCode::Up, crossterm::event::KeyEventKind::Press) => {
                    if col > 0 {
                        col -= 1;
                    }

                    execute!(stdout(), crossterm::cursor::MoveTo(row, col))?;
                }
                (crossterm::event::KeyCode::Down, crossterm::event::KeyEventKind::Press) => {
                    if col < 9 {
                        col += 1;
                    }

                    execute!(stdout(), crossterm::cursor::MoveTo(row, col))?;
                }
                (crossterm::event::KeyCode::Enter, crossterm::event::KeyEventKind::Press) => {
                    match col {
                        0 => draw_achievement("Very Hard")?,
                        1 => draw_achievement("Hard")?,
                        2 => draw_achievement("Medium")?,
                        3 => draw_achievement("Easy")?,
                        4 => draw_achievement("Very Easy")?,
                        5 => draw_achievement("Random")?,
                        6 => draw_achievement("Complete")?,
                        7 => draw_achievement("Track")?,
                        8 => draw_achievement("Current")?,
                        9 => draw_achievement("Clear")?,
                        _ => (),
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    Ok(())
}

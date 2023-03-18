use crossterm::cursor::{EnableBlinking, Hide, MoveTo, Show};
use crossterm::event::KeyCode::{Down, Enter, Esc, Up};
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{read, Event, Event::Key, KeyCode::Backspace, KeyCode::Char};
use crossterm::style::{Color::Reset, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::size;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear,
    ClearType::{FromCursorDown, FromCursorUp, UntilNewLine},
};
use crossterm::{execute, Result};
use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, Write};
mod categories;
mod types;
pub use categories::Categories;
use types::{
    Achievement, EasyAchievements, HardAchievements, List, MediumAchievements,
    VeryEasyAchievements, VeryHardAchievements,
};

fn command_match(ty: &str) -> Result<()> {
    let mut file = File::open("achievements.json").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mut list: List = serde_json::from_str(&text)?;
    let mut count = 0;

    let (row, _) = size()?;

    macro_rules! achievement {
        ($($name:ident).+) => {

            'outer: loop {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..list.$($name).+.collection.len());

                if list.$($name).+.collection[i].is_complete == false {
                    execute!(stdout(), crossterm::style::Print(format!("{} ({})", list.$($name).+.collection[i].name.clone(),
                    list.$($name).+.collection[i].id,)))?;

                    let len = list.$($name).+.collection[i].description.len();

                    if len > row as usize{
                        let (half1, half2) =  list.$($name).+.collection[i].description.split_at(len / 2);
                        execute!(stdout(),crossterm::cursor::MoveTo(10, 1), crossterm::style::Print(half1))?;
                        execute!(stdout(),crossterm::cursor::MoveTo(10, 2), crossterm::style::Print(half2))?;
                    } else{
                        execute!(stdout(),crossterm::cursor::MoveTo(10, 1), crossterm::style::Print(list.$($name).+.collection[i].description.clone()))?;
                    }

                    break 'outer;
                } else {
                    if count == list.$($name).+.collection.len() {
                        println!("You have completed all achievements in that category");
                        break 'outer;
                    }

                    count += 1;
                    continue 'outer;
                }
            }
        };
    }
    macro_rules! command {
        ($command: ident) => {
            let mut id = String::new();
            let mut i = 0;
            execute!(stdout(), Print("ID: "), Show, EnableBlinking,)?;
            loop {
                match read()? {
                    Key(event) => match (event.code, event.kind) {
                        (Char(c), Press) => {
                            insert_char(c, &mut i, &mut id)?;
                        }
                        (Backspace, Press) => {
                            remove_char(&mut i, &mut id)?;
                        }
                        (Enter, Press) => {
                            clear_achievement()?;
                            if id.len() > 0 {
                                $command(list, id.parse::<usize>().unwrap());
                            }
                            break;
                        }
                        (Esc, Press) => {
                            clear_achievement()?;
                            break;
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        };
    }

    match ty.to_uppercase().as_str() {
        "EASY" => {
            achievement!(easy)
        }
        "HARD" => {
            achievement!(hard)
        }
        "MEDIUM" => {
            achievement!(medium)
        }
        "VERY HARD" => {
            achievement!(very_hard)
        }
        "VERY EASY" => {
            achievement!(very_easy);
        }
        "RANDOM" => {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..5);
            match i {
                0 => achievement!(very_easy),
                1 => achievement!(easy),
                2 => achievement!(medium),
                3 => achievement!(hard),
                4 => achievement!(very_hard),
                _ => (),
            };
        }
        "COMPLETE" => {
            command!(complete_achievement);
        }
        "TRACK" => {
            command!(track_achievement);
        }
        "CURRENT" => {
            if let Some(ach) = list.current {
                execute!(
                    stdout(),
                    crossterm::style::Print(format!("{} ({})", ach.name.clone(), ach.id,))
                )?;

                let len = ach.description.len();

                if len > row as usize {
                    let (half1, half2) = ach.description.split_at(len / 2);
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(10, 1),
                        crossterm::style::Print(half1)
                    )?;
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(10, 2),
                        crossterm::style::Print(half2)
                    )?;
                } else {
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(10, 1),
                        crossterm::style::Print(ach.description.clone())
                    )?;
                }
            } else {
                println!("You are not trackking an achievement right now");
            }
        }
        "CLEAR" => {
            list.current = None;
            let mut file = std::fs::File::create("achievements.json").unwrap();
            let text = serde_json::to_string_pretty(&list).unwrap();
            file.write_all(text.as_bytes()).unwrap();
        }
        _ => {}
    }

    Ok(())
}
fn insert_char(c: char, i: &mut u16, id: &mut String) -> Result<()> {
    let c = c.to_digit(10);
    if let Some(n) = c {
        id.push_str(n.to_string().as_str());
        execute!(stdout(), MoveTo(14 + *i, 0), Print(n.to_string()))?;
        *i += 1;
    }

    Ok(())
}
fn remove_char(i: &mut u16, id: &mut String) -> Result<()> {
    if *i > 0 {
        id.pop();
        *i -= 1;
        execute!(stdout(), MoveTo(14 + *i, 0), Print(" "), MoveTo(14 + *i, 0))?;
    }

    Ok(())
}
fn clear_achievement() -> Result<()> {
    execute!(
        stdout(),
        MoveTo(10, 0),
        SetBackgroundColor(Reset),
        SetForegroundColor(Reset),
        Clear(UntilNewLine),
        Hide,
    )
}
fn track_achievement(list: List, id: usize) {
    let found = false;
    let mut very_easy: Vec<Achievement> = vec![];
    let mut easy: Vec<Achievement> = vec![];
    let mut medium: Vec<Achievement> = vec![];
    let mut hard: Vec<Achievement> = vec![];
    let mut very_hard: Vec<Achievement> = vec![];
    let mut tracked_ach: Option<Achievement> = None;
    macro_rules! ach {
        ($($name:ident).+) => {
            if !found {
                for ach in list.$($name).+.collection {
                    let aid = ach.id;
                    if aid == id {
                        tracked_ach = Some(ach.clone());
                    }
                    $($name).+.push(ach);
                };
            }
        }
    }

    ach!(very_easy);
    ach!(easy);
    ach!(medium);
    ach!(hard);
    ach!(very_hard);

    if let Some(ach) = tracked_ach {
        let new_list = List {
            current: Some(ach),
            very_hard: VeryHardAchievements {
                collection: very_hard,
            },
            hard: HardAchievements { collection: hard },
            medium: MediumAchievements { collection: medium },
            easy: EasyAchievements { collection: easy },
            very_easy: VeryEasyAchievements {
                collection: very_easy,
            },
        };
        let mut file = std::fs::File::create("achievements.json").unwrap();
        let text = serde_json::to_string_pretty(&new_list).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }
}
fn complete_achievement(list: List, id: usize) {
    let found = false;
    let mut very_easy: Vec<Achievement> = vec![];
    let mut easy: Vec<Achievement> = vec![];
    let mut medium: Vec<Achievement> = vec![];
    let mut hard: Vec<Achievement> = vec![];
    let mut very_hard: Vec<Achievement> = vec![];

    macro_rules! ach {
        ($($name:ident).+) => {
            if !found {
                for mut ach in list.$($name).+.collection {
                    let aid = ach.id;
                    if aid == id {
                        ach.is_complete = true;
                    }
                    $($name).+.push(ach);
                };
            }
        }
    }

    ach!(very_easy);
    ach!(easy);
    ach!(medium);
    ach!(hard);
    ach!(very_hard);

    let new_list = List {
        current: list.current,
        very_hard: VeryHardAchievements {
            collection: very_hard,
        },
        hard: HardAchievements { collection: hard },
        medium: MediumAchievements { collection: medium },
        easy: EasyAchievements { collection: easy },
        very_easy: VeryEasyAchievements {
            collection: very_easy,
        },
    };

    let mut file = std::fs::File::create("achievements.json").unwrap();
    let text = serde_json::to_string_pretty(&new_list).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
pub fn run(mut categories: Categories) -> Result<()> {
    let mut col = 0;

    enable_raw_mode()?;
    execute!(
        stdout(),
        Clear(FromCursorUp),
        MoveTo(0, 0),
        Hide,
        MoveTo(0, col),
    )?;

    loop {
        categories.draw(col)?;

        match read()? {
            Event::Key(event) => match (event.code, event.kind) {
                (Esc, Press) => {
                    execute!(
                        stdout(),
                        MoveTo(0, 0),
                        SetForegroundColor(Reset),
                        SetBackgroundColor(Reset),
                        Clear(FromCursorDown),
                        Show,
                    )?;

                    disable_raw_mode()?;
                    break;
                }
                (Up, Press) => {
                    if col > 0 {
                        col -= 1;
                    }

                    execute!(stdout(), MoveTo(0, col))?;
                }
                (Down, Press) => {
                    if col < 9 {
                        col += 1;
                    }

                    execute!(stdout(), MoveTo(0, col))?;
                }
                (Enter, Press) => match col {
                    0 => command("Very Hard")?,
                    1 => command("Hard")?,
                    2 => command("Medium")?,
                    3 => command("Easy")?,
                    4 => command("Very Easy")?,
                    5 => command("Random")?,
                    6 => command("Complete")?,
                    7 => command("Track")?,
                    8 => command("Current")?,
                    9 => command("Clear")?,
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    Ok(())
}
fn command(text: &str) -> Result<()> {
    execute!(
        stdout(),
        MoveTo(10, 0),
        SetBackgroundColor(Reset),
        SetForegroundColor(Reset),
        Clear(UntilNewLine),
        MoveTo(10, 1),
        Clear(UntilNewLine),
        MoveTo(10, 2),
        Clear(UntilNewLine),
        MoveTo(10, 0),
    )?;
    command_match(text)?;

    Ok(())
}

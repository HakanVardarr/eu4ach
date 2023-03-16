use rand::Rng;
use std::io::{Read, Write};

mod types;
use types::{
    Achievement, EasyAchievements, HardAchievements, List, MediumAchievements,
    VeryEasyAchievements, VeryHardAchievements,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = std::fs::File::open("achievements.json")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let list: List = serde_json::from_str(&text)?;
    let mut count = 0;

    macro_rules! achievement {
        ($($name:ident).+) => {

            'outer: loop {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..list.$($name).+.collection.len());

                if list.$($name).+.collection[i].is_complete == false {
                    println!(
                        "{} ({}): {}",
                        list.$($name).+.collection[i].name.clone(),
                        list.$($name).+.collection[i].id,
                        list.$($name).+.collection[i].description.clone()
                    );
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

    if args.len() < 2 {
        println!("USAGE:");
        println!("     eu4ach random: random achievement");
        println!("     eu4ach veasy: random very easy achievement");
        println!("     eu4ach easy: random  easy achievement");
        println!("     eu4ach medium: random  medium achievement");
        println!("     eu4ach hard: random hard achievement");
        println!("     eu4ach very_hard: random very hard achievement");
        println!("     eu4ach complete <id : number>: complete achievement");

        std::process::exit(0);
    }

    let arg = &args[1];
    match arg.to_uppercase().as_str() {
        "EASY" => {
            achievement!(easy)
        }
        "HARD" => {
            achievement!(hard)
        }
        "MEDIUM" => {
            achievement!(medium)
        }
        "VHARD" => {
            achievement!(very_hard)
        }
        "VEASY" => {
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
            let id = &args[2].trim().parse::<usize>().unwrap();
            complete_achievement(list, *id);
        }

        _ => {}
    }

    Ok(())
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

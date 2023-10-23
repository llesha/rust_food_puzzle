mod food;
use crate::food::{Food, FoodGroup, Taste};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};
use std::time::SystemTime;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    let mut history = Vec::<String>::new();
    loop {
        let input = prompt("> ");
        history.push(input.clone());
        match &input[..] {
            "now" => {
                let unixtime = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                print!("Current Unix time is {:?}\n", unixtime);
            }
            "help" => {}
            "\u{1b}[A" | "history" => {
                history.pop();
                history.push(input);
                println!("{:#?}", history);
            }
            "exit" | "quit" => break,
            other => {
                //  history.pop();
                println!("Unknown command: {}", other)
            }
        }
    }
    let map = read_file("food.txt");
    println!("{:#?}", map);
}

fn read_file(path: &str) -> io::Result<HashMap<String, FoodGroup>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut fg = FoodGroup { foods: Vec::new() };
    let mut name: String = String::from("");

    for l in reader.lines() {
        let line = l?.to_string();
        let parts: Vec<&str> = line.split(";").collect();
        if parts.len() == 1 {
            if parts[0].len() > 1 {
                name = line.clone();
                map.insert(line, fg);
                fg = FoodGroup { foods: Vec::new() };
            }
        } else {
            let taste_parts = parts[2].replace("{", "").replace("}", "");
            let taste: Vec<&str> = taste_parts.split(",").collect();
            let group = map.get_mut(&mut name).unwrap();
            group.foods.push(Food {
                name: parts[0].into(),
                emoji: parts[1].into(),
                taste: Taste {
                    umami: taste[0].parse::<i8>().unwrap(),
                    sour: taste[1].parse::<i8>().unwrap(),
                    spicy: taste[2].parse::<i8>().unwrap(),
                    sweet: taste[3].parse::<i8>().unwrap(),
                },
            });
        }
    }

    Ok(map)
}

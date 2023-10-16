use crate::utilities::{get_parsed_user_input, INPUT_READ_ERROR};
use log::info;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Player {
    health: i32,
    level: u32,
    strength: i32,
    agility: i32,
    intelligence: i32,
}

impl Player {
    fn level_up(&mut self, attribute: &Attribute) {
        self.level = self.level + 1;
        match attribute {
            Attribute::Strength => self.strength = self.strength + 1,
            Attribute::Agility => self.agility = self.agility + 1,
            Attribute::Intelligence => self.intelligence = self.intelligence + 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Attribute {
    Strength,
    Agility,
    Intelligence,
}

impl FromStr for Attribute {
    type Err = ();

    fn from_str(attribute: &str) -> Result<Attribute, Self::Err> {
        match attribute {
            "str" => Ok(Attribute::Strength),
            "strength" => Ok(Attribute::Strength),
            "agi" => Ok(Attribute::Agility),
            "agility" => Ok(Attribute::Agility),
            "int" => Ok(Attribute::Intelligence),
            "intelligence" => Ok(Attribute::Intelligence),
            _ => Err(()),
        }
    }
}

struct Enemy {
    health: i32,
    damage: i32,
}
pub fn run_rpg_game() {
    let player: Player = create_character();
}

//def start_stats(func):
//     while True:
//         stat = input("Choose your " + str(func()) + ". attribute to increase.")
//         global vit
//         global agi
//         global wis
//         global n
//         if stat == "str":
//             vit += 1
//             print("One point was added to strength. \n")
//             return vit
//         elif stat == "agi":
//             agi += 1
//             print("One point was added to agility. \n")
//             return agi
//         elif stat == "int":
//             wis += 1
//             print("One point was added to intelligence. \n")
//             return wis
//         else:
//             print("Uups, something went wrong. Try again.")
//             n -= 1

fn create_character() -> Player {
    let mut player = Player {
        health: 100,
        level: 1,
        strength: 1,
        agility: 1,
        intelligence: 1,
    };

    for _ in 0..5 {
        level_up_player(&mut player);
    }

    info!("Created player: {player:?}");
    player
}

fn level_up_player(player: &mut Player) {
    println!("Select attribute to level up:");
    let mut attribute: String = String::new();
    io::stdin()
        .read_line(&mut attribute)
        .expect(INPUT_READ_ERROR);

    let attribute: Attribute = match attribute.trim().parse() {
        Ok(attribute) => attribute,
        Err(_) => {
            println!("Not a correct attribute!");
            panic!()
        }
    };

    player.level_up(&attribute);
    info!("Leveled up {:?}", &attribute)
}

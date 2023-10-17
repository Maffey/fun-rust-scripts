use crate::utilities::{get_parsed_user_input, INPUT_READ_ERROR};
use log::info;
use std::str::FromStr;
use std::{fmt, io};

#[derive(Debug)]
struct Character {
    health: i32,
    level: u32,
    strength: i32,
    agility: i32,
    intelligence: i32,
}

impl Character {
    fn new() -> Character {
        Character {
            health: 100,
            level: 1,
            strength: 1,
            agility: 1,
            intelligence: 1,
        }
    }

    fn level_up(&mut self, attribute: &Attribute) {
        self.level = self.level + 1;
        match attribute {
            Attribute::Strength => self.strength = self.strength + 1,
            Attribute::Agility => self.agility = self.agility + 1,
            Attribute::Intelligence => self.intelligence = self.intelligence + 1,
        }
    }
}

impl fmt::Display for Character {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Character details:\n\t\
               HP: {}\n\t\
               LVL: {}\n\t\
               Strength: {}\n\t\
               Agility: {}\n\t\
               Intelligence: {}",
            self.health, self.level, self.strength, self.agility, self.intelligence
        )
    }
}

struct AttributeModifier {
    strength: i32,
    agility: i32,
    intelligence: i32,
}

impl AttributeModifier {
    fn default() -> AttributeModifier {
        AttributeModifier {
            strength: 1,
            agility: 1,
            intelligence: 1,
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

pub fn run_rpg_game() {
    let player: Character = create_player_character();
}

fn create_player_character() -> Character {
    let mut player = Character::new();

    for _ in 0..5 {
        level_up_player(&mut player);
    }

    info!("Created player: {player:?}");
    println!("{player}");
    player
}

fn level_up_player(player: &mut Character) {
    loop {
        println!("Select attribute to level up:");
        let mut attribute: String = String::new();
        io::stdin()
            .read_line(&mut attribute)
            .expect(INPUT_READ_ERROR);

        let attribute: Attribute = match attribute.trim().parse() {
            Ok(attribute) => attribute,
            Err(_) => {
                println!("Not a correct attribute! Try again.");
                continue;
            }
        };

        player.level_up(&attribute);
        info!("Leveled up {:?}", &attribute);
        break;
    }
}

fn fight(player: &mut Character, enemy: &mut Character, attributes_modifier: &AttributeModifier) {
    //     def fight(a, b, c):
    //     while True:
    //         option = input("What you want to do? Type 'a', 'b' or 'c'.")
    //         if option == "a":
    //             enemy_hp -= (vit * 10 * a)
    //             print("Your hit causes the enemy's health to drop to " + str(enemy_hp) + " points!")
    //             return enemy_hp
    //
    //         elif option == "b":
    //             enemy_hp -= (agi * 10 * b)
    //             print("Your hit causes the enemy's health to drop to " + str(enemy_hp) + " points!")
    //             return enemy_hp
    //
    //         elif option == "c":
    //             enemy_hp -= (wis * 10 * c)
    //             print("Your hit causes the enemy's health to drop to " + str(enemy_hp) + " points!")
    //             return enemy_hp
    //
    //         else:
    //             print("Uups, something went wrong. Try again.")
    loop {
        println!("What do you want to do? Type 'a', 'b' or 'c'.");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect(INPUT_READ_ERROR);

        match choice.as_str() {
            // TODO implement this
            "a" => choice,
            _ => {
                println!("Not a correct choice! Try again.");
                continue;
            }
        };
    }
}

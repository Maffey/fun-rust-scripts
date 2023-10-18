use crate::utilities::INPUT_READ_ERROR;
use log::info;
use std::str::FromStr;
use std::{fmt, io};

const STARTING_PLAYER_LEVEL: u32 = 3;

#[derive(Debug)]
struct Character {
    health: f32,
    level: u32,
    strength: i32,
    agility: i32,
    intelligence: i32,
}

impl Character {
    fn new() -> Character {
        Character {
            health: 100.0,
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

    fn deal_damage(&mut self, damage: f32) {
        self.health = self.health - damage;
        println!("{damage} was dealt. Character is at {} HP.", self.health);
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
    strength: f32,
    agility: f32,
    intelligence: f32,
}

impl AttributeModifier {
    fn default() -> AttributeModifier {
        AttributeModifier {
            strength: 1.0,
            agility: 1.0,
            intelligence: 1.0,
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
    // TODO this for testing for now, add proper flow later.
    let mut player: Character = create_player_character();
    let mut enemy: Character = Character::new();
    let simple_modifier = AttributeModifier::default();
    enemy.health = 50.0;

    attack_enemy(&mut player, &mut enemy, &simple_modifier);
    info!("{player:?}");
    info!("{enemy:?}");
}

fn create_player_character() -> Character {
    let mut player = Character::new();

    for _ in 0..STARTING_PLAYER_LEVEL {
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

fn attack_enemy(
    player: &mut Character,
    enemy: &mut Character,
    attributes_modifier: &AttributeModifier,
) {
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

        let damage_dealt: f32;

        match choice.trim() {
            "a" => {
                damage_dealt = (player.strength / enemy.strength) as f32
                    * (player.level / enemy.level) as f32
                    * attributes_modifier.strength
            }
            "b" => {
                damage_dealt = (player.agility / enemy.agility) as f32
                    * (player.level / enemy.level) as f32
                    * attributes_modifier.agility
            }
            "c" => {
                damage_dealt = (player.intelligence / enemy.intelligence) as f32
                    * (player.level / enemy.level) as f32
                    * attributes_modifier.intelligence
            }
            _ => {
                println!("Not a correct choice! Try again.");
                continue;
            }
        };

        enemy.deal_damage(damage_dealt);
        break;
    }
}

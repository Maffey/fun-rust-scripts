use crate::utilities::INPUT_READ_ERROR;
use log::info;
use rand::Rng;
use std::ops::Range;
use std::process::exit;
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
        println!(
            "{damage:.2} damage was dealt. Character is at {:.2} HP.",
            self.health
        );
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
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

fn player_attack(
    player: &Character,
    enemy: &mut Character,
    attributes_modifier: &AttributeModifier,
) {
    loop {
        println!("What do you want to do? Type 'a', 'b' or 'c'.");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect(INPUT_READ_ERROR);

        let damage: f32;

        match choice.trim() {
            "a" => {
                damage = (player.strength as f32 / enemy.strength as f32)
                    * (player.level as f32 / enemy.level as f32)
                    * attributes_modifier.strength
            }
            "b" => {
                damage = (player.agility as f32 / enemy.agility as f32)
                    * (player.level as f32 / enemy.level as f32)
                    * attributes_modifier.agility
            }
            "c" => {
                damage = (player.intelligence as f32 / enemy.intelligence as f32)
                    * (player.level as f32 / enemy.level as f32)
                    * attributes_modifier.intelligence
            }
            _ => {
                println!("Not a correct choice! Try again.");
                continue;
            }
        };

        enemy.deal_damage(damage);
        break;
    }
}

fn simple_enemy_attack(enemy: &Character, player: &mut Character, damage_range: Range<f32>) {
    println!("The enemy attacks you!");
    let mut rng = rand::thread_rng();
    let damage: f32 = rng.gen_range(damage_range) * (enemy.level as f32 / player.level as f32);
    player.deal_damage(damage);
}

fn fight(
    player: &mut Character,
    enemy: &mut Character,
    attribute_modifier: &AttributeModifier,
    enemy_damage_range: Range<f32>,
) {
    loop {
        player_attack(player, enemy, &attribute_modifier);
        if enemy.is_dead() {
            println!("The enemy has been defeated. You've won the fight and got healed!");
            player.health = 100.0;
            break;
        }
        simple_enemy_attack(&enemy, player, enemy_damage_range.clone());
        if player.is_dead() {
            println!("You've died!");
            exit(0);
        }
    }
}

pub fn run_rpg_game() {
    println!("\nWelcome, young warrior.\nYour mission is to defeat the enemies each day. \
    You will do that by selecting the right choices. The attributes will also help you in fights. \
    \nGood luck!\nYou have 100 hp, 1 strength, 1 agility and 1 intelligence point in the beginning. \
    You will be able to increase those attributes later in the game.\n \
    Let's start with selecting few attributes. By typing down 'str', 'agi' or 'int' and pressing ENTER. \
    \n\nYou get {STARTING_PLAYER_LEVEL} points to spend for now.");
    let mut player: Character = create_player_character();
    println!("Now, after you created your character, it's time to begin the journey!\n\n
    Day 1\nYou've just entered local tavern to find out about a fighting tournament that should start soon.
     When you are asking people in the tavern about this, you get to meet some drunk asshole, who seems pretty stupid.
     The guys is aggressive, even though you try to avoid the conflict.
     You decide to test your skills on him, to warm up the muscles before great tournament.\n
    What do you do?\n\
    a) You hit him in the head with great amount of force.\n\
    b)You attack rapidly from below.\n\
    c) You take a bottle lying nearby and hit him with it in the head.\n");

    let mut enemy: Character = Character::new();
    let first_fight_modifier = AttributeModifier {
        strength: 1.5,
        agility: 1.7,
        intelligence: 1.9,
    };
    enemy.health = 50.0;
    fight(&mut player, &mut enemy, &first_fight_modifier, 10.0..20.0);
    println!("Your attributes were increased!\n");
    level_up_player(&mut player);
    println!("Player: {player}\nEnemy: {enemy}")
}

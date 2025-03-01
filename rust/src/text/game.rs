use core::time;
use std::{
    fs::File,
    io::{self, Write},
    str::FromStr,
    thread::sleep,
};

use crate::models::{
    data::ABILITIES, gate::Gate, monster::Monster, player::Player, shadow::Shadow,
};

const DEFAULT_ARISE_ATTEMPTS: i32 = 3;

pub struct Game<'a> {
    player: Player,
    file_path: &'a str,
}

impl<'a> Game<'a> {
    pub fn new(file_path: &'a str) -> Game<'a> {
        Game {
            player: Player::new(None),
            file_path,
        }
    }

    pub fn load(file_path: &'a str) -> io::Result<Game<'a>> {
        let file = File::open(file_path)?;

        Ok(Game {
            player: serde_json::from_reader(file)?,
            file_path,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.player.is_defeated() {
            print_header("Welcome, Shadow Monarch! Prepare for battle.");
            println!();
            print_menu();
            println!();
            print!("Please enter your choice: ");
            io::stdout().flush()?;

            match get_user_input()?.as_str() {
                "1" => {
                    self.encounter_dungeon()?;
                }
                "2" => {
                    print_shadows(&self.player.shadows)?;
                    println!();
                }
                "3" => break,
                _ => {
                    println!("Invalid input!")
                }
            }
            self.save_state()?;
        }

        print_exit_sequence(&self.player.shadows)
    }

    fn encounter_dungeon(&mut self) -> io::Result<()> {
        let mut gate = Gate::generate_random(&mut self.player.rng);

        print_header(format!("You encounter a {}!", gate.dungeon.name).as_str());

        loop {
            println!();
            print!("Do you want to enter the gate? (y/n): ");
            io::stdout().flush()?;

            match get_user_input()?.to_lowercase().as_str() {
                "n" => {
                    let l = gate.monsters.len();
                    let m = match l {
                        1 => "monster",
                        _ => "monsters",
                    };
                    println!("{} {} detected...", l, m);
                    break;
                }
                "y" => {
                    for monster in gate.monsters.iter_mut() {
                        self.fight_monster(monster)?;

                        if self.player.is_defeated() {
                            print_header(
                                "You have fallen in battle... The shadows claim your soul.",
                            );
                        } else if monster.is_defeated() {
                        }
                    }
                    break;
                }
                _ => {
                    println!("Invalid input!");
                }
            }
        }
        return Ok(());
    }

    fn fight_monster(&mut self, monster: &mut Monster) -> io::Result<()> {
        print_header(format!(
            "A {} (Power: {}) appears!",
            monster.name, monster.power
        ));

        while !self.player.is_defeated() && !monster.is_defeated() {
            print_abilities();
            println!();
            print!("Choose ability number: ");
            io::stdout().flush()?;

            let input = usize::from_str(get_user_input()?.as_str());
            if input.is_err() {
                println!("Invalid input!");
                continue;
            }

            let input = input.unwrap() - 1;

            if input > ABILITIES.len() - 1 {
                println!("Invalid input!");
                continue;
            }

            let ability = &ABILITIES[input];

            let damage = self.player.attack(monster, ability);

            println!();
            println!(">> You used {}!", ability.name);
            println!(" {} took {} damage.", monster.name, damage);
            println!(" {} Remaining HP: {}", monster.name, monster.health);
            println!();
            if monster.is_defeated() {
                self.attempt_arise(monster)?;
                break;
            }
            println!();

            let damage = monster.power * 5;
            self.player.health -= damage;

            println!(">> {} attacks!", monster.name);
            println!(" You took {} damage.", damage);
            println!(" Your HP: {}", self.player.health);
            println!();
        }

        Ok(())
    }

    fn attempt_arise(&mut self, monster: &mut Monster) -> io::Result<()> {
        print_header(format!("Arise Attempt on {}", monster.name));

        for i in (0..DEFAULT_ARISE_ATTEMPTS).rev() {
            loop {
                print!("Do you want to use Arise? (y/n): ");
                io::stdout().flush()?;

                match get_user_input()?.to_lowercase().as_str() {
                    "y" => {
                        let shadow = self.player.attempt_arise(monster);

                        if shadow.is_some() {
                            let shadow = shadow.unwrap();
                            print_header(format!(
                                "{} has become a shadow minion of rank {}!",
                                shadow.name, shadow.rank
                            ));
                            return Ok(());
                        } else {
                            println!(">> {} resisted Arise! Attempts left: {}", monster.name, i);
                        }
                        break;
                    }
                    "n" => {
                        println!(">> Arise attempt skipped.");
                        println!();
                        return Ok(());
                    }
                    _ => {
                        println!("Invalid input!");
                        println!();
                    }
                }
            }
        }

        println!(">> {} resisted all attempts!", monster.name);
        println!();
        Ok(())
    }

    pub fn save_state(&self) -> io::Result<()> {
        let file = File::create(self.file_path)?;
        serde_json::to_writer_pretty(file, &self.player)?;
        Ok(())
    }
}

fn print_header(text: impl AsRef<str>) {
    println!("\n{}", "=".repeat(50));
    println!("{:^50}", text.as_ref());
    println!("{}", "=".repeat(50));
}

const MENU_ITEMS: [&'static str; 3] = ["Find a Dungeon", "View Minions", "Quit"];

fn print_menu() {
    println!("--- Main Menu ---");
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }
    println!("{}", "-".repeat(17));
}

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();
    Ok(input)
}

fn print_creepy_pause() -> io::Result<()> {
    for _ in 0..3 {
        sleep(time::Duration::from_millis(500));
        print!(".");
        io::stdout().flush()?;
    }
    sleep(time::Duration::from_millis(500));
    println!();

    Ok(())
}

fn print_exit_sequence(shadows: &Vec<Shadow>) -> io::Result<()> {
    print_header("See you again ShAdOw MoNaRcH~");
    println!();
    print_creepy_pause()?;
    println!();
    println!("The shadows stir, your Shadow Army silently awaits your return from the abyss.");
    println!();
    print_creepy_pause()?;
    println!();
    print_shadows(shadows)?;
    Ok(())
}

fn print_shadows(shadows: &Vec<Shadow>) -> io::Result<()> {
    print_header("Your Shadow Minions");

    if shadows.len() == 0 {
        println!("You have no shadow minions.");
        return Ok(());
    }

    for m in shadows.iter() {
        println!("- {} (Rank: {}, Power: {})", m.name, m.rank, m.power);
    }

    Ok(())
}

fn print_abilities() {
    println!("Your Abilities:");

    for (i, ability) in ABILITIES.iter().enumerate() {
        println!("{}. {}", i + 1, ability.name);
    }
}

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// Struct to represent each habit
#[derive(Debug)] // Derive the debug trait for the Habit struct
struct Habit {
    name: String,
    completed_today: bool,
    streak: u32,
}

impl Habit {
    fn new(name: &str) -> Habit {
        Habit {
            name: name.to_string(),
            completed_today: false,
            streak: 0,
        }
    }
}

// Struct to manage habit tracking
#[derive(Debug)]
struct HabitTracker {
    habits: Vec<Habit>,
}

impl HabitTracker {
    fn new() -> HabitTracker {
        HabitTracker { habits: Vec::new() }
    }

    fn add_habit(&mut self, name: &str) {
        let habit = Habit::new(name);
        self.habits.push(habit);
    }

    fn remove_habit(&mut self, name: &str) {
        if let Some(pos) = self.habits.iter_mut().position(|h| h.name == name) {
            self.habits.remove(pos);
            println!("Habit removed");

            self.save_to_file();
        } else {
            println!("Unable to remove habit");
        }
    }

    fn mark_complete(&mut self, name: &str) {
        if let Some(habit) = self.habits.iter_mut().find(|h| h.name == name) {
            habit.completed_today = true;
            habit.streak += 1;
        }
    }

    fn mark_incomplete(&mut self, name: &str) {
        if let Some(habit) = self.habits.iter_mut().find(|h| h.name == name) {
            habit.completed_today = false;
            habit.streak = 0;
        }
    }

    fn display(&self) {
        for habit in &self.habits {
            println!("{:?}: Streak: {} days", habit.name, habit.streak);
        }
    }

    fn save_to_file(&self) {
        let path = Path::new("habits.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("Unable to open file...");

        for habit in &self.habits {
            writeln!(
                file,
                "{}:{}:{}",
                habit.name, habit.completed_today, habit.streak
            )
            .expect("Failed to write to file!");
        }
    }

    fn load_from_file(&mut self) {
        let path = Path::new("habits.txt");
        if !path.exists() {
            return;
        }

        let mut file = File::open(path).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");

        for line in content.lines() {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() == 3 {
                let habit = Habit {
                    name: parts[0].to_string(),
                    completed_today: parts[1] == "true",
                    streak: parts[2].parse().expect("Invalid streak value"),
                };
                self.habits.push(habit);
            }
        }
    }
}

fn main() {
    let mut tracker = HabitTracker::new();

    // Load habits
    tracker.load_from_file();

    loop {
        println!("Habit Tracker:");
        println!("1. Add a new habit");
        println!("2. Mark a habit as completed");
        println!("3. Mark a habit as incomplete");
        println!("4. View all habits");
        println!("5. Remove a habit");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Invalid input");
        match choice {
            1 => {
                println!("Enter habit to add: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Unable to read habit name");
                tracker.add_habit(name.trim());
            }
            2 => {
                println!("Habit to mark complete: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Unable to read habit name");
                tracker.mark_complete(name.trim());
            }
            3 => {
                println!("Enter habit to mark incomplete: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Unable to read habit name");
                tracker.mark_incomplete(name.trim());
            }
            4 => {
                tracker.display();
            }
            5 => {
                println!("Enter habit to remove: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Unable to read habit name");
                tracker.remove_habit(name.trim());
            }
            6 => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid option!");
            }
        }
        tracker.save_to_file();
    }
}

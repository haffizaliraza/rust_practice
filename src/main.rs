use std::collections::VecDeque;
use std::io::{self, Write};

// Enum to represent the state of the lift
#[derive(Debug, PartialEq)]
enum LiftState {
    MovingUp,
    MovingDown,
    Idle,
    DoorsOpen,
    DoorsClosed,
}

// Struct to represent the lift
struct Lift {
    current_floor: i32,
    total_floors: i32,
    state: LiftState,
    request_queue: VecDeque<i32>,
}

impl Lift {
    // Constructor to initialize the lift
    fn new(total_floors: i32) -> Self {
        Lift {
            current_floor: 0, // Ground floor
            total_floors,
            state: LiftState::Idle,
            request_queue: VecDeque::new(),
        }
    }

    // Method to request the lift to a specific floor
    fn request_floor(&mut self, floor: i32) {
        if floor < 0 || floor > self.total_floors {
            println!("Invalid floor request. Floor {} does not exist.", floor);
        } else if !self.request_queue.contains(&floor) {
            self.request_queue.push_back(floor);
            println!("Floor {} has been added to the request queue.", floor);
        } else {
            println!("Floor {} is already in the request queue.", floor);
        }
    }

    // Method to open the lift doors
    fn open_doors(&mut self) {
        self.state = LiftState::DoorsOpen;
        println!("Lift doors are open.");
    }

    // Method to close the lift doors
    fn close_doors(&mut self) {
        self.state = LiftState::DoorsClosed;
        println!("Lift doors are closed.");
    }

    // Method to move the lift to the next floor in the request queue
    fn move_to_next_floor(&mut self) {
        if let Some(next_floor) = self.request_queue.pop_front() {
            if next_floor > self.current_floor {
                self.state = LiftState::MovingUp;
                println!("Lift is moving up to floor {}.", next_floor);
                self.current_floor = next_floor;
            } else if next_floor < self.current_floor {
                self.state = LiftState::MovingDown;
                println!("Lift is moving down to floor {}.", next_floor);
                self.current_floor = next_floor;
            }

            self.open_doors();
            println!("Lift has reached floor {}.", self.current_floor);
            self.close_doors();

            self.state = LiftState::Idle;
        } else {
            println!("No more requests in the queue. Lift is idle.");
            self.state = LiftState::Idle;
        }
    }

    // Method to get the current status of the lift
    fn status(&self) {
        println!(
            "Lift status -> Current Floor: {}, State: {:?}, Request Queue: {:?}",
            self.current_floor, self.state, self.request_queue
        );
    }
}

// Function to display the menu and get user input
fn display_menu() {
    println!("\n=== Lift Management System ===");
    println!("1. Request Floor");
    println!("2. Move Lift to Next Floor");
    println!("3. View Lift Status");
    println!("4. Exit");
    print!("Please enter your choice: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
}

// Main function to handle the user interface and interact with the lift
fn main() {
    let mut my_lift = Lift::new(10); // Lift with 10 floors

    loop {
        display_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter the floor number to request: ");
                io::stdout().flush().unwrap();
                let mut floor_input = String::new();
                io::stdin().read_line(&mut floor_input).expect("Failed to read input");
                let floor: i32 = match floor_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid floor number. Please enter a valid integer.");
                        continue;
                    }
                };
                my_lift.request_floor(floor);
            }
            2 => my_lift.move_to_next_floor(),
            3 => my_lift.status(),
            4 => {
                println!("Exiting the Lift Management System. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a number between 1 and 4."),
        }
    }
}
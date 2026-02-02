use std::io::{self, Write};

mod tasks;

fn main() {
    loop {
        println!("\n=== Select a task to run ===");
        println!("1. Control Flow");
        println!("2. Deserialize Raw String");
        println!("3. Ownership & Borrowing");
        println!("4. Serialize / Deserialize");
        println!("5. Mutex Request Tracker");
        println!("6. RwLock Request Tracker");
        println!("0. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => tasks::control_flow::start(),
            "2" => tasks::deserialise_raw_string::start(),
            "3" => tasks::ownership_borrow::start(),
            "4" => tasks::seri_deseri::start(),
            "5" => tasks::mutex_req_tracker::start(),
            "6" => tasks::rw_lock_req_tracker::start(),
            "0" => {
                println!("👋 Exiting. Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice, try again."),
        }
    }
}

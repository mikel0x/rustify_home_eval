mod property;
mod utils;

use property::House;
use utils::input;

fn main() {
    let mut houses: Vec<House> = Vec::new();

    loop {
        println!("\nProperty Manager Menu:");
        println!("1. Add new house");
        println!("2. List all houses");
        println!("3. Evaluate all houses");
        println!("4. Filter houses (bedrooms >= 3)");
        println!("5. Delete house by index");
        println!("0. Exit");

        let choice = input("\nEnter your choice: ");

        match choice.trim() {
            "1" => {
                let house = House::new_from_input();
                houses.push(house);
                println!(" House added successfully!");
            }
            "2" => {
                if houses.is_empty() {
                    println!(" No houses available.");
                } else {
                    for (i, h) in houses.iter().enumerate() {
                        println!("\n[{}] {:#?}", i, h);
                        println!("   Bathrooms: {}", h.bathrooms);
                        println!("   House Type: {}", h.describe_type());
                    }
                }
            }
            "3" => {
                for h in &houses {
                    println!("\nEvaluating house: {:#?}", h);
                    if h.is_spacious() {
                        println!("✔ Spacious");
                    } else {
                        println!(" Cozy");
                    }
                    if h.has_garden() {
                        println!(" Has garden");
                    } else {
                        println!(" No garden");
                    }
                    println!("Type: {}", h.describe_type());
                }
            }
            "4" => {
                let filtered: Vec<&House> = houses.iter().filter(|h| h.bedrooms >= 3).collect();
                if filtered.is_empty() {
                    println!("🔍 No houses found with 3 or more bedrooms.");
                } else {
                    println!(" Houses with >= 3 bedrooms:");
                    for h in filtered {
                        println!("{:#?}", h);
                        println!("   Type: {}", h.describe_type());
                    }
                }
            }
            "5" => {
                let idx_input = input("Enter index to remove: ");
                if let Ok(idx) = idx_input.trim().parse::<usize>() {
                    if idx < houses.len() {
                        houses.remove(idx);
                        println!(" House removed.");
                    } else {
                        println!(" Invalid index.");
                    }
                } else {
                    println!(" Please enter a valid number.");
                }
            }
            "0" => {
                println!(" Exiting. See you soon!");
                break;
            }
            _ => println!(" Invalid option. Try again."),
        }
    }
}
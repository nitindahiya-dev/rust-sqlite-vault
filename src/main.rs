use std::io;
use std::io::Write;

mod db;
use db::{init_database, read_passwords_from_db, search_service_by_name, update_entry, delete_from_db};

fn main() {
    let conn = init_database().expect("Failed to initialize the database.");

    loop {
        println!("Password Manager");
        println!("1. Add new entry");
        println!("2. List all entries");
        println!("3. Search for a service");
        println!("4. Update an entry");
        println!("5. Delete an entry");
        println!("6. Exit");

        let choice = prompt("Enter your choice: ");
        match choice.as_str() {
            "1" => {
                let service = prompt("Service: ");
                let username = prompt("Username: ");
                let password = prompt("Password: ");
                db::write_password_to_db(&conn, &service, &username, &password).expect("Failed to add entry.");
            }
            "2" => {
                let entries = read_passwords_from_db(&conn).expect("Failed to read entries.");
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            "3" => {
                let service = prompt("Service: ");
                match search_service_by_name(&conn, &service).expect("Failed to search service.") {
                    Some(entry) => println!("{:?}", entry),
                    None => println!("Service not found."),
                }
            }
            "4" => {
                let id: i64 = prompt("Entry ID to update: ").parse().expect("Invalid ID.");
                let new_service = Some(prompt("New Service: "));
                let new_username = Some(prompt("New Username: "));
                let new_password = Some(prompt("New Password: "));
                update_entry(&conn, id, new_service.as_deref(), new_username.as_deref(), new_password.as_deref())
                    .expect("Failed to update entry.");
            }
            "5" => {
                let id: i64 = prompt("Entry ID to delete: ").parse().expect("Invalid ID.");
                delete_from_db(&conn, id).expect("Failed to delete entry.");
            }
            "6" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

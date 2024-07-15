use serde::{Deserialize, Serialize};
use std::io::{self, Write};
extern crate rusqlite;
use rusqlite::{Connection, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            id: None,
            service,
            username,
            password,
        }
    }
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn init_database() -> Result<Connection, Error> {
    let conn = Connection::open("passwords.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords(
        id INTEGER PRIMARY KEY,
        service TEXT,
        username TEXT,
        password TEXT
        )",
        [],
    )?;
    Ok(conn)
}

pub fn write_password_to_db(
    conn: &Connection,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO passwords (service, username, password) VALUES ( ?, ?, ? )",
        &[service, username, password],
    )?;
    Ok(())
}

pub fn read_passwords_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password FROM passwords")?;
    let entries = stmt
        .query_map([], |row| {
            Ok(ServiceInfo {
                id: row.get(0)?,
                service: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

pub fn search_service_by_name(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password FROM passwords WHERE service = ?")?;
    let result = stmt.query_row([name], |row| {
        Ok(ServiceInfo {
            id: row.get(0)?,
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

pub fn update_entry(
    conn: &Connection,
    id: i64,
    new_service: Option<&str>,
    new_username: Option<&str>,
    new_password: Option<&str>,
) -> Result<(), Error> {
    let current_entry = search_service_by_id(conn, id)?;

    if let Some(mut entry) = current_entry {
        if let Some(service) = new_service {
            entry.service = service.to_string();
        }
        if let Some(username) = new_username {
            entry.username = username.to_string();
        }
        if let Some(password) = new_password {
            entry.password = password.to_string();
        }

        conn.execute(
            "UPDATE passwords SET service = ?, username = ?, password = ? WHERE id = ?",
            &[&entry.service, &entry.username, &entry.password, &id.to_string()],
        )?;
    } else {
        return Err(Error::QueryReturnedNoRows);
    }

    Ok(())
}

pub fn delete_from_db(conn: &Connection, id: i64) -> Result<(), Error> {
    conn.execute("DELETE FROM passwords WHERE id = ?", &[&id.to_string()])?;
    Ok(())
}

pub fn search_service_by_id(conn: &Connection, id: i64) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password FROM passwords WHERE id = ?")?;
    let result = stmt.query_row([id], |row| {
        Ok(ServiceInfo {
            id: row.get(0)?,
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

fn main() {
    let conn = init_database().expect("Failed to initialize the database");

    let service = prompt("Enter the service name: ");
    let username = prompt("Enter the username: ");
    let password = prompt("Enter the password: ");

    write_password_to_db(&conn, &service, &username, &password).expect("Failed to write password to database");

    let passwords = read_passwords_from_db(&conn).expect("Failed to read passwords from database");
    for entry in passwords {
        println!("{:?}", entry);
    }

    let id = prompt("Enter the ID of the entry to update: ").parse::<i64>().expect("Invalid ID");
    let new_service = prompt("Enter the new service name (or leave blank to keep unchanged): ");
    let new_username = prompt("Enter the new username (or leave blank to keep unchanged): ");
    let new_password = prompt("Enter the new password (or leave blank to keep unchanged): ");

    update_entry(&conn, id, Some(&new_service), Some(&new_username), Some(&new_password)).expect("Failed to update entry");

    let id_to_delete = prompt("Enter the ID of the entry to delete: ").parse::<i64>().expect("Invalid ID");
    delete_from_db(&conn, id_to_delete).expect("Failed to delete entry");
}

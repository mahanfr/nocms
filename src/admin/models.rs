use crate::database::{Model, Properties, Table};
// Pasword managment
use rand::Rng;
use sha2::{Digest, Sha256};

fn generate_salt() -> String {
    let mut rnd = rand::thread_rng();
    let char_set = "GXhMW2xV1wH7zZ5jJO32nN3o4fU0v45Cg2dD9K06FqQ6rR7sSulL13eaA01c8E4yiIpP5mbBY9k8tT";
    let mut salt = String::new();
    for _ in 0..10 {
        let index = rnd.gen_range(0..char_set.len());
        salt.push(char_set.as_bytes()[index] as char);
    }
    salt
}

// password = H(pass + salt) + 1b8c + salt
fn make_password(password: String) -> String {
    let salt = generate_salt();
    let mut hasher = Sha256::new();
    hasher.update(salt.as_str());
    hasher.update(password);
    let mut new_pass = format!("{:x}", hasher.finalize());
    new_pass.push_str("1b8c");
    new_pass.push_str(salt.as_str());
    new_pass
}

#[allow(dead_code)]
pub struct Admin {
    id: u32,
    name: String,
    email: String,
    password: String,
}
impl Admin {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: 0,
            name,
            email,
            password: make_password(password),
        }
    }
    // // TODO: Add Cryptographic hash to password
    // fn create_user_quary(name: String, email: String, password: String) -> String {
    //     let mut quary = String::new();
    //     quary.push_str("INSERT INTO admin ");
    //     quary.push_str("(name, email, password) ");
    //     quary.push_str(format!("VALUES ('{name}','{email}','{}');",make_password(password)).as_str());
    //     quary
    // }

    // async fn create_superuser(connection : &Connection, name: String, email: String, password: String) {
    //     // Check if table exists
    //     let quary = "CREATE TABLE IF NOT EXISTS admin (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, password TEXT)";
    //     // create table
    //     connection.execute(quary).unwrap();
    //     connection.execute( create_user_quary(name, email, password)).unwrap();

    //     connection.iterate("SELECT * FROM admin", |pairs| {
    //         for &(name, value) in pairs.iter() {
    //             println!("{} = {}", name, value.unwrap());
    //         }
    //         true
    //     }).unwrap();
    // }
}
impl Model for Admin {
    fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(
            "id".to_string(),
            "INTEGER".to_owned(),
            "PRIMARY KEY AUTOINCREMENT".to_owned(),
        );
        table.add_row("name".to_string(), "TEXT".to_owned(), Properties::default());
        table.add_row(
            "email".to_string(),
            "TEXT".to_owned(),
            Properties::default(),
        );
        table.add_row(
            "password".to_string(),
            "TEXT".to_owned(),
            Properties::default(),
        );
        table
    }

    fn get_name(&self) -> String {
        "admin".to_string()
    }
}

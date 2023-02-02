use crate::{
    database::{Model, Table},
    table,
};
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
    name: String,
    email: String,
    password: String,
}
impl Admin {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password: make_password(password),
        }
    }
}
impl Model for Admin {
    fn insert_query(&self) -> String {
        let mut quary = String::new();
        quary.push_str("INSERT INTO admin ");
        quary.push_str("(name, email, password) ");
        quary.push_str(
            format!(
                "VALUES ('{}','{}','{}');",
                self.name,
                self.email,
                make_password(self.password.to_string())
            )
            .as_str(),
        );
        quary
    }
}
pub fn default_tables() -> Vec<Table> {
    let mut tables = Vec::<Table>::new();
    let admin_table = table!(admin {
        id: "INTEGER PRIMARY KEY AUTOINCREMENT",
        name: "TEXT",
        email: "TEXT",
        password: "TEXT",
    });
    tables.push(admin_table);
    tables
}

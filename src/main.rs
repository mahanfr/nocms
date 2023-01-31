use std::error::Error;
use std::env;
use sqlite::Connection;
use tokio::join;
// Pasword managment
use sha2::{Sha256, Digest};
use rand::Rng;
mod admin;
mod utils;

// struct Context {
//     db_connection: Connection
// }

fn generate_salt() -> String {
    let mut rnd = rand::thread_rng();
    let char_set = "GXhMW2xV1wH7zZ5jJO32nN3o4fU0v45Cg2dD9K06FqQ6rR7sSulL13eaA01c8E4yiIpP5mbBY9k8tT";
    let mut salt = String::new();
    for _ in 0..10{
        let index = rnd.gen_range(0..char_set.len());
        salt.push(char_set.as_bytes()[index] as char);
    }
    salt
}

fn get_superuser_args(args: &Vec<String>) -> Result<(String,String,String),String> {
    if args.len() == 5 {
        Ok((args[2].to_string(),args[3].to_string(),args[4].to_string()))
    }else {
        Err(format!("not enogth arguments"))
    }
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

// TODO: Add Cryptographic hash to password
fn create_user_quary(name: String, email: String, password: String) -> String {
    let mut quary = String::new();
    quary.push_str("INSERT INTO admin ");
    quary.push_str("(name, email, password) ");
    quary.push_str(format!("VALUES ('{name}','{email}','{}');",make_password(password)).as_str());
    quary
}

async fn create_superuser(connection : &Connection, name: String, email: String, password: String) {
    // Check if table exists
    let quary = "CREATE TABLE admin (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, password TEXT)";
    // create table
    match connection.execute(quary) {
        Ok(_) => {
            // insert user inside table
            connection.execute( create_user_quary(name, email, password)).unwrap();
        },
        Err(err) => {
            if err.message == Some("table admin already exists".to_owned()) {
                // insert user inside table
                connection.execute( create_user_quary(name, email, password)).unwrap();
            }
        },
    }
    
    connection.iterate("SELECT * FROM admin", |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    }).unwrap();
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let db_connection = sqlite::open("db.sqlite3")?;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {}
    match &args[1].as_str() {
        &"createsuperuser" => {
            let (name,email,password) = get_superuser_args(&args).unwrap();
            create_superuser(&db_connection,name,email,password).await;
        },
        &"run" => {
            let admin_server = admin::run_admin_service();
            let _ret = join!(admin_server,);
        },
        _ => {
            println!("command not found!");
            println!("help: --help -h");
        }
    }
    Ok(())
}

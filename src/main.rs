use std::error::Error;
use std::env;
use std::process::exit;
use tokio::join;
mod utils;
mod database;
mod admin;
use crate::admin::models;

fn get_superuser_args(args: &Vec<String>) -> Result<(String,String,String),String> {
    if args.len() == 5 {
        Ok((args[2].to_string(),args[3].to_string(),args[4].to_string()))
    }else {
        Err(format!("not enogth arguments"))
    }
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let db = database::Database::new();
    db.migrate_all(models::default_tables());
    //let db_connection = sqlite::open("db.sqlite3")?;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("ERROR: Not Enough Arguments!");
        println!("nocms help | nocms --help | nocms -h");
        exit(1)
    }
    match &args[1].as_str() {
        &"createsuperuser" => {
            let (name,email,password) = get_superuser_args(&args).unwrap();
            let admin = models::Admin::new(name,email,password);
            db.insert_one(admin);
        },
        &"start" => {
            let admin_server = admin::run_admin_service();
            let _ret = join!(admin_server,);
        },
        &"help" => {
            println!("nocms commands <options>");
            println!("  - start           : runs admin server for db managment");
            println!("  - createsuperuser : creating managment user");
            println!("      args: <username> <email> <password>");
            println!("  - help            : --help -h");
        },
        _ => {
            println!("command not found!");
            println!("nocms help | nocms --help | nosms -h");
        }
    }
    Ok(())
}

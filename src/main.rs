use std::error::Error;
use std::env;
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
    //let db_connection = sqlite::open("db.sqlite3")?;
    println!("{:?}",database::test());
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {}
    match &args[1].as_str() {
        &"createsuperuser" => {
            let (name,email,password) = get_superuser_args(&args).unwrap();
            let admin = models::Admin::new(name,email,password);
            db.migrate(admin);
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

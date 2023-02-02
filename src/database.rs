use std::{collections::HashMap};

use sqlite::Connection;

// TODO: Add all velue properties
pub type Properties = String;
// pub struct Properties {
//     not_null: bool,
//     primary_key: bool,
//     foreign_key: bool,
//     auto_increment: bool,
// }
// impl Default for Properties {
//     fn default() -> Self {
//         Self {
//             not_null: Default::default(),
//             primary_key: Default::default(),
//             foreign_key: Default::default(),
//             auto_increment: Default::default(),
//         }
//     }
// }

pub struct Database {
    connection: Box<Connection>,
    #[allow(dead_code)]
    map: HashMap<String, Table>,
}
impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open("db.sqlite3").unwrap();
        let map = Self::get_schema(&connection);
        Database {
            connection: Box::new(connection),
            map,
        }
    }

    pub fn migrate(self,model: impl Model) {
        let table = model.create_table(); 
        self.connection.execute(table.create(model.get_name())).unwrap();
    }

    // pub fn insert_model(self, model:impl Model) {

    // }


    fn get_schema(conn: &Connection) -> HashMap<String, Table> {
        let mut _map: HashMap<String, Table> = HashMap::new();
        conn.iterate("SELECT sql FROM sqlite_master", |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();
        HashMap::new()
    }

}

// TODO: Create macro for tables
pub struct Table {
    columns: HashMap<String, ModelValue>,
}
impl Table {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }
    pub fn add_row(&mut self, name: String, kind: String, properties: Properties) {
        self.columns.insert(name, ModelValue { kind, properties });
    }
    pub fn create(self, name:String) -> String {
        // CREATE TABLE IF NOT EXISTS admin (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, password TEXT)
        let mut quary = String::new();
        quary.push_str(format!("CREATE TABLE IF NOT EXISTS {name} (").as_str());
        for item in self.columns.iter(){
            quary.push_str(&format!("{} {} {},", item.0, item.1.kind, item.1.properties))
        }
        if quary.ends_with(',') {
            quary.pop();
        }
        quary.push_str(format!(");").as_str());
        quary
    }
}

pub trait Model {
    fn create_table(&self) -> Table;
    fn get_name(&self) -> String;
}

struct ModelValue {
    kind: String,
    properties: String,
    //properties: Properties,
    // read_only: bool,
}

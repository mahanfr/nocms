use std::collections::HashMap;

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
#[allow(dead_code)]
impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open("db.sqlite3").unwrap();
        let map = Self::get_schema(&connection);
        Database {
            connection: Box::new(connection),
            map,
        }
    }

    pub fn migrate(self, table: Table) {
        self.connection
            .execute(table.create_insert_query())
            .unwrap();
    }

    pub fn insert_one(&self, model: impl Model) {
        self.connection.execute(model.insert_query()).unwrap();
    }

    pub fn migrate_all(&self, tables: Vec<Table>) {
        for table in tables {
            self.connection
                .execute(table.create_insert_query())
                .unwrap();
        }
    }

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
#[derive(Debug)]
pub struct Table {
    columns: HashMap<String, ModelValue>,
    name: String,
}
#[allow(dead_code)]
impl Table {
    pub fn new(name: String) -> Self {
        Self {
            columns: HashMap::new(),
            name,
        }
    }
    pub fn add_row(&mut self, name: String, kind: String, properties: Properties) {
        self.columns.insert(name, ModelValue { kind, properties });
    }
    pub fn add_row_default(&mut self, name: String, kind: String) {
        self.columns.insert(
            name,
            ModelValue {
                kind,
                properties: Properties::default(),
            },
        );
    }
    pub fn create_insert_query(self) -> String {
        // CREATE TABLE IF NOT EXISTS admin (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, password TEXT)
        let mut quary = String::new();
        quary.push_str(format!("CREATE TABLE IF NOT EXISTS {} (", self.name).as_str());
        for item in self.columns.iter() {
            quary.push_str(&format!(
                "{} {} {},",
                item.0, item.1.kind, item.1.properties
            ))
        }
        if quary.ends_with(',') {
            quary.pop();
        }
        quary.push_str(format!(");").as_str());
        quary
    }
}

// Creates a table struct based on structure
/// ```
/// table!{
///     id: INTEGER PRIMARY KEY AUTOINCRIMENT,
///     name: TEXT,
///     email: TEXT,
///     pasword: TEXT
/// }
/// ```
/// TODO: make function create model Struct as well and
/// add table as a unstructed function inside implementaion
#[macro_export]
macro_rules! table {(
    $table_name:ident {
        $(
            $field_name:ident : $atrr:tt
        ),*$(,)+
    }
) => {{
        let mut temp_vec = Table::new(stringify!($table_name).to_string());
        $(
            temp_vec.add_row_default(stringify!($field_name).to_string(),$atrr.to_string());
        )*
        temp_vec
    }}
}

pub trait Model {
    fn insert_query(&self) -> String;
    // fn get_name(&self) -> String;
}

#[derive(Debug)]
struct ModelValue {
    kind: String,
    properties: String,
    //properties: Properties,
    // read_only: bool,
}

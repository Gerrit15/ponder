use crate::*;
pub struct Database {
    pub connection: Connection
}

impl Database {
    pub fn new(dir: &str) -> (Database, SpellEnums) {
        let connection = sqlite::open(":memory:").unwrap();
        let mut setup = "
            CREATE TABLE spells (
                name TEXT, 
                source TEXT,
                level INTEGER,
                School TEXT,
                ritual INTEGER,
                casting_time INTEGER,
                casting_units TEXT,
                v INTEGER,
                s INTEGER,
                m INTEGER,
                component_cost INTEGER,
                range INTEGER,
                radius INTEGER,
                shape TEXT,
                text TEXT,
                higher_level TEXT,
                lists TEXT,
                proc_eff TEXT,
                proc_save TEXT,
                damage1 INTEGER,
                damage2 INTEGER,
                damage3 INTEGER,
                damage_types TEXT,
                tags TEXT
        );".to_string();
        let spells = Spell::load_spells(dir);
        let mut spell_enums = SpellEnums::new();

        for i in spells {
            setup += &("INSERT INTO spells VALUES (".to_string() + &i.1.values() + ");");
            spell_enums.update(&i.1);
        }
        connection.execute(setup).unwrap();
        let db = Database {connection};
        return (db, spell_enums)
    }
}

#[derive(Debug)]
pub struct Query {
    pub text: String
}

impl Query {
    pub fn new(table: &str, field: &str, operator: &str, value: QueryValue) -> Query {
        Query {text: "SELECT * FROM ".to_owned() + table + " WHERE " + field + " " + operator + " " + &value.to_string()}
    }
    
    pub fn append(&mut self, field: &str, operator: &str, value: QueryValue) {
        self.text += &(" AND ".to_string() + &field + " " + &operator + " " + &value.to_string());
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum QueryValue {
    Text(String),
    Integer(i32),
    Boolean(bool)
}

impl QueryValue {
    pub fn to_string(&self) -> String {
        return match self {
            Self::Text(t) => "'".to_owned() + &t.to_owned() + "'",
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

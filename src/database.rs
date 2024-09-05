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
        let spells = load_spells(dir);
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

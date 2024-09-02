use sqlite;
use serde_json::Error;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::io;
fn main() {
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
        );
    ".to_string();

    let mut path = std::path::PathBuf::new();
    path.push("/home/gerrit/projects/ponder/spells");
    let mut spells = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            let x = entries.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>();
            match x {
                Ok(mut x) => {
                    x.sort();
                    for spell_entry_buff in x {
                        let spell_entries = fs::read_dir(spell_entry_buff);
                        match spell_entries {
                            Ok(entries) => {
                                let x = entries.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
                                for i in x {
                                    let spell = Spell::new_from_json(i);
                                    match spell {
                                        Ok(okspell) => spells.push(okspell),
                                        Err(e) => println!("Final load error: {}, {}", e.0, e.1.to_str().unwrap())
                                    }
                                }
                            },
                            Err(_e) => panic!("Error in loading entries"),
                        }
                    }
                },
                Err(_e) => panic!("Error in reading directories")
            }
        },
        Err(_e) => panic!("Error in reading path")
    }
    for i in spells {
        setup += &("INSERT INTO spells VALUES (".to_string() + &i.values() + ");");
    }



    /*let spell = Spell::new_from_json(path).unwrap();
    setup += &("INSERT INTO spells VALUES (".to_string() + &spell.values() + ");"); 
    */

    connection.execute(setup).unwrap();

    let mut query = Query::new("spells","level", "=", QueryValue::Integer(8));
    query.append("radius", ">=", QueryValue::Integer(30));

    let mut values = vec![];
    connection.iterate(query.text, |pairs| {
        let pair = pairs[0].1.unwrap_or("None");
        values.push(pair.to_owned());
        true
    }).unwrap();
    for i in values {println!("{i}");}
}


#[derive(Debug)]
struct Query {
    text: String
}

impl Query {
    fn new(table: &str, field: &str, operator: &str, value: QueryValue) -> Query {
        Query {text: "SELECT * FROM ".to_owned() + table + " WHERE " + field + " " + operator + " " + &value.to_string()}
    }
    
    fn append(&mut self, field: &str, operator: &str, value: QueryValue) {
        self.text += &(" AND ".to_string() + &field + " " + &operator + " " + &value.to_string());
    }
}

//Note that bool must be converted into text or a integer
#[derive(Debug)]
#[allow(unused)]
enum QueryValue {
    Text(String),
    Integer(i32),
    Boolean(bool)
}

impl QueryValue {
    fn to_string(&self) -> String {
        return match self {
            Self::Text(t) => "'".to_owned() + &t.to_owned() + "'",
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}


#[derive(Clone, Deserialize, Debug)]
pub struct Spell {
    pub title: String, 
    pub source: String,
    pub lv: u32,
    pub school: String,
    pub ritual: bool,
    //time length, time type, (v,s,m)
    pub casting_time: (u32, String, (bool, bool, bool)),
    pub component_cost: bool,
    //range, size of aoe, measurement (ft) type
    pub range: (u32, u32, String),
    //last one is for concentration
    pub duration: (u32, String, bool),
    pub text: String,
    pub higher_lv: String, 
    pub spell_lists: Vec<String>,
    //"Instant", "Save", "Ranged Spell Attack", "Melee Attack" and "Dextarity", "Strength", etc
    pub proc: (String, String),
    //# dice, dice type, added damage, damage type(s)
    pub damage: (u32, u32, u32, Vec<String>),
    pub tags: Vec<String>,
}

impl Spell {
    pub fn new_from_json(spell_path: PathBuf) -> Result<Spell, (String, PathBuf)> {
        let spell_file = fs::read_to_string(spell_path.clone());
        match spell_file {
            Ok(x) => {
                let spell: Result<Spell, Error> = serde_json::from_str(&x);
                match spell {
                    Ok(x) => Ok(x),
                    Err(x) => Err((x.to_string(), spell_path))
                }
            },
            Err(x) => Err((x.to_string(), spell_path)),
        }
    }
    pub fn values(&self) -> String {
        let delim = "', '";
        let spell_list = {
            let mut s = String::new();
            for i in &self.spell_lists {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let types = {
            let mut s = String::new();
            for i in &self.damage.3 {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let tags = {
            let mut s = String::new();
            for i in &self.tags {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let title = self.title.clone().replace("'", "''");
        let source = self.source.clone().replace("'", "''");
        let text = self.source.clone().replace("'", "''");
        let higher_lv = self.source.clone().replace("'", "''");
        let s = "'".to_string() + &title + delim + &source + "', "+ &self.lv.to_string() + ", '"+ &self.school + "', "+ &(self.ritual as u32).to_string() + ", "+ &self.casting_time.0.to_string() + ", '"+ &self.casting_time.1 + "', "+ &(self.casting_time.2.0 as u32).to_string() + ", "+ &(self.casting_time.2.1 as u32).to_string() + ", "+ &(self.casting_time.2.2 as u32).to_string() + ", " + &(self.component_cost as u32).to_string() + ", "+ &self.range.0.to_string() + ", "+ &self.range.1.to_string() + ", '"+ &self.range.2 + delim + &text + delim + &higher_lv + delim + &spell_list + delim + &self.proc.0 + delim + &self.proc.1 + "', "+ &self.damage.0.to_string() + ", "+ &self.damage.1.to_string() + ", " + &self.damage.2.to_string() + ", '"+ &types + delim + &tags + "' ";
        return s
    }
}

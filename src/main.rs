//TODO TODO TODO TODO TODO TODO TODO TODO 
//YOU NEED TO CLEAN OUT ANY APOSTROPHIES like in player's
//TODO TODO TODO TODO TODO TODO TODO TODO 
use sqlite;
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
    setup += "INSERT INTO spells VALUES('Eldritch Blast', 'Players Handbook', 0, 'Evocation', 0, 1, 'action', 1, 1, 0, 0, 120, 0, 'none', 'text', 'higher text', 'Warlock', 'Ranged Spell Attack', 'none', 1, 10, 0, 'Force', 'Attack');";

    connection.execute(setup).unwrap();

    let mut values = vec![];
    //let query = "SELECT * FROM spells WHERE level = 0 AND damage_type = 'Fire'";
    let query = Query::new("spells","level", "=", QueryValue::Integer(0));
    println!("{}", query.text);

    connection.iterate(query.text, |pairs| {
        println!("{:?}", pairs);
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

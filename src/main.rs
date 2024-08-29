use sqlite;
fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    let query = "
        CREATE TABLE spells (name TEXT, level INTEGER, damage_type TEXT);
        INSERT INTO spells VALUES ('Eldritch Blast', 0, 'Force');
        INSERT INTO spells VALUES ('Fire Bolt', 0, 'Fire');
        INSERT INTO spells VALUES ('Fireball', 3, 'Fire');
    ";
    connection.execute(query).unwrap();

    let mut values = vec![];
    //let query = "SELECT * FROM spells WHERE level = 0 AND damage_type = 'Fire'";
    let mut query = Query::new("spells".to_string(),"level".to_owned(), "=".to_owned(), QueryValue::Integer(0));
    query.append("damage_type".to_string(), "=".to_owned(), QueryValue::Text("Fire".to_string()));
    println!("{}", query.text);

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
    fn new(table: String, field: String, operator: String, value: QueryValue) -> Query {
        Query {text: "SELECT * FROM ".to_owned() + &table + " WHERE " + &field + " " + &operator + " " + &value.to_string()}
    }
    
    fn append(&mut self, field: String, operator: String, value: QueryValue) {
        self.text += &(" AND ".to_string() + &field + " " + &operator + " " + &value.to_string());
    }
}

//Note that bool must be converted into text or a integer
#[derive(Debug)]
enum QueryValue {
    Text(String),
    Integer(i32),
    Boolean(bool)
}

impl QueryValue {
    fn to_string(&self) -> String {
        return match self {
            Self::Text(t) => t.to_owned(),
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

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
    let query = "SELECT * FROM spells WHERE level = 0";

    connection.iterate(query, |pairs| {
        let pair = pairs[0].1.unwrap_or("None");
        values.push(pair.to_owned());
        true
    }).unwrap();

    for i in values {println!("{i}");}
}

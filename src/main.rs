mod database;
mod spellenums;
mod spell;
use sqlite::{self, Connection};
use serde_json::Error;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::io;
use std::collections::HashMap;
use database::{Database, Query, QueryValue};
use spellenums::SpellEnums;
use spell::Spell;

fn main() {
    let (db, spell_enums) = Database::new("/home/gerrit/projects/ponder/spells");
    for i in spell_enums.damage_types {print!("{i}, ")}
    let mut query = Query::new("spells","source", "=", QueryValue::Text("Player''s Handbook".to_owned()));
    query.append("level", "<=", QueryValue::Integer(1));
    query.append("damage_types", "=", QueryValue::Text(" Force".to_owned()));

    let mut values = vec![];
    db.connection.iterate(query.text, |pairs| {
        let pair = pairs[0].1.unwrap_or("None");
        values.push(pair.to_owned());
        true
    }).unwrap();
    println!("Spells: ");
    for i in values {println!("{i}");}
}

mod database;
mod spellenums;
mod spell;
mod app;
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
use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block,
        Paragraph,
        Widget
    },
    DefaultTerminal, Frame
};

/*
* TODO
*  - Filtering with strings, should happen after get Vec<> from hashmap with a search
*  - Frontend
*  - idk make it nice
*/

fn main() {
    /*let (db, spell_enums) = Database::new("/home/gerrit/projects/ponder/spells");
    for i in spell_enums.damage_types {print!("{i}, ")}
    let mut query = Query::new("spells","source", "=", QueryValue::Text("Player''s Handbook".to_owned()));

    let mut values = vec![];
    db.connection.iterate(query.text, |pairs| {
        let pair = pairs[0].1.unwrap_or("None");
        values.push(pair.to_owned());
        true
    }).unwrap();
    println!("Spells: ");
    for i in values {println!("{i}");}*/

    App::new("TEST".to_owned()).start().unwrap();
}

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
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
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

#[derive(Debug)]
struct App {
    text: String,
    exit: bool,
}

impl App {
    fn new(text: String) -> App {
        App { text, exit: false }
    }
    fn start(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = self.run(&mut terminal);
        ratatui::restore();
        app_result
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
    //Pickup at Displaying the application https://ratatui.rs/tutorials/counter-app/basic-app/
}



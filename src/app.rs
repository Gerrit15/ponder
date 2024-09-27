use ratatui::prelude::{Layout, Direction, Constraint};

use crate::*;

pub struct App {
    pub exit: bool,
    pub db: Database,
    //TODO make sure in the end that this is actually used?
    //Spells hash could just be in 1st page
    //pub spells: HashMap<String, Spell>,
    pub spell_enums: SpellEnums,
    selected_tab: Tab,
    pages: Vec<Box<dyn Page>>,
}

impl App {
    pub fn new(dir: &str) -> App {
        let (db, spells, spell_enums) = Database::new(dir);
        App {
            exit: false,
            db,
            //spells: spells.clone(),
            spell_enums,
            selected_tab: Tab::new(),
            pages: vec![Box::new(MainList::new(spells))]
        }
    }

    //Init function, setting up everything it'll need, kinda acts as a wrapper around Run()
    //It makes sure that when it stops running it cleans up
    pub fn start(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = self.run(&mut terminal);
        ratatui::restore();
        app_result
    }

    //the main loop of the App, rn it just draws a frame and asks for what happened
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let out_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),
                Constraint::Percentage(94),
            ]).split(frame.area());

        self.selected_tab.draw(frame, out_layout[0]);
        self.pages[self.selected_tab.pointer].draw_page(frame, out_layout[1]);
    }


    //This is how we manage *shit that happened* in the loop 
    //Right now mainly just offloads onto checking keys
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => ()
        };
        Ok(())
    }

    //this is where we check the keys
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Right => self.selected_tab.next(),
            KeyCode::Left => self.selected_tab.prev(),
            k => {self.pages[self.selected_tab.pointer].key(k)}
        }
    }

    //Says it on the tin, it just flips our bit
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

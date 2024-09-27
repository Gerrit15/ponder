use crate::*;

pub struct App {
    pub exit: bool,
    pub db: Database,
    //TODO make sure in the end that this is actually used?
    //Spells hash could just be in 1st page
    pub spells: HashMap<String, Spell>,
    pub spell_enums: SpellEnums,
    source_index: usize,
    page_num: usize,
    pages: Vec<Box<dyn Page>>
}

impl App {
    pub fn new(dir: &str) -> App {
        let (db, spells, spell_enums) = Database::new(dir);
        App {
            exit: false,
            db,
            spells: spells.clone(),
            spell_enums,
            source_index: 0,
            page_num: 0,
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
            terminal.draw(|frame| self.pages[self.page_num].draw_page(frame))?;
            self.handle_events()?;
        }
        Ok(())
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
            k => {self.pages[self.page_num].key(k)}
        }
    }

    //Says it on the tin, it just flips our bit
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" TEST TITLE ".bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom),)
            .border_set(border::THICK);
        let text = Text::from(vec![Line::from(vec![self.spell_enums.sources[self.source_index].clone().into()])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

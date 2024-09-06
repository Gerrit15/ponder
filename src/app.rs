use crate::*;
pub struct App {
    pub exit: bool,
    pub db: Database,
    pub spells: HashMap<String, Spell>,
    pub spell_enums: SpellEnums,
    source_index: usize,
}

impl App {
    pub fn new(dir: &str) -> App {
        let (db, spells, spell_enums) = Database::new(dir);
        App {
            exit: false,
            db,
            spells,
            spell_enums,
            source_index: 0,
        }
    }
    pub fn start(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = self.run(&mut terminal);
        ratatui::restore();
        app_result
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => ()
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('l') => self.next_source(),
            KeyCode::Char('h') => self.prev_source(),
            _ => ()
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn next_source(&mut self) {
        if (self.source_index + 1) == self.spell_enums.sources.len() {
            self.source_index = 0
        } else {
            self.source_index += 1
        }
    }
    pub fn prev_source(&mut self) {
        if self.source_index == 0 {
            self.source_index = self.spell_enums.sources.len() - 1
        } else {
            self.source_index -= 1
        }
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

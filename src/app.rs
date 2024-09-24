use ratatui::{prelude::{Layout, Direction, Constraint}, widgets::{Borders, BorderType, List, ListState}, style::{Style, Modifier}};

use crate::*;
pub struct App {
    pub exit: bool,
    pub db: Database,
    pub spells: HashMap<String, Spell>,
    pub spell_enums: SpellEnums,
    source_index: usize,
    spell_state: ListState 
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
            spell_state: ListState::default(),
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

    //Simply draws our frame. this will be where to edit the appearence
    pub fn draw(&mut self, frame: &mut Frame) {
        let out_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),
                Constraint::Percentage(94),
            ]).split(frame.area());
        let in_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(75)
            ]).split(out_layout[1]);

        let spell_names = self.spells.values().map(|s| s.title.clone()).collect::<Vec<String>>();
        let list = List::new(spell_names.clone())
                    .block(Block::bordered().title("LIST TITLE"))
                    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                    .highlight_symbol(">")
                    .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, in_layout[0], &mut self.spell_state);
            
//        frame.render_widget(self, frame.area());
        frame.render_widget(Paragraph::new("TEST").block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded)), out_layout[0]);
        frame.render_widget(Paragraph::new(self.spells.get(&spell_names[self.spell_state.selected().unwrap_or(0)]).unwrap().text.clone()).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded)), in_layout[1]);
//        frame.render_widget(Paragraph::new(self.spell_enums.school[self.source_index].clone()).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded)), in_layout[1]);
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
            KeyCode::Char('l') => self.next_source(),
            KeyCode::Char('h') => self.prev_source(),
            KeyCode::Char('j') => self.spell_state.select_next(),
            KeyCode::Char('k') => self.spell_state.select_previous(),
            _ => ()
        }
    }

    //Says it on the tin, it just flips our bit
    pub fn exit(&mut self) {
        self.exit = true;
    }

    //A test function, cycles between sources
    pub fn next_source(&mut self) {
        if (self.source_index + 1) == self.spell_enums.sources.len() {
            self.source_index = 0
        } else {
            self.source_index += 1
        }
    }

    //A test function, cycles between sources
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

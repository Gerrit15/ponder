use crate::*;
#[derive(Debug)]
pub struct App {
    pub text: String,
    pub number: u32,
    pub exit: bool,
}

impl App {
    pub fn new(text: String) -> App {
        App { text, number: 0, exit: false }
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
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => ()
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn increment_counter(&mut self) {
        self.number += 1;
    }
    pub fn decrement_counter(&mut self) {
        self.number -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" TEST TITLE ".bold());
        let instructions = Title::from(Line::from(vec![
            " Down ".into(),
            "<--".blue().bold(),
            " Up ".into(),
            "-->".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom),)
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.number.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

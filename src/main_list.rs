use ratatui::{widgets::{ListState, List, Borders, BorderType, Wrap}, prelude::{Layout, Direction, Constraint}, style::{Style, Modifier}};
use crate::*;

pub struct MainList {
    spell_state: ListState,
    spells: HashMap<String, Spell>
}

impl MainList {
    pub fn new(spells: HashMap<String, Spell>) -> MainList {
        MainList {
            spell_state: ListState::default(),
            spells: spells.clone()
        }
    }
    
}

impl Page for MainList {
    fn draw_page(&mut self, frame: &mut Frame) {
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
            
        frame.render_widget(Paragraph::new("TEST").block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded)), out_layout[0]);
        frame.render_widget(Paragraph::new(self.spells.get(&spell_names[self.spell_state.selected().unwrap_or(0)]).unwrap().text.clone()).wrap(Wrap {trim: true}).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded)), in_layout[1]);

    }
    fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') => self.spell_state.select_next(),
            KeyCode::Char('k') => self.spell_state.select_previous(),
            _ => ()
        }
    }
}

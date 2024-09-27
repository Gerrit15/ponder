use ratatui::{widgets::Tabs, style::{Style, Modifier}};

use crate::*;

pub struct Tab {
    titles: Vec<String>,
    pub pointer: usize
}

impl Tab {
    pub fn new() -> Tab {
        Tab { 
            titles: vec!["Spells".to_string(), "Search".to_string(), "Log".to_string()],
            pointer: 0
        }
    }
}

impl Tab {
    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        /*frame.render_widget(
            Paragraph::new(self.selected_tab[self.pointer].clone())
                .block(Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
            layout);*/
        let t = Tabs::new(self.titles.clone()).block(Block::bordered().title("Ponder")).highlight_style(Style::default().add_modifier(Modifier::REVERSED)).select(self.pointer);
        frame.render_widget(t, layout);
    }
    pub fn next(&mut self) {
        if self.pointer + 1 == self.titles.len() {
            self.pointer = 0
        }
        else {
            self.pointer += 1
        }
    }
    pub fn prev(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.titles.len() - 1
        }
        else {
            self.pointer -= 1
        }
    }
}

use crate::*;
use ratatui::widgets::{Borders, BorderType};

pub struct Tab {
    selected_tab: Vec<String>,
    pointer: usize
}

impl Tab {
    pub fn new() -> Tab {
        Tab { 
            selected_tab: vec!["Spells".to_string(), "Search".to_string(), "Log".to_string()],
            pointer: 0
        }
    }
}

impl Tab {
    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        frame.render_widget(
            Paragraph::new(self.selected_tab[self.pointer].clone())
                .block(Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
            layout);
    }
    pub fn next(&mut self) {
        if self.pointer + 1 == self.selected_tab.len() {
            self.pointer = 0
        }
        else {
            self.pointer += 1
        }
    }
    pub fn prev(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.selected_tab.len() - 1
        }
        else {
            self.pointer -= 1
        }
    }
}

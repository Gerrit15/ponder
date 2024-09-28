use ratatui::{prelude::{Constraint, Layout}, layout::Flex, widgets::Clear};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    popup: bool
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            popup: false
        }
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        frame.render_widget(Paragraph::new("AAAAAAAA"), layout);
        if self.popup {
            frame.render_widget(Clear, area);
            frame.render_widget(Paragraph::new(self.spell_enums.tags.join(" ")), area)
        }
    }
    fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') => self.popup = !self.popup,
            _ => ()
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

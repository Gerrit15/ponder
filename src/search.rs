use ratatui::{prelude::{Constraint, Layout}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    popup: bool,
    tags_state: ListState,
    pre_search: PreSearch,
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            popup: false,
            tags_state: ListState::default(),
            pre_search: PreSearch::new()
        }
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        frame.render_widget(Paragraph::new(self.pre_search.tags.join(" ")), layout);
        if self.popup {
            let checked_tabs: Vec<String> = {
                let mut tabs = vec![];
                for i in &self.spell_enums.tags {
                    let check; 
                    if self.pre_search.tags.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i)
                }
                tabs
            };
            //let tags = List::new(self.spell_enums.tags.clone())
            let tags = List::new(checked_tabs)
                .block(Block::bordered().title("TAGS"))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(tags, area, &mut self.tags_state);
        }
    }
    fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') => self.popup = !self.popup,
            KeyCode::Char('j') => if self.popup { self.tags_state.select_next() }
            KeyCode::Char('k') => if self.popup { self.tags_state.select_previous() }
            KeyCode::Enter => if self.popup {match self.tags_state.selected() {
                Some(n) => self.pre_search.toggle_tag(&self.spell_enums.tags[n]),
                _ => ()
            }}
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


#[derive(Debug)]
struct PreSearch {
    tags: Vec<String>
}

impl PreSearch {
    pub fn new() -> PreSearch {
        PreSearch { tags: vec![] }
    }

    pub fn toggle_tag(&mut self, s:  &String) {
        let index = self.tags.iter().position(|r| r == s);
        match index {
            Some(n) => {let _ = self.tags.remove(n);},
            None => self.tags.push(s.clone())
        };
    }
}

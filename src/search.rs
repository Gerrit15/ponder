use ratatui::{prelude::{Constraint, Layout}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    popup: bool,
    selected: SearchSelected,
    tags_state: ListState,
    pre_search: SpellEnums,
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            popup: false,
            tags_state: ListState::default(),
            selected: SearchSelected::TAGS,
            pre_search: SpellEnums::new()
        }
    }
    //TODO make a macro for this, shit sucks
    fn get_checked(&self) -> Vec<String> {
        let mut tabs: Vec<String> = vec![];
        tabs.push("[ CLEAR ]".to_owned());
        tabs.push("[ NONE ]".to_owned());
        tabs.push("[ ALL ]".to_owned());

        use SearchSelected::*;
        match self.selected {
            SOURCES => {
                for i in &self.spell_enums.sources {
                    let check; 
                    if self.pre_search.sources.contains(i) {
                        check = "[+]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            SCHOOL => {
                 for i in &self.spell_enums.school {
                    let check; 
                    if self.pre_search.school.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            CASTINGUNITS => {
                 for i in &self.spell_enums.casting_units {
                    let check; 
                    if self.pre_search.casting_units.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            SHAPES => {
                 for i in &self.spell_enums.shapes {
                    let check; 
                    if self.pre_search.shapes.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            LISTS => {
                 for i in &self.spell_enums.lists{
                    let check; 
                    if self.pre_search.lists.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            PROCEFF => {
                 for i in &self.spell_enums.proc_eff{
                    let check; 
                    if self.pre_search.proc_eff.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            PROCSAVE => {
                 for i in &self.spell_enums.proc_save{
                    let check; 
                    if self.pre_search.proc_save.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            DMGTYPE => {
                 for i in &self.spell_enums.damage_types{
                    let check; 
                    if self.pre_search.damage_types.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
            TAGS => {
                 for i in &self.spell_enums.tags{
                    let check; 
                    if self.pre_search.tags.contains(i) {
                        check = "[x]".to_owned();
                    } else {
                        check = "[ ]".to_owned();
                    }
                tabs.push(check + i) 
                }
            },
        }

        return tabs
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        frame.render_widget(Paragraph::new(self.pre_search.tags.join(" ")), layout);
        if self.popup {
            let checked_tabs = self.get_checked();
            //let tags = List::new(self.spell_enums.tags.clone());
            let list = List::new(checked_tabs)
                .block(Block::bordered().title("TAGS"))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(list, area, &mut self.tags_state);
        }
    }
    fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') => self.popup = !self.popup,
            KeyCode::Char('j') => if self.popup { self.tags_state.select_next() }
            KeyCode::Char('k') => if self.popup { self.tags_state.select_previous() }
            KeyCode::Enter => if self.popup {match self.tags_state.selected() {
                Some(0) => self.pre_search.tags.clear(),
                //Some(1) => set all to negative
                Some(2) => self.pre_search.tags = self.spell_enums.tags.clone(),
                Some(n) => self.pre_search.toggle_tag(&self.spell_enums.tags[n-3]),
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
#[allow(dead_code)]
enum SearchSelected {
    SOURCES,
    SCHOOL,
    CASTINGUNITS,
    SHAPES,
    LISTS,
    PROCEFF,
    PROCSAVE,
    DMGTYPE,
    TAGS,
}

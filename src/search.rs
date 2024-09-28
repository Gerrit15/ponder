use ratatui::{prelude::{Constraint, Layout}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    popup: bool,
    selected: SearchSelected,
    states: Vec<ListState>,
    pre_search: SpellEnums,
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            popup: false,
            states: vec![ListState::default(), ListState::default(), ListState::default(), ListState::default(), ListState::default(), ListState::default(), ListState::default(), ListState::default(), ListState::default()],
            selected: SearchSelected::SCHOOL,
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
        macro_rules! checks {
            ($($variant:ident => $field:ident),*) => {
                match self.selected {
                    $($variant => {
                        for i in &self.spell_enums.$field{
                            let check; 
                            if self.pre_search.$field.contains(i) {
                                check = "[x]".to_owned();
                            } else {
                                check = "[ ]".to_owned();
                            }
                            tabs.push(check + i) 
                        }

                    },)*
                    _ => ()
                }
            };
        }
        checks!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types);
        return tabs
    }

    fn next_select(&mut self) {
        let mut u = usize::from(self.selected.clone());
        if u == 8 {u = 0}
        else {u += 1}
        self.selected = SearchSelected::from_usize(u).unwrap();
    }
    fn prev_select(&mut self) {
        let mut u = usize::from(self.selected.clone());
        if u == 0 {u = 8}
        else {u -= 1}
        self.selected = SearchSelected::from_usize(u).unwrap();
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        //frame.render_widget(Paragraph::new(self.pre_search.school.join(" ")), layout);
        if self.popup {
            let checked_tabs = self.get_checked();
            //let tags = List::new(self.spell_enums.tags.clone());
            let list = List::new(checked_tabs)
                .block(Block::bordered().title("TAGS"))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(list, area, &mut self.states[self.selected.clone() as usize]);
        }
    }
    fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') => self.popup = !self.popup,
            KeyCode::Char('j') => {
                if self.popup { self.states[self.selected.clone() as usize].select_next() }
                else {self.next_select()}
            }
            KeyCode::Char('k') => {
                if self.popup { self.states[self.selected.clone() as usize].select_previous() }
                else {self.prev_select()}
            }
            KeyCode::Enter => if self.popup {match self.states[self.selected.clone() as usize].selected() {
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


#[derive(Clone)]
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

impl SearchSelected {
    fn from_usize (value: usize) -> Option<SearchSelected> {
        use SearchSelected::*;
        match value {
            0 => Some(SOURCES),
            1 => Some(SCHOOL),
            2 => Some(CASTINGUNITS),
            3 => Some(SHAPES),
            4 => Some(LISTS),
            5 => Some(PROCEFF),
            6 => Some(PROCSAVE),
            7 => Some(DMGTYPE),
            8 => Some(TAGS),
            _ => None
        }
    }
}

impl From<SearchSelected> for usize {
    fn from(value: SearchSelected) -> Self {
        use SearchSelected::*;
        match value {
            SOURCES => 0,
            SCHOOL => 1,
            CASTINGUNITS => 2,
            SHAPES => 3,
            LISTS => 4,
            PROCEFF => 5,
            PROCSAVE => 6,
            DMGTYPE => 7,
            TAGS => 8,
        }
    }
}

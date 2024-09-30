use ratatui::{prelude::{Constraint, Layout}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    //popup: bool,
    selected: SearchSelected,
    states: Vec<ListState>,
    pre_search: (PreSearch, PreSearch),
    mode: SearchPageMode,
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            //popup: false,
            states: vec![ListState::default(); 9],
            selected: SearchSelected::SCHOOL,
            pre_search: (PreSearch::new(), PreSearch::new()),
            mode: SearchPageMode::POPUP
        }
    }
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
                            if self.pre_search.0.$field.contains(i) {
                                check = "[+] ".to_owned();
                            } else if self.pre_search.1.$field.contains(i) {
                                check = "[-] ".to_owned();
                            } else {
                                check = "[ ] ".to_owned();
                            }
                            tabs.push(check + i) 
                        }

                    },)*
                }
            };
        }
        checks!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources);
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
    fn popup_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') => {
                match self.mode {
                    SearchPageMode::TITLE => self.mode = SearchPageMode::POPUP,
                    _ => self.mode = SearchPageMode::TITLE
                }
            },
            KeyCode::Char('j') => {
                if self.mode == SearchPageMode::POPUP { self.states[self.selected.clone() as usize].select_next() }
            }
            KeyCode::Char('k') => {
                if self.mode == SearchPageMode::POPUP { self.states[self.selected.clone() as usize].select_previous() }
            }
            KeyCode::Char('l') => {
                self.next_select()
            }
            KeyCode::Char('h') => {
                self.prev_select()
            }
            KeyCode::Enter => if self.mode == SearchPageMode::POPUP {match self.states[self.selected.clone() as usize].selected() {
                Some(0) => {
                    use SearchSelected::*;
                    macro_rules! all_off {
                        ($($varient:ident => $field:ident),*) => {
                            match self.selected {
                                $($varient => {self.pre_search.0.$field.clear(); self.pre_search.1.$field.clear();},)*
                            }
                        }
                    }
                    all_off!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources)
                },
                Some(1) => {
                    use SearchSelected::*;
                    macro_rules! all_on {
                        ($($varient:ident => $field:ident),*) => {
                            match self.selected {
                                $($varient => {
                                    self.pre_search.1.$field = self.spell_enums.$field.clone();
                                    self.pre_search.0.$field.clear();
                                },)*
                            }
                        }
                    }
                    all_on!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources);

                }
                Some(2) => {
                    use SearchSelected::*;
                    macro_rules! all_on {
                        ($($varient:ident => $field:ident),*) => {
                            match self.selected {
                                $($varient => {
                                    self.pre_search.0.$field = self.spell_enums.$field.clone();
                                    self.pre_search.1.$field.clear();
                                },)*
                            }
                        }
                    }
                    all_on!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources);
                },
                Some(n) => {
                    use SearchSelected::*;
                    macro_rules! toggle_one {
                        ($($varient:ident => $field:ident),*) => {
                            match self.selected {
                                $($varient => {
                                    let s = &self.spell_enums.$field[n-3];
                                    let indexes = (self.pre_search.0.$field.iter().position(|r| r == s), self.pre_search.1.$field.iter().position(|r| r == s));
                                    match indexes {
                                        (Some(n), None) => self.pre_search.1.$field.push(self.pre_search.0.$field.remove(n)),
                                        (None, Some(n)) => {let _ = self.pre_search.1.$field.remove(n);},
                                        (None, None) => self.pre_search.0.$field.push(s.clone()),
                                        _ => ()
                                    }

                                })*
                            }
                        }
                    }
                    toggle_one!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources);
                },
                _ => ()
            }}
            _ => ()
        }
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        //frame.render_widget(Paragraph::new(self.pre_search.school.join(" ")), layout);

        if self.mode == SearchPageMode::POPUP{
            let checked_tabs = self.get_checked();
            //let tags = List::new(self.spell_enums.tags.clone());
            let list = List::new(checked_tabs)
                .block(Block::bordered().title(String::from(self.selected.clone())))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(list, area, &mut self.states[self.selected.clone() as usize]);
        }
    }
    fn key(&mut self, key: KeyCode) {
        match self.mode {
            SearchPageMode::POPUP => self.popup_key(key),
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

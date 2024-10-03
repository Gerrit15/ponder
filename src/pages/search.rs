use ratatui::{prelude::{Constraint, Layout, Direction, Alignment}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    //popup: bool,
    selected: SearchSelected,
    states: Vec<ListState>,
    pre_search: PreSearch,
    mode: SearchPageMode,
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            //popup: false,
            states: vec![ListState::default(); 9],
            selected: SearchSelected::SCHOOL,
            pre_search: PreSearch::new(),
            mode: SearchPageMode::NONE,
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
                            if self.pre_search.$field.0.contains(i) {
                                check = "[+] ".to_owned();
                            } else if self.pre_search.$field.1.contains(i) {
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
            KeyCode::Enter => self.mode = SearchPageMode::NONE,
            KeyCode::Char('j') => self.states[self.selected.clone() as usize].select_next(),
            KeyCode::Char('k') => self.states[self.selected.clone() as usize].select_previous(),
            KeyCode::Char('l') => self.next_select(),
            KeyCode::Char('h') => self.prev_select(),
            KeyCode::Char(' ')=> match self.states[self.selected.clone() as usize].selected() {
                Some(0) => {
                    use SearchSelected::*;
                    macro_rules! all_off {
                        ($($varient:ident => $field:ident),*) => {
                            match self.selected {
                                $($varient => {self.pre_search.$field.0.clear(); self.pre_search.$field.1.clear();},)*
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
                                    self.pre_search.$field.1 = self.spell_enums.$field.clone();
                                    self.pre_search.$field.0.clear();
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
                                    self.pre_search.$field.0 = self.spell_enums.$field.clone();
                                    self.pre_search.$field.1.clear();
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
                                    let indexes = (self.pre_search.$field.0.iter().position(|r| r == s), self.pre_search.$field.1.iter().position(|r| r == s));
                                    match indexes {
                                        (Some(n), None) => self.pre_search.$field.1.push(self.pre_search.$field.0.remove(n)),
                                        (None, Some(n)) => {let _ = self.pre_search.$field.1.remove(n);},
                                        (None, None) => self.pre_search.$field.0.push(s.clone()),
                                        _ => ()
                                    }

                                })*
                            }
                        }
                    }
                    toggle_one!(TAGS => tags, SCHOOL => school, CASTINGUNITS => casting_units, SHAPES => shapes, LISTS => lists, PROCEFF => proc_eff, PROCSAVE => proc_save, DMGTYPE => damage_types, SOURCES => sources);
                },
                _ => ()
            }
            _ => ()
        }
    }
    fn title_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.pre_search.title += &c.to_string(),
            KeyCode::Delete | KeyCode::Backspace => {if self.pre_search.title.len() != 0 {self.pre_search.title.remove(self.pre_search.title.len()-1);};},
            KeyCode::Esc | KeyCode::Enter => {self.mode = SearchPageMode::NONE},
            _ => ()
        }
    }
    fn none_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('t') => {self.mode = SearchPageMode::TITLE},
            KeyCode::Char('p') => {self.mode = SearchPageMode::POPUP},
            _ => ()
        }
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        let area = popup_area(layout, 60, 20);
        let bar = match self.mode {
            SearchPageMode::TITLE => "|",
            _ => ""
        };
        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(7),
                Constraint::Percentage(7),
                Constraint::Percentage(5),
                Constraint::Min(0),
            ]).split(layout);
        let top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(35),
                Constraint::Min(0)
            ]).split(inner_layout[0]);
        let mid_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(6),
                Constraint::Percentage(6),
                Constraint::Percentage(6),
                Constraint::Percentage(8),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(7),
                Constraint::Percentage(15),
                Constraint::Min(0),
            ]).split(inner_layout[1]);
        let mid_low_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(31),
                Constraint::Percentage(18),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(12),
                Constraint::Percentage(31)
            ]).split(inner_layout[2]);

        frame.render_widget(Paragraph::new("Title: ".to_string() + &self.pre_search.title.clone() + bar).block(Block::bordered()), top_row[0]);
        frame.render_widget(Paragraph::new("Content: ".to_string()).block(Block::bordered()), top_row[1]);

        frame.render_widget(Paragraph::new("V: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[0]);
        frame.render_widget(Paragraph::new("S: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[1]);
        frame.render_widget(Paragraph::new("M: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[2]);
        frame.render_widget(Paragraph::new("Ritual: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[3]);
        frame.render_widget(Paragraph::new("Component Cost: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[4]);
        frame.render_widget(Paragraph::new("Higher Level: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[5]);
        frame.render_widget(Paragraph::new("Concentration: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[6]);
        frame.render_widget(Paragraph::new("Level: [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[7]);
        frame.render_widget(Paragraph::new("Damage: [ ]D[ ] + [ ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[8]);
        frame.render_widget(Paragraph::new("Duration: [ ] [       ] ".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_row[9]);


        frame.render_widget(Paragraph::new("Casting Time: [       ] [ ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[1]);
        frame.render_widget(Paragraph::new("Range: [ ] [ ] [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[2]);
        frame.render_widget(Paragraph::new("Proc: [       ] [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[3]);
        frame.render_widget(Paragraph::new("Source: [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[4]);

        if self.mode == SearchPageMode::POPUP{
            let checked_tabs = self.get_checked();
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
            SearchPageMode::NONE => self.none_key(key),
            SearchPageMode::POPUP => self.popup_key(key),
            SearchPageMode::TITLE => self.title_key(key),
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

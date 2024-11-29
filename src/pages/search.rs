use ratatui::{prelude::{Constraint, Layout, Direction, Alignment}, layout::Flex, widgets::{Clear, ListState, List}, style::{Style, Modifier}};

use crate::*;

pub struct Search {
    spell_enums: SpellEnums,
    //popup: bool,
    damage_type_selector: ListState,
    //states: Vec<ListState>,
    pre_search: PreSearch,
    selected: SearchSelected,
    duration_pop: (bool, ListState),
}

impl Search {
    pub fn new(spell_enums: SpellEnums) -> Search {
        Search {
            spell_enums,
            //popup: false,
            //states: vec![ListState::default(); 9],
            damage_type_selector: ListState::default(),
            selected: SearchSelected::NONE,
            pre_search: PreSearch::new(),
            duration_pop: (false, ListState::default()),
        }
    }
    fn get_checked(&self, selected: SearchSelected) -> Vec<String> {
        let mut tabs: Vec<String> = vec![];
        tabs.push("[ CLEAR ]".to_owned());
        tabs.push("[ NONE ]".to_owned());
        tabs.push("[ ALL ]".to_owned());

        use SearchSelected::*;
        macro_rules! checks {
            ($($variant:ident => $field:ident),*) => {
                match selected {
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
                    NONE => (),
                    _ => ()
                }
            };
        }
        checks!(TAGS => tags, DMGTYPE => damage_types, LISTS => lists);
        return tabs
    }

/*
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
*/
    fn none_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => self.selected = SearchSelected::TITLE,
            _ => ()
        }
    }
    fn title_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.pre_search.title += &c.to_string(),
            KeyCode::Delete | KeyCode::Backspace => {if self.pre_search.title.len() != 0 {self.pre_search.title.remove(self.pre_search.title.len()-1);};},
            KeyCode::Esc | KeyCode::Enter => {self.selected = SearchSelected::NONE},
            KeyCode::Tab => self.selected = SearchSelected::CONTENT,
            _ => ()
        }
    }
    fn content_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.pre_search.content += &c.to_string(),
            KeyCode::Delete | KeyCode::Backspace => {if self.pre_search.content.len() != 0 {self.pre_search.content.remove(self.pre_search.content.len()-1);};},
            KeyCode::Esc | KeyCode::Enter => {self.selected = SearchSelected::NONE},
            KeyCode::Tab => self.selected = SearchSelected::V,
            _ => ()
        }
    }

    fn lv_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                match c.to_digit(10) {
                    Some(n) => {
                        self.pre_search.lv = Some(n)
                    },
                    _ => ()
                }
            },
            KeyCode::Delete | KeyCode::Backspace => self.pre_search.lv = None,
            KeyCode::Esc | KeyCode::Enter => {self.selected = SearchSelected::NONE},
            KeyCode::Tab => self.selected = SearchSelected::DAMAGE(1),
            _ => ()
        }
    }

    fn damage_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                if let Some(n) = c.to_digit(10) {
                    if let SearchSelected::DAMAGE(i) = self.selected {
                        self.pre_search.damage[i as usize - 1] = Some(n)
                    }
                }
            },
            KeyCode::Delete | KeyCode::Backspace => {
                if  let SearchSelected::DAMAGE(i) = self.selected {
                    self.pre_search.damage[i as usize - 1] = None
                }
            },
            KeyCode::Esc => {self.selected = SearchSelected::NONE},
            KeyCode::Tab => {
                if let SearchSelected::DAMAGE(i) = self.selected {
                    if i != 3 {
                        self.selected = SearchSelected::DAMAGE(i + 1)
                    } else {
                        self.selected = SearchSelected::DURATION(0)
                    }
                }
            },
            _ => ()
        }
    }

    fn duration_key(&mut self, key:KeyCode) {
        match key {
            KeyCode::Char(c) => {
                //we don't let it go above 3 digits
                if let (SearchSelected::DURATION(0), Some(n)) = (self.selected.clone(), c.to_digit(10)) {
                    let n_val = self.pre_search.duration.0.unwrap_or(0)*10 + n;
                    let n_val = n_val - (n_val / 100)*100;
                    self.pre_search.duration.0 = Some(n_val);
                } else if c == 'j' {
                    self.duration_pop.1.select_next();
                } else if c == 'k' {
                    self.duration_pop.1.select_previous();
                }
            },
            KeyCode::Delete | KeyCode::Backspace => {
                self.pre_search.duration.0 = None;
                self.pre_search.duration.1 = None;
            },
            KeyCode::Esc => {
                match self.selected {
                    SearchSelected::DURATION(1) => {
                        if self.duration_pop.0 {
                            self.duration_pop.0 = false;
                        } else {
                            self.selected = SearchSelected::NONE;
                        }
                    },
                    _ => self.selected = SearchSelected::NONE
                };
            },
            KeyCode::Enter => {
                if let SearchSelected::DURATION(1) = self.selected {
                    if self.duration_pop.0 {
                        let selected = self.duration_pop.1.selected().unwrap_or(0);
                        //self.pre_search.duration.1 = Some(String::from(SearchSelected::from_usize(selected).unwrap()));
                        self.pre_search.duration.1 = Some(self.spell_enums.casting_units[selected].clone());
                        if self.pre_search.duration.1.clone().unwrap() == "NONE" {
                            self.pre_search.duration.1 = None;
                        }
                        self.duration_pop.0 = false;
                    } else {
                        self.duration_pop.0 = true;
                    }
                }
            },
            KeyCode::Tab => {
                match self.selected {
                    SearchSelected::DURATION(0) => self.selected = SearchSelected::DURATION(1),
                    SearchSelected::DURATION(1) => {
                        self.duration_pop.0 = false;
                        self.selected = SearchSelected::TITLE;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn castingtime_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {},
            KeyCode::Delete | KeyCode::Backspace => {},
            KeyCode::Esc => {},
            KeyCode::Tab => {
                self.selected = SearchSelected::TITLE
            },
            _ => ()
        }
    }
}

macro_rules! impl_boolkey {
    ($( ($fn_name: ident, $next: ident, $($field: tt)+ ) ), *) => {
        impl Search {
            $(fn $fn_name(&mut self, key:KeyCode) {
                match key {
                    KeyCode::Enter => {
                        match &self.pre_search.$($field)+ {
                            Some(b) => {
                                if *b {self.pre_search.$($field)+ = Some(false)}
                                else {self.pre_search.$($field)+ = None}
                            },
                            None => {self.pre_search.$($field)+ = Some(true)}
                            }
                        },
                    KeyCode::Esc => {self.selected = SearchSelected::NONE},
                    KeyCode::Tab => self.selected = SearchSelected::$next,
                    _ => ()
                }
            })*
        }
    };
}

impl_boolkey!(
    (v_key, S, vsm.0),
    (s_key, M, vsm.1),
    (m_key, RITUAL, vsm.2),
    (ritual_key, COMPONENT, ritual),
    (cost_key, HIGHERLV, component_cost),
    (higher_lv_key, CONCENTRATION, higher_lv),
    (concentration_key, LEVEL, concentration)
);

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        //let area = popup_area(layout, 60, 20);
        let title_content = {
            "Level: ".to_string() + &self.pre_search.title + match self.selected {
                SearchSelected::TITLE => "|",
                _ => ""
            }
        };

        let content_content = {
            "Content: ".to_string() + &self.pre_search.content + match self.selected {
                SearchSelected::CONTENT=> "|",
                _ => ""
            }
        };

        let lv_content = {
            let prefix = match self.selected {
                SearchSelected::LEVEL => "Level [|",
                _ => "Level ["
            };
            let mid = match self.pre_search.lv {
                Some(n) => &n.to_string(),
                _ => " "
            };
            prefix.to_string() + mid + "]"
        };

        let damage_content = {
            let s = "Damage: [".to_string();
            let dmg_strs = self.pre_search.damage.iter().map(|v| 
                match v {
                    &Some(n) => n.clone().to_string(),
                    _ => " ".to_string()
                }
            ).collect::<Vec<String>>();
            match self.selected {
                SearchSelected::DAMAGE(1) => s + "|" + &dmg_strs[0].clone() + "]D[" + &dmg_strs[1].clone() + "] + [" + &dmg_strs[2].clone() + "]",
                SearchSelected::DAMAGE(2) => s + &dmg_strs[0].clone() + "]D[|" + &dmg_strs[1].clone() + "] + [" + &dmg_strs[2].clone() + "]",
                SearchSelected::DAMAGE(3) => s + &dmg_strs[0].clone() + "]D[" + &dmg_strs[1].clone() + "] + [|" + &dmg_strs[2].clone() + "]",
                _ => s + &dmg_strs[0].clone() + "]D[" + &dmg_strs[1].clone() + "] + [" + &dmg_strs[2].clone() + "]",
            }
        };

        let duration_content = {
            let s = "Duration [".to_string();
            let n = match self.pre_search.duration.0 {
                Some(num) => num.to_string(),
                _ => " ".to_owned()
            };
            let len = self.pre_search.duration.1.clone().unwrap_or("    ".to_string());
            match self.selected {
                SearchSelected::DURATION(0) => {s + "|" + &n + "] [" + &len + "]"}
                SearchSelected::DURATION(1) => {s + &n + "] [|" + &len + "]"}
                _ => {s + &n + " ] [" + &len + "]"}
            }
        };

        macro_rules! toggle_content {
            ($var: ident = $str:literal, $($field:tt)+) => {
                 let $var = match self.pre_search.$($field)+ {
                    Some(b) => {
                        if b {$str.to_owned() + ": [+]"}
                        else {$str.to_owned() + ": [-]"}
                    },
                    None => $str.to_owned() + ": [ ]"
                };
            };
        }
        toggle_content!(v_content = "V", vsm.0);
        toggle_content!(s_content = "S", vsm.1);
        toggle_content!(m_content = "M", vsm.2);
        toggle_content!(ritual_content = "Ritual", ritual);
        toggle_content!(component_cost = "Component Cost", component_cost);
        toggle_content!(higher_lv = "Higher Level", higher_lv);
        toggle_content!(concentration = "Concentration", concentration);

        macro_rules! lists {
            ($var: ident, $type: ident) => {
                let $var = List::new(self.get_checked(SearchSelected::$type))
                    .block(get_style(&self.selected, SearchSelected::$type, true))
                    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                    .highlight_symbol(">")
                    .repeat_highlight_symbol(true);
            }
        }

        lists!(damage_list, DMGTYPE);
        lists!(tag_list, TAGS);
        lists!(spell_list, LISTS);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(7),
                Constraint::Percentage(7),
                Constraint::Percentage(7),
                Constraint::Percentage(55),
                Constraint::Percentage(7),
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
                Constraint::Percentage(14),
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
        let select_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]).split(inner_layout[3]);
        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Min(0)
            ]).split(inner_layout[4]);


        macro_rules! widget_render {
            ($( ($var: tt, ($($style_select: tt)+), $($location: tt)+) ), *) => {
                $(
                    let tempn = usize::from(SearchSelected::$($style_select)+.clone());

                    let style = {
                        let b = Block::bordered();
                        if usize::from(self.selected.clone()) == usize::from(SearchSelected::$($style_select)+.clone()) {
                            b.style(Style::default().add_modifier(Modifier::REVERSED))
                        } else {
                            b
                        }
                    };
                    if tempn <= 2 && tempn != 0 {
                        frame.render_widget(Paragraph::new($var.to_string()).alignment(Alignment::Left).block(style), $($location)+);
                    } else {
                        frame.render_widget(Paragraph::new($var.to_string()).alignment(Alignment::Center).block(style), $($location)+);
                    }
                )*
            };
        }


        widget_render!(
            (title_content, (TITLE), top_row[0]),
            (content_content, (CONTENT), top_row[1]),
            (v_content, (V), mid_row[0]),
            (s_content, (S), mid_row[1]),
            (m_content, (M), mid_row[2]),
            (ritual_content, (RITUAL), mid_row[3]),
            (component_cost, (COMPONENT), mid_row[4]),
            (higher_lv, (HIGHERLV), mid_row[5]),
            (concentration, (CONCENTRATION), mid_row[6]),
            (lv_content, (LEVEL), mid_row[7]),
            //the (0) is ok here because any value of DAMAGE(*) turns into the same usize
            (damage_content, (DAMAGE(0)), mid_row[8]),
            (duration_content, (DURATION(0)), mid_row[9])
        );

        frame.render_widget(Paragraph::new("Casting Time: [       ] [ ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[1]);
        frame.render_widget(Paragraph::new("Range: [ ] [ ] [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[2]);
        frame.render_widget(Paragraph::new("Proc: [       ] [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[3]);
        frame.render_widget(Paragraph::new("Source: [       ]".to_string()).alignment(Alignment::Center).block(Block::bordered()), mid_low_row[4]);

        frame.render_stateful_widget(damage_list, select_row[0], &mut self.damage_type_selector);
        frame.render_stateful_widget(tag_list, select_row[1], &mut self.damage_type_selector);
        frame.render_stateful_widget(spell_list, select_row[2], &mut self.damage_type_selector);

        frame.render_widget(Paragraph::new("SEARCH").centered().block(Block::bordered()), bottom_row[0]);
        frame.render_widget(Paragraph::new("CLEAR").centered().block(Block::bordered()), bottom_row[1]);

        if self.duration_pop.0 {
            let area = popup_area(layout, 60, 20);
            let list = List::new(self.spell_enums.casting_units.clone())
                .block(Block::bordered().title(String::from("Duration units")))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(list, area, &mut self.duration_pop.1);
        }

        /*if self.mode == SearchPageMode::POPUP{
            let checked_tabs = self.get_checked();
            let list = List::new(checked_tabs)
                .block(Block::bordered().title(String::from(self.selected.clone())))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">")
                .repeat_highlight_symbol(true);
            frame.render_widget(Clear, area);
            frame.render_stateful_widget(list, area, &mut self.states[self.selected.clone() as usize]);
        }*/
    }
    fn key(&mut self, key: KeyCode) {
        match self.selected {
            SearchSelected::NONE => self.none_key(key),
            SearchSelected::TITLE => self.title_key(key),
            SearchSelected::CONTENT => self.content_key(key),
            SearchSelected::V => self.v_key(key),
            SearchSelected::S => self.s_key(key),
            SearchSelected::M => self.m_key(key),
            SearchSelected::RITUAL => self.ritual_key(key),
            SearchSelected::COMPONENT => self.cost_key(key),
            SearchSelected::HIGHERLV => self.higher_lv_key(key),
            SearchSelected::CONCENTRATION => self.concentration_key(key),
            SearchSelected::LEVEL => self.lv_key(key),
            SearchSelected::DAMAGE(_) => self.damage_key(key),
            SearchSelected::DURATION(_) => self.duration_key(key),
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

fn get_style(selected: &SearchSelected, cur: SearchSelected, titled: bool) -> Block {
    let mut b = Block::bordered();
    if titled {
        b = b.title(String::from(cur.clone()))
    }
    if usize::from(selected.clone()) == usize::from(cur) {
        return b.style(Style::default().add_modifier(Modifier::REVERSED))
    }
    return b
}

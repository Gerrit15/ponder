use ratatui::style::Style;

/*#[derive(Clone)]
pub enum SearchSelected {
    NONE,
    SOURCES,
    SCHOOL,
    CASTINGUNITS,
    SHAPES,
    LISTS,
    PROCEFF,
    PROCSAVE,
    DMGTYPE,
    TAGS,
}*/

#[derive(Clone)]
pub enum SearchSelected {
    NONE,
    TITLE,
    CONTENT,
    V,
    S,
    M,
    RITUAL,
    COMPONENT,
    HIGHERLV,
    CONCENTRATION,
    LEVEL,
    DAMAGE,
    DURATION,
    CASTINGTIME,
    RANGE,
    PROC,
    SOURCE,
    DMGTYPE,
    TAGS,
    LISTS,
    SEARCH,
    CLEAR
}

impl SearchSelected {
    pub fn from_usize (value: usize) -> Option<SearchSelected> {
        use SearchSelected::*;
        match value {
            0 => Some(NONE),
            1 => Some(TITLE),
            2 => Some(CONTENT),
            3 => Some(V),
            4 => Some(S),
            5 => Some(M),
            6 => Some(RITUAL),
            7 => Some(COMPONENT),
            8 => Some(HIGHERLV),
            9 => Some(CONCENTRATION),
            10 => Some(LEVEL),
            11 => Some(DAMAGE),
            12 => Some(DURATION),
            13 => Some(CASTINGTIME),
            14 => Some(RANGE),
            15 => Some(PROC),
            16 => Some(SOURCE),
            17 => Some(DMGTYPE),
            18 => Some(TAGS),
            19 => Some(LISTS),
            20 => Some(SEARCH),
            21 => Some(CLEAR),
            _ => None
        }
    }
}

impl From<SearchSelected> for usize {
    fn from(value: SearchSelected) -> Self {
        use SearchSelected::*;
        match value {
            NONE => 0,
            TITLE => 1,
            CONTENT=> 2,
            V => 3,
            S => 4,
            M => 5,
            RITUAL => 6,
            COMPONENT => 7,
            HIGHERLV => 8,
            CONCENTRATION=> 9,
            LEVEL => 10,
            DAMAGE => 11,
            DURATION => 12,
            CASTINGTIME => 13,
            RANGE => 14,
            PROC => 15,
            SOURCE => 16,
            DMGTYPE => 17,
            TAGS => 18,
            LISTS => 19,
            SEARCH => 20,
            CLEAR => 21,
        }
    }
}

impl From<SearchSelected> for String {
    fn from(value: SearchSelected) -> Self {
        use SearchSelected::*;
        let s = match value {
            NONE => "None",
            TITLE => "Title",
            CONTENT=> "Content",
            V => "V",
            S => "S",
            M => "M",
            RITUAL => "Ritual",
            COMPONENT => "Component Cost",
            HIGHERLV => "Higher Level",
            CONCENTRATION=> "Concentration",
            LEVEL => "Level",
            DAMAGE => "Damage",
            DURATION => "Duration",
            CASTINGTIME => "Casting Time",
            RANGE => "Range",
            PROC => "Proc",
            SOURCE => "Source",
            DMGTYPE => "Damage Type",
            TAGS => "Tags",
            LISTS => "Lists",
            SEARCH => "Search",
            CLEAR => "Clear",
        };
        return s.to_string()
    }
}

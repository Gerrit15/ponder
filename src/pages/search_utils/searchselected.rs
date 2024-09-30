#[derive(Clone)]
pub enum SearchSelected {
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
    pub fn from_usize (value: usize) -> Option<SearchSelected> {
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

impl From<SearchSelected> for String {
    fn from(value: SearchSelected) -> Self {
        use SearchSelected::*;
        let s = match value {
            SOURCES => "Sources",
            SCHOOL => "School",
            CASTINGUNITS => "Casting Units",
            SHAPES => "Shapes",
            LISTS => "Lists",
            PROCEFF => "Proc Effect",
            PROCSAVE => "Proc Save",
            DMGTYPE => "Damage Types",
            TAGS => "Tags"
        };
        return s.to_string()
    }
}

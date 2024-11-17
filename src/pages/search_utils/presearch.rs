pub struct PreSearch {
    pub sources: (Vec<String>, Vec<String>),
    pub school: (Vec<String>, Vec<String>),
    pub casting_units: (Vec<String>, Vec<String>),
    pub shapes: (Vec<String>, Vec<String>),
    pub lists: (Vec<String>, Vec<String>),
    pub proc_eff: (Vec<String>, Vec<String>),
    pub proc_save: (Vec<String>, Vec<String>),
    pub damage_types: (Vec<String>, Vec<String>),
    pub tags: (Vec<String>, Vec<String>),
    pub title: String,
    pub content: String,
    pub vsm: (Option<bool>, Option<bool>, Option<bool>),
    pub ritual: Option<bool>,
}

impl PreSearch {
    pub fn new() -> PreSearch{
        PreSearch { 
            sources: (vec![], vec![]), 
            school: (vec![], vec![]), 
            casting_units: (vec![], vec![]), 
            shapes: (vec![], vec![]), 
            lists: (vec![], vec![]), 
            proc_eff: (vec![], vec![]), 
            proc_save: (vec![], vec![]), 
            damage_types: (vec![], vec![]), 
            tags: (vec![], vec![]),
            title: String::new(),
            content: String::new(),
            vsm: (None, None, None),
            ritual: None,
        }
    }
}

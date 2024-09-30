pub struct PreSearch {
    pub sources: Vec<String>,
    pub school: Vec<String>,
    pub casting_units: Vec<String>,
    pub shapes: Vec<String>,
    pub lists: Vec<String>,
    pub proc_eff: Vec<String>,
    pub proc_save: Vec<String>,
    pub damage_types: Vec<String>,
    pub tags: Vec<String>,
    pub title: String,
}

impl PreSearch {
    pub fn new() -> PreSearch{
        PreSearch { 
            sources: vec![], 
            school: vec![], 
            casting_units: vec![], 
            shapes: vec![], 
            lists: vec![], 
            proc_eff: vec![], 
            proc_save: vec![], 
            damage_types: vec![], 
            tags: vec![],
            title: String::new(),
        }
    }
}

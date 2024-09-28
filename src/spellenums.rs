use crate::*;

#[derive(Debug)]
pub struct SpellEnums {
    pub sources: Vec<String>,
    pub school: Vec<String>,
    pub casting_units: Vec<String>,
    pub shapes: Vec<String>,
    pub lists: Vec<String>,
    pub proc_eff: Vec<String>,
    pub proc_save: Vec<String>,
    pub damage_types: Vec<String>,
    pub tags: Vec<String>
}

impl SpellEnums {
    pub fn new() -> SpellEnums{
        SpellEnums { 
            sources: vec![], 
            school: vec![], 
            casting_units: vec![], 
            shapes: vec![], 
            lists: vec![], 
            proc_eff: vec![], 
            proc_save: vec![], 
            damage_types: vec![], 
            tags: vec![] 
        }
    }
    pub fn update(&mut self, spell: &Spell) {
        if !self.sources.contains(&spell.source) {self.sources.push(spell.source.clone())}
        if !self.school.contains(&spell.school) {self.school.push(spell.school.clone())}
        if !self.casting_units.contains(&spell.casting_time.1) {self.casting_units.push(spell.casting_time.1.clone())}
        if !self.shapes.contains(&spell.range.2) {self.shapes.push(spell.range.2.clone())}
        for j in &spell.spell_lists {if !self.lists.contains(&j) {self.lists.push(j.clone())}}
        if !self.proc_eff.contains(&spell.proc.0) {self.proc_eff.push(spell.proc.0.clone())}
        if !self.proc_save.contains(&spell.proc.1) {self.proc_save.push(spell.proc.1.clone())}
        for j in &spell.damage.3 {if !self.damage_types.contains(&j) {self.damage_types.push(j.clone())}}
        for j in &spell.tags {if !self.tags.contains(&j) {self.tags.push(j.clone())}}
    }
    pub fn toggle_tag(&mut self, s:  &String) {
        let index = self.tags.iter().position(|r| r == s);
        match index {
            Some(n) => {let _ = self.tags.remove(n);},
            None => self.tags.push(s.clone())
        };
    }
}

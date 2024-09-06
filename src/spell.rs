use crate::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Spell {
    pub title: String, 
    pub source: String,
    pub lv: u32,
    pub school: String,
    pub ritual: bool,
    //time length, time type, (v,s,m)
    pub casting_time: (u32, String, (bool, bool, bool)),
    pub component_cost: bool,
    //range, size of aoe, measurement (ft) type
    pub range: (u32, u32, String),
    //last one is for concentration
    pub duration: (u32, String, bool),
    pub text: String,
    pub higher_lv: String, 
    pub spell_lists: Vec<String>,
    //"Instant", "Save", "Ranged Spell Attack", "Melee Attack" and "Dextarity", "Strength", etc
    pub proc: (String, String),
    //# dice, dice type, added damage, damage type(s)
    pub damage: (u32, u32, u32, Vec<String>),
    pub tags: Vec<String>,
}

impl Spell {
    pub fn new_from_json(spell_path: PathBuf) -> Result<Spell, (String, PathBuf)> {
        let spell_file = fs::read_to_string(spell_path.clone());
        match spell_file {
            Ok(x) => {
                let spell: Result<Spell, Error> = serde_json::from_str(&x);
                match spell {
                    Ok(x) => Ok(x),
                    Err(x) => Err((x.to_string(), spell_path))
                }
            },
            Err(x) => Err((x.to_string(), spell_path)),
        }
    }
    pub fn values(&self) -> String {
        let delim = "', '";
        let spell_list = {
            let mut s = String::new();
            for i in &self.spell_lists {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let types = {
            let mut s = String::new();
            for i in &self.damage.3 {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let tags = {
            let mut s = String::new();
            for i in &self.tags {s.push_str(&(" ".to_owned() + i))}
            s
        };
        let title = self.title.clone().replace("'", "''");
        let source = self.source.clone().replace("'", "''");
        let text = self.source.clone().replace("'", "''");
        let higher_lv = self.source.clone().replace("'", "''");
        let s = "'".to_string() + &title + delim + &source + "', "+ &self.lv.to_string() + ", '"+ &self.school + "', "+ &(self.ritual as u32).to_string() + ", "+ &self.casting_time.0.to_string() + ", '"+ &self.casting_time.1 + "', "+ &(self.casting_time.2.0 as u32).to_string() + ", "+ &(self.casting_time.2.1 as u32).to_string() + ", "+ &(self.casting_time.2.2 as u32).to_string() + ", " + &(self.component_cost as u32).to_string() + ", "+ &self.range.0.to_string() + ", "+ &self.range.1.to_string() + ", '"+ &self.range.2 + delim + &text + delim + &higher_lv + delim + &spell_list + delim + &self.proc.0 + delim + &self.proc.1 + "', "+ &self.damage.0.to_string() + ", "+ &self.damage.1.to_string() + ", " + &self.damage.2.to_string() + ", '"+ &types + delim + &tags + "' ";
        return s
    }
    pub fn load_spells(path_str: &str) -> HashMap<String, Spell> {
        let mut path = std::path::PathBuf::new();
        path.push(path_str);
        let mut spells = HashMap::new();
        match fs::read_dir(path) {
            Ok(entries) => {
                let x = entries.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>();
                match x {
                    Ok(mut x) => {
                        x.sort();
                        for spell_entry_buff in x {
                            let spell_entries = fs::read_dir(spell_entry_buff);
                            match spell_entries {
                                Ok(entries) => {
                                    let x = entries.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
                                    for i in x {
                                        let spell = Spell::new_from_json(i);
                                        match spell {
                                            Ok(okspell) => {let _ = spells.insert(okspell.title.clone(), okspell);},
                                            Err(e) => println!("Final load error: {}, {}", e.0, e.1.to_str().unwrap())
                                        }
                                    }
                                },
                                Err(_e) => panic!("Error in loading entries"),
                            }
                        }
                    },
                    Err(_e) => panic!("Error in reading directories")
                }
            },
            Err(_e) => panic!("Error in reading path")
        }
        return spells
    }
}

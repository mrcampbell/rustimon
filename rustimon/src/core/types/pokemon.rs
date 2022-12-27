#[derive(Clone, Debug)]
pub struct Pokemon {
  pub id: String,
  pub level: u32,
  pub species: Species,
  pub nickname: Option<String>,
  pub iv: StatGroup,
  pub ev: StatGroup,
  pub stats: StatGroup,
}

impl Pokemon {
  pub fn new(id: String, level: u32, species: Species, nickname: Option<String>, iv: StatGroup, ev: StatGroup) -> Pokemon {
    Pokemon {
      id: id,
      level: level,
      species: species.clone(),
      nickname: nickname,
      iv: iv.clone(),
      ev: ev.clone(),
      stats: calculate_stats(species, level, iv, ev),
    }
  }
}
  fn calculate_stats(species: Species, level: u32, iv: StatGroup, ev: StatGroup) -> StatGroup {
    StatGroup { hp:1, atk: 2, def: 3, spec_atk: 4, spec_def: 5, spd: 6 }
  }

#[derive(Clone, Debug)]
pub struct Species {
  pub id: u32,
  pub name: String,
  pub stats: StatGroup,
}

#[derive(Clone, Debug)]
pub struct StatGroup {
  pub hp: u32,
  pub atk: u32,
  pub def: u32,
  pub spec_atk: u32,
  pub spec_def: u32,
  pub spd: u32,
}
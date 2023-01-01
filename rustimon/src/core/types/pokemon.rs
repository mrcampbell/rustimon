use std::ops::Div;

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
    pub fn new(
        id: String,
        level: u32,
        species: Species,
        nickname: Option<String>,
        iv: StatGroup,
        ev: StatGroup,
    ) -> Pokemon {
        Pokemon {
            id,
            level,
            nickname,
            iv: iv.clone(),
            ev: ev.clone(),
            species: species.clone(),
            stats: calculate_stats( level, species.stats, iv, ev),
        }
    }
}
fn calculate_stats(level: u32, species: StatGroup, iv: StatGroup, ev: StatGroup) -> StatGroup {
    StatGroup {
        hp: 1,
        atk: 2,
        def: 3,
        spec_atk: 4,
        spec_def: 5,
        spd: 6,
    }
}

// using Gen-III-Onward formula
fn calculate_hp(level: f32, species: f32, iv: f32, ev: f32) -> f32 {
 (((2.0 * species + iv + ev.div(4.0).floor()) * level).floor()/100.0).floor() + level + 10.0
}

// scenario from here: https://bulbapedia.bulbagarden.net/wiki/Stat
#[test]
fn test_calculate_hp() {
  assert_eq!(calculate_hp(78.0, 108.0, 24.0, 74.0), 289.0);
}

// using Gen-III-Onward formula
fn calculate_stat(level: f32, species: f32, iv: f32, ev: f32) -> f32 {
  ((((2.0 * species + iv + ev.div(4.0).floor()) * level).floor()/100.0) + 5.0 /* todo: mulitply by nature, etc. */).floor()
}

// scenario from here: https://bulbapedia.bulbagarden.net/wiki/Stat
#[test]
fn test_calculate_atk() {
  assert_eq!(calculate_stat(78.0, 102.0, 5.0, 23.0), 171.0);
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

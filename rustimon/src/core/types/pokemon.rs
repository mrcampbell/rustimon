use super::stat_group::StatGroup;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::ops::Div;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub id: String,
    pub level: u32,
    pub species: Species,
    pub iv: StatGroup,
    pub ev: StatGroup,
    pub stats: StatGroup,
}

impl Pokemon {
    pub fn new(
        seed: Option<u64>,
        id: String,
        level: u32,
        species: Species,
        iv: Option<StatGroup>,
        ev: Option<StatGroup>,
    ) -> Pokemon {
        let mut rng  = StdRng::seed_from_u64(seed.unwrap_or(0));

        let iv = iv.unwrap_or(StatGroup {
            hp: rng.gen_range(0..31),
            atk: rng.gen_range(0..31),
            def: rng.gen_range(0..31),
            spec_atk: rng.gen_range(0..31),
            spec_def: rng.gen_range(0..31),
            speed: rng.gen_range(0..31),
        });

        let ev = ev.unwrap_or(StatGroup {
            hp: 0,
            atk: 0,
            def: 0,
            spec_atk: 0,
            spec_def: 0,
            speed: 0,
        });

        let stats = Self::calculate_stats(level, species.stats.clone(), &iv, &ev);

        Pokemon {
            id,
            level,
            iv,
            ev,
            species: species.clone(),
            stats,
        }
    }

    fn calculate_stats(
        level: u32,
        species: StatGroup,
        iv: &StatGroup,
        ev: &StatGroup,
    ) -> StatGroup {
        StatGroup {
            hp: Self::calculate_hp(level as f32, species.hp as f32, iv.hp as f32, ev.hp as f32)
                as u32,
            atk: Self::calculate_stat(
                level as f32,
                species.atk as f32,
                iv.atk as f32,
                ev.atk as f32,
            ) as u32,
            def: Self::calculate_stat(
                level as f32,
                species.def as f32,
                iv.def as f32,
                ev.def as f32,
            ) as u32,
            spec_atk: Self::calculate_stat(
                level as f32,
                species.spec_atk as f32,
                iv.spec_atk as f32,
                ev.spec_atk as f32,
            ) as u32,
            spec_def: Self::calculate_stat(
                level as f32,
                species.spec_def as f32,
                iv.spec_def as f32,
                ev.spec_def as f32,
            ) as u32,
            speed: Self::calculate_stat(
                level as f32,
                species.speed as f32,
                iv.speed as f32,
                ev.speed as f32,
            ) as u32,
        }
    }

    // using Gen-III-Onward formula
    pub(crate) fn calculate_hp(level: f32, species: f32, iv: f32, ev: f32) -> f32 {
        (((2.0 * species + iv + ev.div(4.0).floor()) * level).floor() / 100.0).floor()
            + level
            + 10.0
    }

    // using Gen-III-Onward formula
    pub(crate) fn calculate_stat(level: f32, species: f32, iv: f32, ev: f32) -> f32 {
        ((((2.0 * species + iv + ev.div(4.0).floor()) * level).floor()/100.0) + 5.0 /* todo: mulitply by nature, etc. */).floor()
    }
}
#[derive(Clone, Debug)]
pub struct Species {
    pub id: u32,
    pub name: String,
    pub stats: StatGroup,
}

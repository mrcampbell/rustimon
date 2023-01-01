pub enum Element {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub const SUPER_EFFECTIVE_MULTIPLIER: f32 = 2.0;
pub const NOT_VERY_EFFECTIVE_MULTIPLIER: f32 = 0.5;
pub const NO_EFFECT_MULTIPLIER: f32 = 0.0;
pub const NORMALLY_EFFECTIVE_MULTIPLIER: f32 = 1.0;


pub fn get_effectiveness(attacking: &Element, defending: &Element) -> f32 {
    match attacking {
        Element::Normal => match defending {
            Element::Rock | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Ghost => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Fire => match defending {
            Element::Fire | Element::Water | Element::Rock | Element::Dragon => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Grass | Element::Ice | Element::Bug | Element::Steel => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Water => match defending {
            Element::Fire | Element::Water | Element::Ice | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Ground | Element::Rock | Element::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Electric => match defending {
            Element::Electric | Element::Grass | Element::Dragon => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Ground => NO_EFFECT_MULTIPLIER,
            Element::Water | Element::Flying => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Grass => match defending {
            Element::Fire | Element::Grass | Element::Poison | Element::Flying | Element::Bug
            | Element::Dragon | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Water | Element::Ground | Element::Rock => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Ice => match defending {
            Element::Fire | Element::Water | Element::Ice | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Grass | Element::Ground | Element::Flying | Element::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Fighting => match defending {
            Element::Normal | Element::Ice | Element::Rock | Element::Dark | Element::Steel => SUPER_EFFECTIVE_MULTIPLIER,
            Element::Poison | Element::Flying | Element::Psychic | Element::Bug | Element::Fairy => {
                NOT_VERY_EFFECTIVE_MULTIPLIER
            }
            Element::Ghost => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Poison => match defending {
            Element::Grass | Element::Fighting | Element::Poison | Element::Bug
            | Element::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Poison | Element::Ground | Element::Rock | Element::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Ground => match defending {
            Element::Grass | Element::Bug => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Fire | Element::Electric | Element::Poison | Element::Rock | Element::Steel => {
                SUPER_EFFECTIVE_MULTIPLIER
            }
            Element::Flying => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Flying => match defending {
            Element::Electric | Element::Rock | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Grass | Element::Fighting | Element::Bug => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Psychic => match defending {
            Element::Psychic | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Fighting | Element::Poison => SUPER_EFFECTIVE_MULTIPLIER,
            Element::Dark => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Bug => match defending {
            Element::Fire | Element::Fighting | Element::Poison | Element::Flying | Element::Ghost
            | Element::Steel | Element::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Grass | Element::Psychic | Element::Dark => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Rock => match defending {
            Element::Fighting | Element::Ground | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Fire | Element::Ice | Element::Flying | Element::Bug => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Ghost => match defending {
            Element::Dark => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Psychic | Element::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            Element::Normal => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Dragon => match defending {
            Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Dark => match defending {
            Element::Fighting | Element::Dark | Element::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Psychic | Element::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Steel => match defending {
            Element::Fire | Element::Water | Element::Electric | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Ice | Element::Rock | Element::Fairy => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        Element::Fairy => match defending {
            Element::Fire | Element::Poison | Element::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            Element::Fighting | Element::Dragon | Element::Dark => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
    }
}
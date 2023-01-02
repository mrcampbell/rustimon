#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementalType {
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


pub fn get_effectiveness(attacking: &ElementalType, defending: &ElementalType) -> f32 {
    match attacking {
        ElementalType::Normal => match defending {
            ElementalType::Rock | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Ghost => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Fire => match defending {
            ElementalType::Fire | ElementalType::Water | ElementalType::Rock | ElementalType::Dragon => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Grass | ElementalType::Ice | ElementalType::Bug | ElementalType::Steel => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Water => match defending {
            ElementalType::Fire | ElementalType::Water | ElementalType::Ice | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Ground | ElementalType::Rock | ElementalType::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Electric => match defending {
            ElementalType::Electric | ElementalType::Grass | ElementalType::Dragon => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Ground => NO_EFFECT_MULTIPLIER,
            ElementalType::Water | ElementalType::Flying => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Grass => match defending {
            ElementalType::Fire | ElementalType::Grass | ElementalType::Poison | ElementalType::Flying | ElementalType::Bug
            | ElementalType::Dragon | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Water | ElementalType::Ground | ElementalType::Rock => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Ice => match defending {
            ElementalType::Fire | ElementalType::Water | ElementalType::Ice | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Grass | ElementalType::Ground | ElementalType::Flying | ElementalType::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Fighting => match defending {
            ElementalType::Normal | ElementalType::Ice | ElementalType::Rock | ElementalType::Dark | ElementalType::Steel => SUPER_EFFECTIVE_MULTIPLIER,
            ElementalType::Poison | ElementalType::Flying | ElementalType::Psychic | ElementalType::Bug | ElementalType::Fairy => {
                NOT_VERY_EFFECTIVE_MULTIPLIER
            }
            ElementalType::Ghost => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Poison => match defending {
            ElementalType::Grass | ElementalType::Fighting | ElementalType::Poison | ElementalType::Bug
            | ElementalType::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
           /* ElementalType::Poison */ | ElementalType::Ground | ElementalType::Rock | ElementalType::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Ground => match defending {
            ElementalType::Grass | ElementalType::Bug => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Fire | ElementalType::Electric | ElementalType::Poison | ElementalType::Rock | ElementalType::Steel => {
                SUPER_EFFECTIVE_MULTIPLIER
            }
            ElementalType::Flying => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Flying => match defending {
            ElementalType::Electric | ElementalType::Rock | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Grass | ElementalType::Fighting | ElementalType::Bug => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Psychic => match defending {
            ElementalType::Psychic | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Fighting | ElementalType::Poison => SUPER_EFFECTIVE_MULTIPLIER,
            ElementalType::Dark => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Bug => match defending {
            ElementalType::Fire | ElementalType::Fighting | ElementalType::Poison | ElementalType::Flying | ElementalType::Ghost
            | ElementalType::Steel | ElementalType::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Grass | ElementalType::Psychic | ElementalType::Dark => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Rock => match defending {
            ElementalType::Fighting | ElementalType::Ground | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Fire | ElementalType::Ice | ElementalType::Flying | ElementalType::Bug => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Ghost => match defending {
            ElementalType::Dark => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Psychic | ElementalType::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            ElementalType::Normal => NO_EFFECT_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Dragon => match defending {
            ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Dragon => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Dark => match defending {
            ElementalType::Fighting | ElementalType::Dark | ElementalType::Fairy => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Psychic | ElementalType::Ghost => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Steel => match defending {
            ElementalType::Fire | ElementalType::Water | ElementalType::Electric | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Ice | ElementalType::Rock | ElementalType::Fairy => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
        ElementalType::Fairy => match defending {
            ElementalType::Fire | ElementalType::Poison | ElementalType::Steel => NOT_VERY_EFFECTIVE_MULTIPLIER,
            ElementalType::Fighting | ElementalType::Dragon | ElementalType::Dark => SUPER_EFFECTIVE_MULTIPLIER,
            _ => NORMALLY_EFFECTIVE_MULTIPLIER,
        },
    }
}
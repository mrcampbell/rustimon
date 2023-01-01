#[cfg(test)]

use crate::core::types::{
    pokemon::{Pokemon, Species},
    stat_group::StatGroup,
};

#[test]
fn test_end_to_end() {
    let pokemon = Pokemon::new(
        Some(0),
        "test".to_string(),
        78,
        Species {
            id: 0,
            name: "test".to_string(),
            stats: StatGroup {
                hp: 108,
                atk: 130,
                def: 95,
                spec_atk: 80,
                spec_def: 85,
                speed: 102,
            },
        },
        Some(StatGroup {
            hp: 24,
            atk: 12,
            def: 30,
            spec_atk: 16,
            spec_def: 23,
            speed: 5,
        }),
        Some(StatGroup {
            hp: 74,
            atk: 190,
            def: 91,
            spec_atk: 48,
            spec_def: 84,
            speed: 23,
        }),
    );

    assert_eq!(pokemon.stats.hp, 289);
    assert_eq!(pokemon.stats.atk, 253);
    assert_eq!(pokemon.stats.def, 193);
    assert_eq!(pokemon.stats.spec_atk, 151);
    assert_eq!(pokemon.stats.spec_def, 171);
    assert_eq!(pokemon.stats.speed, 171);

    assert_eq!(pokemon.level, 78);
    assert_eq!(pokemon.species.name, "test");
    assert_eq!(pokemon.species.stats.hp, 108);
    assert_eq!(pokemon.species.stats.atk, 130);
    assert_eq!(pokemon.species.stats.def, 95);
    assert_eq!(pokemon.species.stats.spec_atk, 80);
    assert_eq!(pokemon.species.stats.spec_def, 85);
    assert_eq!(pokemon.species.stats.speed, 102);

    assert_eq!(pokemon.iv.hp, 24);
    assert_eq!(pokemon.iv.atk, 12);
    assert_eq!(pokemon.iv.def, 30);
    assert_eq!(pokemon.iv.spec_atk, 16);
    assert_eq!(pokemon.iv.spec_def, 23);
    assert_eq!(pokemon.iv.speed, 5);

    assert_eq!(pokemon.ev.hp, 74);
    assert_eq!(pokemon.ev.atk, 190);
    assert_eq!(pokemon.ev.def, 91);
    assert_eq!(pokemon.ev.spec_atk, 48);
    assert_eq!(pokemon.ev.spec_def, 84);
    assert_eq!(pokemon.ev.speed, 23);

}

// scenario from here: https://bulbapedia.bulbagarden.net/wiki/Stat
#[test]
fn test_calculate_atk() {
    assert_eq!(Pokemon::calculate_stat(78.0, 102.0, 5.0, 23.0), 171.0);
}

#[test]
fn test_calculate_hp() {
    assert_eq!(Pokemon::calculate_hp(78.0, 108.0, 24.0, 74.0), 289.0);
}

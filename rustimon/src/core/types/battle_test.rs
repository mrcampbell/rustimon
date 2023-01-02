#[cfg(test)]

use super::elemental_types::get_effectiveness;
use super::elemental_types::ElementalType;

#[test]
pub fn test_effectiveness() {
    assert_eq!(get_effectiveness(&ElementalType::Fire, &ElementalType::Grass), 2.0);
    assert_eq!(get_effectiveness(&ElementalType::Bug, &ElementalType::Grass), 2.0);
    assert_eq!(get_effectiveness(&ElementalType::Normal, &ElementalType::Ghost), 0.0);
    assert_eq!(get_effectiveness(&ElementalType::Flying, &ElementalType::Fighting), 2.0);
    assert_eq!(get_effectiveness(&ElementalType::Flying, &ElementalType::Rock), 0.5);
}
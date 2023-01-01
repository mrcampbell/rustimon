// scenario from here: https://bulbapedia.bulbagarden.net/wiki/Stat
#[test]
fn test_calculate_atk() {
  assert_eq!(Pokemon::calculate_stat(78.0, 102.0, 5.0, 23.0), 171.0);
}

#[test]
fn test_calculate_hp() {
  assert_eq!(Pokemon::calculate_hp(78.0, 108.0, 24.0, 74.0), 289.0);
}
use crate::core::types::pokemon::{Pokemon, Species, StatGroup};

pub mod core;

pub fn init() {
  print!("Hello, world!");

 let p =  Pokemon::new("1".to_string(), 1, Species{ id: todo!(), name: todo!(), stats: todo!() }, None, StatGroup{ hp: todo!(), atk: todo!(), def: todo!(), spec_atk: todo!(), spec_def: todo!(), spd: todo!() }, StatGroup{ hp: todo!(), atk: todo!(), def: todo!(), spec_atk: todo!(), spec_def: todo!(), spd: todo!() });

  println!("Pokemon: {:?}", p);
}
use super::{elemental_types::get_effectiveness, pokemon::Pokemon, pokemon_move::PokemonMove};

pub struct Battle {

}

pub struct AttackResult {
  pub elemental_effectiveness_multiplier: f32,
}


impl Battle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_attack_results(_attacker: Pokemon, defender: Pokemon, attack: PokemonMove) -> AttackResult {

      let elemental_effectiveness_multiplier = {
        let elemental_effectiveness_multiplier = get_effectiveness(&attack.elemental_type, &defender.species.elemental_types.0);
        if let Some(elemental_type) = &defender.species.elemental_types.1 {
          elemental_effectiveness_multiplier * get_effectiveness(&attack.elemental_type, elemental_type)
        } else {
          elemental_effectiveness_multiplier
        }
      };

        let attack_result = AttackResult {
            elemental_effectiveness_multiplier,
        };
        return attack_result;
    }
}
use super::{stat_group::StatGroup, elemental_types::ElementalType};

#[derive(Clone, Debug)]
pub struct Species {
    pub id: u32,
    pub name: String,
    pub stats: StatGroup,
    pub elemental_types: (ElementalType, Option<ElementalType>),
}

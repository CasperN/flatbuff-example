// Manual implementation of the Rust object API.
// For learning and eventual testing purposes.

// flatbuffer-enums and flatbuffer-structs can share types with the builder
// api but tables and unions need to be generated to remove WOffset stuff.
use crate::monster_generated::my_game::sample::Color;
use crate::monster_generated::my_game::sample::Monster;
use crate::monster_generated::my_game::sample::Vec3;
use flatbuffers;

pub struct WeaponT {
    name: String,
    damage: i16,
}

pub enum EquipmentT {
    None,
    WeaponT(WeaponT),
}

pub struct MonsterT {
    pub pos: Option<Vec3>,
    pub mana: i16,
    pub hp: i16,
    pub name: String,
    pub inventory: Vec<u8>,
    pub color: Color,
    pub weapons: Vec<WeaponT>,
    pub equipped: EquipmentT,
    pub path: Vec<Vec3>,
}

impl Default for MonsterT {
    fn default() -> Self {
        MonsterT {
            pos: None,
            mana: 150,
            hp: 100,
            name: String::new(),
            inventory: Vec::new(),
            color: Color::Blue,
            weapons: Vec::new(),
            equipped: EquipmentT::None,
            path: Vec::new(),
        }
    }
}

impl<'a> Monster<'a> {
    pub fn unpack(&self) -> MonsterT {
        unimplemented!()
    }
    pub fn pack(&mut self, m: &MonsterT) {
        unimplemented!()
    }
}

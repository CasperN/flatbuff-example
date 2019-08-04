// Manual implementation of the Rust object API.
// For learning and eventual testing purposes.

// flatbuffer-enums can share types with the builder, maybe. Tables and unions need to be
// generated to remove WOffset stuff. Even structs because of endianness
use crate::monster_generated::my_game::sample::Color;
use crate::monster_generated::my_game::sample::Equipment;
use crate::monster_generated::my_game::sample::Monster;
use crate::monster_generated::my_game::sample::Vec3;
use crate::monster_generated::my_game::sample::Weapon;

// TODO Other derives.
#[derive(Debug)]
pub struct Vec3T {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct WeaponT {
    name: Option<String>,
    damage: i16,
}

#[derive(Debug)]
pub enum EquipmentT {
    NONE,
    WeaponT(WeaponT),
}

#[derive(Debug)]
pub struct MonsterT {
    pub pos: Option<Vec3T>,
    pub mana: i16,
    pub hp: i16,
    pub name: Option<String>,
    pub inventory: Vec<u8>,
    pub color: Color,
    pub weapons: Vec<WeaponT>,
    pub equipped: EquipmentT,
    pub path: Vec<Vec3T>,
}

impl Default for MonsterT {
    fn default() -> Self {
        MonsterT {
            pos: None,
            mana: 150,
            hp: 100,
            name: None,
            inventory: Vec::new(),
            color: Color::Blue,
            weapons: Vec::new(),
            equipped: EquipmentT::NONE,
            path: Vec::new(),
        }
    }
}

impl Vec3 {
    pub fn unpack(&self) -> Vec3T {
        Vec3T {
            x: self.x(),
            y: self.y(),
            z: self.z(),
        }
    }
}

impl<'a> Weapon<'a> {
    pub fn unpack(&self) -> WeaponT {
        WeaponT {
            name: self.name().map(|n| n.to_string()),
            damage: self.damage()
        }
    }
}

impl<'a> Monster<'a> {
    pub fn unpack(&self) -> MonsterT {
        MonsterT {
            pos: self.pos().map(|x| x.unpack()), // Stuct => unpack.
            mana: self.mana(),
            hp: self.hp(),
            name: self.name().map(|x| x.to_string()),
            inventory: self.inventory().unwrap_or(&[]).to_vec(), // Vec of scalars is simple to_vec
            color: self.color(),
            weapons: match self.weapons() { // Vector of tables
                None => Vec::new(),
                Some(w) => (0..w.len()).map(|i| w.get(i).unpack()).collect(),
            },
            equipped: match self.equipped_type() { // Union type.
                Equipment::NONE => EquipmentT::NONE,
                // This breaks references
                Equipment::Weapon => EquipmentT::WeaponT(self.equipped_as_weapon().unwrap().unpack()),
            },
            // Vector of structs needs unpack.
            path: self.path().unwrap_or(&[]).iter().map(|x| x.unpack()).collect(),
        }
    }
    pub fn pack(&mut self, m: &MonsterT) {
        unimplemented!()
    }
}

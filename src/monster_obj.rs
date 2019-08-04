// Manual implementation of the Rust object API.
// For learning and eventual testing purposes.

// flatbuffer-enums can share types with the builder, maybe. Tables and unions need to be
// generated to remove WOffset stuff. Even structs because of endianness
use crate::monster_generated::my_game::sample::Color;
use crate::monster_generated::my_game::sample::Equipment;
use crate::monster_generated::my_game::sample::Monster;
use crate::monster_generated::my_game::sample::MonsterArgs;
use crate::monster_generated::my_game::sample::Vec3;
use crate::monster_generated::my_game::sample::Weapon;
use crate::monster_generated::my_game::sample::WeaponBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct WeaponT {
    name: String,
    damage: i16,
}
impl WeaponT {
    pub fn pack<'bldr: 'mut_bldr, 'mut_bldr>(
        &self,
        fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    ) -> flatbuffers::WIPOffset<Weapon<'bldr>> {
        // Need to create ptrs before builder b/c lifetimes.
        let name = if self.name.is_empty() {
            None
        } else {
            Some(fbb.create_string(&self.name))
        };
        let mut builder = WeaponBuilder::new(fbb);
        if let Some(name) = name {
            builder.add_name(name);
        }
        builder.add_damage(self.damage);
        builder.finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EquipmentT {
    NONE,
    Weapon(WeaponT),
}
impl Default for EquipmentT {
    fn default() -> Self {
        EquipmentT::NONE
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            equipped: EquipmentT::NONE,
            path: Vec::new(),
        }
    }
}
impl MonsterT {
    pub fn pack<'bldr: 'mut_bldr, 'mut_bldr>(
        &self,
        fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    ) -> flatbuffers::WIPOffset<Monster<'bldr>> {
        let name = if self.name.is_empty() {
            None
        } else {
            Some(fbb.create_string(&self.name))
        };
        let inventory = if self.inventory.is_empty() {
            None
        } else {
            Some(fbb.create_vector(&self.inventory))
        };
        let weapons = if self.weapons.is_empty() {
            None
        } else {
            let ws: Vec<_> = self.weapons.iter().map(|w| w.pack(fbb)).collect();
            Some(fbb.create_vector(&ws))
        };
        let (equipped_type, equipped) = match &self.equipped {
            EquipmentT::NONE => (Equipment::NONE, None),
            EquipmentT::Weapon(w) => (
                Equipment::Weapon,
                Some(w.pack(fbb).as_union_value()),
            ),
        };
        let path = if self.path.is_empty() {
            None
        } else {
            Some(fbb.create_vector(&self.path))
        };
        Monster::create(
            fbb,
            &MonsterArgs {
                pos: self.pos.as_ref(),
                hp: self.hp,
                mana: self.mana,
                name,
                inventory,
                color: self.color,
                weapons,
                equipped_type,
                equipped,
                path,
            },
        )
    }
}


impl<'a> Weapon<'a> {
    pub fn unpack(&self) -> WeaponT {
        WeaponT {
            name: self.name().unwrap_or("").to_string(),
            damage: self.damage(),
        }
    }
}

impl<'a> Monster<'a> {
    // TODO: Resolver nonsense?
    pub fn unpack(&self) -> MonsterT {
        MonsterT {
            pos: self.pos().cloned(), // Stuct => copy
            mana: self.mana(),                  // scalars can be copied.
            hp: self.hp(),
            name: self.name().unwrap_or("").to_string(), // Unwrap options for string / vector
            inventory: self.inventory().unwrap_or(&[]).to_vec(),
            color: self.color(),
            weapons: match self.weapons() {
                // Vector of tables -> map unpack
                None => Vec::new(),
                Some(w) => (0..w.len()).map(|i| w.get(i).unpack()).collect(),
            },
            equipped: match self.equipped_type() {
                // Union type.
                Equipment::NONE => EquipmentT::NONE,
                Equipment::Weapon => {
                    // This breaks references but that's what happens in C++
                    EquipmentT::Weapon(self.equipped_as_weapon().unwrap().unpack())
                }
            },
            // Vector of structs are just cloned.
            path: self.path().unwrap_or(&[]).to_vec(),
        }
    }
}

#![feature(dbg_macro)]

extern crate flatbuffers;

mod monster_generated;
mod monster_obj;

use crate::monster_generated::my_game::sample::{
    get_root_as_monster, Color, Equipment, Monster, MonsterArgs, Vec3, Weapon, WeaponArgs,
};

fn make_monster() -> (Vec<u8>, usize) {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let weapon_one_name = builder.create_string("Sword");
    let weapon_two_name = builder.create_string("Axe");
    // Use the `Weapon::create` shortcut to create Weapons with named field
    // arguments.
    let sword = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_one_name),
            damage: 3,
        },
    );
    let axe = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_two_name),
            damage: 5,
        },
    );
    let name = builder.create_string("Orc");
    let inventory = builder.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let weapons = builder.create_vector(&[sword, axe]);
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(4.0, 5.0, 6.0);
    let path = builder.create_vector(&[x, y]);

    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            name: Some(name),
            inventory: Some(inventory),
            color: Color::Red,
            weapons: Some(weapons),
            equipped_type: Equipment::Weapon,
            equipped: Some(axe.as_union_value()),
            path: Some(path),
        },
    );
    builder.finish(orc, None);
    builder.collapse()
}

fn main() {
    println!("Hello, world!");
    let (buf, root) = make_monster();
    let monster = get_root_as_monster(&buf[root..]);

    // Test unpack then repack yields the same MonsterT.
    let mut x = monster.unpack();
    x.hp += 4;

    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let offset = x.pack(&mut builder);
    builder.finish(offset, None);
    let finished_data = builder.finished_data();
    let x2 = get_root_as_monster(&finished_data).unpack();
    assert_eq!(x, x2);
}

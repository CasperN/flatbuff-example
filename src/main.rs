#![feature(dbg_macro)]

extern crate flatbuffers;

mod monster_generated;
mod monster_obj;

use crate::monster_generated::my_game::sample::{get_root_as_monster,
                                             Color, Equipment,
                                             Monster, MonsterArgs,
                                             Vec3,
                                             Weapon, WeaponArgs};

fn make_monster() -> (Vec<u8>, usize) {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let weapon_one_name = builder.create_string("Sword");
    let weapon_two_name = builder.create_string("Axe");
    // Use the `Weapon::create` shortcut to create Weapons with named field
    // arguments.
    let sword = Weapon::create(&mut builder, &WeaponArgs{
        name: Some(weapon_one_name),
        damage: 3,
    });
    let axe = Weapon::create(&mut builder, &WeaponArgs{
        name: Some(weapon_two_name),
        damage: 5,
    });
    // Name of the Monster.
    let name = builder.create_string("Orc");
    // Inventory.
    let inventory = builder.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // Create a FlatBuffer `vector` that contains offsets to the sword and axe
    // we created above.
    let weapons = builder.create_vector(&[sword, axe]);
    // Create the monster using the `Monster::create` helper function. This
    // function accepts a `MonsterArgs` struct, which supplies all of the data
    // needed to build a `Monster`. To supply empty/default fields, just use the
    // Rust built-in `Default::default()` function, as demonstrated below.

    // Create the path vector of Vec3 objects.
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(4.0, 5.0, 6.0);
    let path = builder.create_vector(&[x, y]);
    // Note that, for convenience, it is also valid to create a vector of
    // references to structs, like this:
    // let path = builder.create_vector(&[&x, &y]);

    let orc = Monster::create(&mut builder, &MonsterArgs{
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
    });
    builder.finish(orc, None);
    builder.collapse()
}



fn main() {
    println!("Hello, world!");
    let (buf, root) = make_monster();
    let monster = get_root_as_monster(&buf[root..]);
    dbg!(monster.hp());
    dbg!(monster.mana());
    dbg!(monster.pos());

    dbg!(monster.unpack());

}

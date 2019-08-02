extern crate flatbuffers;

mod monster_generated;
mod monster_obj;

use flatbuffers::FlatBufferBuilder;

fn main() {
    println!("Hello, world!");
    let _bldr = FlatBufferBuilder::new();
    let _bytes: Vec<u8> = Vec::new();
}

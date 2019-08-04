# What is this?
This repo is me experimenting with creating the flatbuffers object API in rust.

## Notes
* CPP object API breaks references within the buffer by copying. Note in
`obj_breaks_ref.cc` the `equipped` is not the second weapon in `weapons`.
* I need to figure out what's happening with the resolver thing and whether
that needs to be in the rust object API -- probably.
* C++ reuses `Vec3` in their object api but in rust, it is not mutable and the
fields are private (accessors do endian conversions). Maybe we should add
mutating accessors in rust first. It is kind of ugly though, I'd kind of prefer
to have a totally native struct.

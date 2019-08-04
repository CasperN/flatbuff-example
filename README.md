# What is this?
This repo is me experimenting with creating the flatbuffers object API in rust.

## Devlog

### 2019 August 2
* I moved `pack` from the flatbuffer object namespace (e.g. `Weapon::pack`) to
the native one `WeaponT::pack`. It seems more ergonomic to me but maybe
we should go for consistency.
* CPP object API breaks references within the buffer by copying. Note in
`obj_breaks_ref.cc` the `equipped` is not the second weapon in `weapons`. This
makes life easier in rust but it does mean unpacking and repacking results in a
different buffer.
* C++ reuses `Vec3` in their object api but in rust, it is not mutable and the
fields are private (accessors do endian conversions). Maybe we should add
mutating accessors in rust first. It is kind of ugly though, I'd kind of prefer
to have a totally native struct.
* I need to figure out what's happening with the resolver thing and whether
that needs to be in the rust object API -- probably.

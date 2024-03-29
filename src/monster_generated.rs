// automatically generated by the FlatBuffers compiler, do not modify

extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod my_game {

    use std::cmp::Ordering;
    use std::mem;

    extern crate flatbuffers;
    use self::flatbuffers::EndianScalar;
    #[allow(unused_imports, dead_code)]
    pub mod sample {

        use std::cmp::Ordering;
        use std::mem;

        extern crate flatbuffers;
        use self::flatbuffers::EndianScalar;

        #[allow(non_camel_case_types)]
        #[repr(i8)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum Color {
            Red = 0,
            Green = 1,
            Blue = 2,
        }

        const ENUM_MIN_COLOR: i8 = 0;
        const ENUM_MAX_COLOR: i8 = 2;

        impl<'a> flatbuffers::Follow<'a> for Color {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::read_scalar_at::<Self>(buf, loc)
            }
        }

        impl flatbuffers::EndianScalar for Color {
            #[inline]
            fn to_little_endian(self) -> Self {
                let n = i8::to_le(self as i8);
                let p = &n as *const i8 as *const Color;
                unsafe { *p }
            }
            #[inline]
            fn from_little_endian(self) -> Self {
                let n = i8::from_le(self as i8);
                let p = &n as *const i8 as *const Color;
                unsafe { *p }
            }
        }

        impl flatbuffers::Push for Color {
            type Output = Color;
            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                flatbuffers::emplace_scalar::<Color>(dst, *self);
            }
        }

        #[allow(non_camel_case_types)]
        const ENUM_VALUES_COLOR: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

        #[allow(non_camel_case_types)]
        const ENUM_NAMES_COLOR: [&'static str; 3] = ["Red", "Green", "Blue"];

        pub fn enum_name_color(e: Color) -> &'static str {
            let index = e as i8;
            ENUM_NAMES_COLOR[index as usize]
        }

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum Equipment {
            NONE = 0,
            Weapon = 1,
        }

        const ENUM_MIN_EQUIPMENT: u8 = 0;
        const ENUM_MAX_EQUIPMENT: u8 = 1;

        impl<'a> flatbuffers::Follow<'a> for Equipment {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::read_scalar_at::<Self>(buf, loc)
            }
        }

        impl flatbuffers::EndianScalar for Equipment {
            #[inline]
            fn to_little_endian(self) -> Self {
                let n = u8::to_le(self as u8);
                let p = &n as *const u8 as *const Equipment;
                unsafe { *p }
            }
            #[inline]
            fn from_little_endian(self) -> Self {
                let n = u8::from_le(self as u8);
                let p = &n as *const u8 as *const Equipment;
                unsafe { *p }
            }
        }

        impl flatbuffers::Push for Equipment {
            type Output = Equipment;
            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                flatbuffers::emplace_scalar::<Equipment>(dst, *self);
            }
        }

        #[allow(non_camel_case_types)]
        const ENUM_VALUES_EQUIPMENT: [Equipment; 2] = [Equipment::NONE, Equipment::Weapon];

        #[allow(non_camel_case_types)]
        const ENUM_NAMES_EQUIPMENT: [&'static str; 2] = ["NONE", "Weapon"];

        pub fn enum_name_equipment(e: Equipment) -> &'static str {
            let index = e as u8;
            ENUM_NAMES_EQUIPMENT[index as usize]
        }

        pub struct EquipmentUnionTableOffset {}
        // struct Vec3, aligned to 4
        #[repr(C, align(4))]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct Vec3 {
            x_: f32,
            y_: f32,
            z_: f32,
        } // pub struct Vec3
        impl flatbuffers::SafeSliceAccess for Vec3 {}
        impl<'a> flatbuffers::Follow<'a> for Vec3 {
            type Inner = &'a Vec3;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                <&'a Vec3>::follow(buf, loc)
            }
        }
        impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
            type Inner = &'a Vec3;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::follow_cast_ref::<Vec3>(buf, loc)
            }
        }
        impl<'b> flatbuffers::Push for Vec3 {
            type Output = Vec3;
            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                let src = unsafe {
                    ::std::slice::from_raw_parts(self as *const Vec3 as *const u8, Self::size())
                };
                dst.copy_from_slice(src);
            }
        }
        impl<'b> flatbuffers::Push for &'b Vec3 {
            type Output = Vec3;

            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                let src = unsafe {
                    ::std::slice::from_raw_parts(*self as *const Vec3 as *const u8, Self::size())
                };
                dst.copy_from_slice(src);
            }
        }

        impl Vec3 {
            pub fn new<'a>(_x: f32, _y: f32, _z: f32) -> Self {
                Vec3 {
                    x_: _x.to_little_endian(),
                    y_: _y.to_little_endian(),
                    z_: _z.to_little_endian(),
                }
            }
            pub fn x<'a>(&'a self) -> f32 {
                self.x_.from_little_endian()
            }
            pub fn y<'a>(&'a self) -> f32 {
                self.y_.from_little_endian()
            }
            pub fn z<'a>(&'a self) -> f32 {
                self.z_.from_little_endian()
            }
        }

        pub enum MonsterOffset {}
        #[derive(Copy, Clone, Debug, PartialEq)]

        pub struct Monster<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
            type Inner = Monster<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> Monster<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Monster { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args MonsterArgs<'args>,
            ) -> flatbuffers::WIPOffset<Monster<'bldr>> {
                let mut builder = MonsterBuilder::new(_fbb);
                if let Some(x) = args.path {
                    builder.add_path(x);
                }
                if let Some(x) = args.equipped {
                    builder.add_equipped(x);
                }
                if let Some(x) = args.weapons {
                    builder.add_weapons(x);
                }
                if let Some(x) = args.inventory {
                    builder.add_inventory(x);
                }
                if let Some(x) = args.name {
                    builder.add_name(x);
                }
                if let Some(x) = args.pos {
                    builder.add_pos(x);
                }
                builder.add_hp(args.hp);
                builder.add_mana(args.mana);
                builder.add_equipped_type(args.equipped_type);
                builder.add_color(args.color);
                builder.finish()
            }

            pub const VT_POS: flatbuffers::VOffsetT = 4;
            pub const VT_MANA: flatbuffers::VOffsetT = 6;
            pub const VT_HP: flatbuffers::VOffsetT = 8;
            pub const VT_NAME: flatbuffers::VOffsetT = 10;
            pub const VT_INVENTORY: flatbuffers::VOffsetT = 14;
            pub const VT_COLOR: flatbuffers::VOffsetT = 16;
            pub const VT_WEAPONS: flatbuffers::VOffsetT = 18;
            pub const VT_EQUIPPED_TYPE: flatbuffers::VOffsetT = 20;
            pub const VT_EQUIPPED: flatbuffers::VOffsetT = 22;
            pub const VT_PATH: flatbuffers::VOffsetT = 24;

            #[inline]
            pub fn pos(&self) -> Option<&'a Vec3> {
                self._tab.get::<Vec3>(Monster::VT_POS, None)
            }
            #[inline]
            pub fn mana(&self) -> i16 {
                self._tab.get::<i16>(Monster::VT_MANA, Some(150)).unwrap()
            }
            #[inline]
            pub fn hp(&self) -> i16 {
                self._tab.get::<i16>(Monster::VT_HP, Some(100)).unwrap()
            }
            #[inline]
            pub fn name(&self) -> Option<&'a str> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Monster::VT_NAME, None)
            }
            #[inline]
            pub fn inventory(&self) -> Option<&'a [u8]> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                        Monster::VT_INVENTORY,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
            #[inline]
            pub fn color(&self) -> Color {
                self._tab
                    .get::<Color>(Monster::VT_COLOR, Some(Color::Blue))
                    .unwrap()
            }
            #[inline]
            pub fn weapons(
                &self,
            ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Weapon<'a>>>>
            {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<flatbuffers::ForwardsUOffset<Weapon<'a>>>,
                >>(Monster::VT_WEAPONS, None)
            }
            #[inline]
            pub fn equipped_type(&self) -> Equipment {
                self._tab
                    .get::<Equipment>(Monster::VT_EQUIPPED_TYPE, Some(Equipment::NONE))
                    .unwrap()
            }
            #[inline]
            pub fn equipped(&self) -> Option<flatbuffers::Table<'a>> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(
                        Monster::VT_EQUIPPED,
                        None,
                    )
            }
            #[inline]
            pub fn path(&self) -> Option<&'a [Vec3]> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<Vec3>>>(
                        Monster::VT_PATH,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
            #[inline]
            #[allow(non_snake_case)]
            pub fn equipped_as_weapon(&self) -> Option<Weapon<'a>> {
                if self.equipped_type() == Equipment::Weapon {
                    self.equipped().map(|u| Weapon::init_from_table(u))
                } else {
                    None
                }
            }
        }

        pub struct MonsterArgs<'a> {
            pub pos: Option<&'a Vec3>,
            pub mana: i16,
            pub hp: i16,
            pub name: Option<flatbuffers::WIPOffset<&'a str>>,
            pub inventory: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub color: Color,
            pub weapons: Option<
                flatbuffers::WIPOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Weapon<'a>>>,
                >,
            >,
            pub equipped_type: Equipment,
            pub equipped: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
            pub path: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Vec3>>>,
        }
        impl<'a> Default for MonsterArgs<'a> {
            #[inline]
            fn default() -> Self {
                MonsterArgs {
                    pos: None,
                    mana: 150,
                    hp: 100,
                    name: None,
                    inventory: None,
                    color: Color::Blue,
                    weapons: None,
                    equipped_type: Equipment::NONE,
                    equipped: None,
                    path: None,
                }
            }
        }
        pub struct MonsterBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
            #[inline]
            pub fn add_pos(&mut self, pos: &'b Vec3) {
                self.fbb_.push_slot_always::<&Vec3>(Monster::VT_POS, pos);
            }
            #[inline]
            pub fn add_mana(&mut self, mana: i16) {
                self.fbb_.push_slot::<i16>(Monster::VT_MANA, mana, 150);
            }
            #[inline]
            pub fn add_hp(&mut self, hp: i16) {
                self.fbb_.push_slot::<i16>(Monster::VT_HP, hp, 100);
            }
            #[inline]
            pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_NAME, name);
            }
            #[inline]
            pub fn add_inventory(
                &mut self,
                inventory: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
            ) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    Monster::VT_INVENTORY,
                    inventory,
                );
            }
            #[inline]
            pub fn add_color(&mut self, color: Color) {
                self.fbb_
                    .push_slot::<Color>(Monster::VT_COLOR, color, Color::Blue);
            }
            #[inline]
            pub fn add_weapons(
                &mut self,
                weapons: flatbuffers::WIPOffset<
                    flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Weapon<'b>>>,
                >,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_WEAPONS, weapons);
            }
            #[inline]
            pub fn add_equipped_type(&mut self, equipped_type: Equipment) {
                self.fbb_.push_slot::<Equipment>(
                    Monster::VT_EQUIPPED_TYPE,
                    equipped_type,
                    Equipment::NONE,
                );
            }
            #[inline]
            pub fn add_equipped(
                &mut self,
                equipped: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_EQUIPPED, equipped);
            }
            #[inline]
            pub fn add_path(
                &mut self,
                path: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Vec3>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_PATH, path);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
                let start = _fbb.start_table();
                MonsterBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Monster<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        pub enum WeaponOffset {}
        #[derive(Copy, Clone, Debug, PartialEq)]

        pub struct Weapon<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for Weapon<'a> {
            type Inner = Weapon<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> Weapon<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                Weapon { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args WeaponArgs<'args>,
            ) -> flatbuffers::WIPOffset<Weapon<'bldr>> {
                let mut builder = WeaponBuilder::new(_fbb);
                if let Some(x) = args.name {
                    builder.add_name(x);
                }
                builder.add_damage(args.damage);
                builder.finish()
            }

            pub const VT_NAME: flatbuffers::VOffsetT = 4;
            pub const VT_DAMAGE: flatbuffers::VOffsetT = 6;

            #[inline]
            pub fn name(&self) -> Option<&'a str> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Weapon::VT_NAME, None)
            }
            #[inline]
            pub fn damage(&self) -> i16 {
                self._tab.get::<i16>(Weapon::VT_DAMAGE, Some(0)).unwrap()
            }
        }

        pub struct WeaponArgs<'a> {
            pub name: Option<flatbuffers::WIPOffset<&'a str>>,
            pub damage: i16,
        }
        impl<'a> Default for WeaponArgs<'a> {
            #[inline]
            fn default() -> Self {
                WeaponArgs {
                    name: None,
                    damage: 0,
                }
            }
        }
        pub struct WeaponBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> WeaponBuilder<'a, 'b> {
            #[inline]
            pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(Weapon::VT_NAME, name);
            }
            #[inline]
            pub fn add_damage(&mut self, damage: i16) {
                self.fbb_.push_slot::<i16>(Weapon::VT_DAMAGE, damage, 0);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WeaponBuilder<'a, 'b> {
                let start = _fbb.start_table();
                WeaponBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<Weapon<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
            flatbuffers::get_root::<Monster<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
            flatbuffers::get_size_prefixed_root::<Monster<'a>>(buf)
        }

        #[inline]
        pub fn finish_monster_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<Monster<'a>>,
        ) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_monster_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<Monster<'a>>,
        ) {
            fbb.finish_size_prefixed(root, None);
        }
    } // pub mod Sample
} // pub mod MyGame

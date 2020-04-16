const tile_sprite:&str="/assets/tile_sprite.png";
const path_sprite:usize=0;
const generic_sprite:usize=0;
const debug_fmt:&str="// Debug : src/world/mod.rs:150 count={},buf={:?}";


pub mod directions{
    pub const TOP:u8=1;
    pub const TOP_RIGHT:u8=2;
    pub const RIGHT:u8=3;
    pub const BOTTOM_RIGHT:u8=4;
    pub const BOTTOM:u8=5;
    pub const BOTTOM_LEFT:u8=6;
    pub const LEFT:u8=7;
    pub const TOP_LEFT:u8=8;
}

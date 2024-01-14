#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod villager;

#[skyline::main(name = "smashline_main")]
pub fn main() {
    villager::install();
}
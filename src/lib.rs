#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod kirby;
mod marth;
mod villager;

#[skyline::main(name = "ultimate_cbp")]
pub fn main() {
    kirby::install();
    marth::install();
    villager::install();
}
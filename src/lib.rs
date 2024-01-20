#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod kazuya;
mod kirby;
mod marth;
mod villager;

#[skyline::main(name = "ultimate_cbp")]
pub fn main() {
    kazuya::install();
    kirby::install();
    marth::install();
    villager::install();
}
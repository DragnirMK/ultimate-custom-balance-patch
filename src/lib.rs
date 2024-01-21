#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod ike;
mod kazuya;
mod kirby;
mod marth;
mod pichu;
mod villager;

#[skyline::main(name = "ultimate_cbp")]
pub fn main() {
    ike::install();
    kazuya::install();
    kirby::install();
    marth::install();
    pichu::install();
    villager::install();
}
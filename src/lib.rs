#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod bowserjr;
mod ike;
mod isabelle;
mod kazuya;
mod kirby;
mod marth;
mod pichu;
mod plant;
mod villager;

#[skyline::main(name = "ultimate_cbp")]
pub fn main() {
    bowserjr::install();
    ike::install();
    isabelle::install();
    kazuya::install();
    kirby::install();
    marth::install();
    pichu::install();
    plant::install();
    villager::install();
}
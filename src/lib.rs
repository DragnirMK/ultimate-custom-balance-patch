#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod bowserjr;
mod chrom;
mod duckhunt;
mod falcon;
mod ike;
mod inkling;
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
    chrom::install();
    duckhunt::install();
    falcon::install();
    ike::install();
    inkling::install();
    isabelle::install();
    kazuya::install();
    kirby::install();
    marth::install();
    pichu::install();
    plant::install();
    villager::install();
}
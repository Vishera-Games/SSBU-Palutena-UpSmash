#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod palutena;

#[skyline::main(name = "PalutenaLightMode")]
pub fn main() {
    palutena::install();
}
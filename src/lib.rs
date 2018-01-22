#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;

mod render_with_layout;
pub mod models;
pub mod apps;

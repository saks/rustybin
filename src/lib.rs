#![cfg(feature = "server")]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;

pub mod apps;
pub mod models;
mod render_with_layout;

pub mod redis;
pub mod server;

pub use self::server::server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod cli;
mod commands;
pub mod helpers;
pub mod repositories;

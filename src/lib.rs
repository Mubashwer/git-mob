pub type Result<T> = anyhow::Result<T>;

pub mod cli;
mod commands;
pub mod helpers;
pub mod repositories;

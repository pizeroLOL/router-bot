use utils::PipeOps;

pub mod adapters;
pub mod api;
pub mod error;
pub mod models;

pub mod utils;

impl<T> PipeOps for T {}

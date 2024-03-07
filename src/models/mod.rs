mod config;
mod field;
mod table;
mod form;
mod param;

pub use param::Param;

pub type Error = Box<dyn std::error::Error>;

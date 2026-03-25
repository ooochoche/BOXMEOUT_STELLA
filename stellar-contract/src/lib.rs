#![no_std]

pub mod amm;
pub mod errors;
pub mod events;
pub mod math;
pub mod prediction_market;
pub mod storage;
pub mod types;

#[cfg(test)]
mod test;

pub use prediction_market::PredictionMarketContract;

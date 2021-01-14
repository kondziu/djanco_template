use djanco_ext::query;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;

pub mod butter;

#[query(May, 2020, seed(42))]
pub fn xxxx1(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}

#[query(May, 2020, subset(C, "C++"), subset(Python))]
pub fn xxxx2(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}

fn _this_one_should_be_ommitted(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}
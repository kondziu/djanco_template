use djanco_ext::djanco;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;
use std::path::Path;

pub mod butter;

#[djanco(May, 2020, seed(42))]
pub fn xxxx1(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}

#[djanco(May, 2020, subset(C, "C++"), subset(Python))]
pub fn xxxx2(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}

fn _this_one_should_be_ommitted(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}
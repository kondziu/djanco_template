use djanco_ext::djanco;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;
use std::path::Path;

#[djanco(December, 2019, seed(12))]
pub fn queryrrr(_x: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}
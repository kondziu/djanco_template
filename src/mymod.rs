use djanco_ext::djanco;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;

#[djanco(December, 2019, seed(12))]
pub fn queryrrr(_x: &Database, _log: &Log) -> Result<()> {
    Ok(())
}
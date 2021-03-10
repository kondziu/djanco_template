use djanco_ext::djanco;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;

#[djanco(May, 2020)]
pub fn xxxx(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}

#[djanco(May, 2020)]
pub fn not_omitted(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}
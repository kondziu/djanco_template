use djanco_ext::query;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;

#[query(May, 2020)]
pub fn xxxx(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}

#[query(May, 2020)]
pub fn not_omitted(_db: &Database, _log: &Log) -> Result<()> {
    Ok(())
}
use djanco_ext::djanco;
use djanco::data::Database;
use djanco::log::Log;
use anyhow::*;
use std::path::Path;

#[djanco(May, 2020)]
pub fn xxxx(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}

#[djanco(May, 2020)]
pub fn not_omitted(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    Ok(())
}
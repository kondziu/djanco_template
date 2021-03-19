pub mod mymod;
pub mod butts;

use djanco_ext::*;
use djanco::*;
use djanco::data::Database;
use djanco::log::Log;

use anyhow::*;
use std::path::Path;

#[djanco(May, 2020)]
pub fn hello_world(db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    db.projects().count();
    bail!("oh noes!")
}

pub fn hello_world2(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
    unimplemented!();
    Ok(())
}

pub mod inner {
    use djanco_ext::*;
    use djanco::data::Database;
    use djanco::log::Log;
    use anyhow::*;
    use std::path::Path;

    #[djanco(May, 2020)]
    pub fn hello_world(db: &Database, _log: &Log, _output: &Path) -> Result<()> {
        db.projects().count();
        Ok(())
    }

    pub fn hello_world2(_db: &Database, _log: &Log, _output: &Path) -> Result<()> {
        unimplemented!();
        Ok(())
    }
}
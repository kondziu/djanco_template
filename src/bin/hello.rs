// use std::io::Write;

use clap::{Clap, crate_name};

use djanco::*;
use djanco::log::*;
use djanco::utils::*;

use djanco_template;

// These are automatically generated for the query.
const PROJECT_NAME: &'static str = crate_name!();
const SAVEPOINT: i64 = 1606780800; // 1st December 2020
const SUBSTORES: [Store; 1] = [Store::Large(store::Language::JavaScript)];

pub fn main() {
    let options = CommandLineOptions::parse();

    let repository = if options.archive {
        Some(create_project_archive(PROJECT_NAME, options.repository.as_str()))
    } else {
        None
    };

    let log = Log::new(options.verbosity);
    let database = Djanco::from_spec(
        options.dataset_path_as_str(),
        options.cache_path_as_str(),
        SAVEPOINT,
        SUBSTORES.iter().map(|store| store.clone()).collect(),
        log.clone()
    ).expect("Error initializing Djanco!");

    macro_rules! execute_query {
        ($method:path) => {
            timed_query!($method[&database, &log, &options.output_path]);
        }
    }

    init_timing_log!();
    execute_query!(djanco_template::hello_world);
    execute_query!(djanco_template::inner::hello_world);
    execute_query!(djanco_template::mymod::queryrrr);
    execute_query!(djanco_template::butts::xxxx1);
    execute_query!(djanco_template::butts::xxxx2);
    execute_query!(djanco_template::butts::butter::not_omitted);
    execute_query!(djanco_template::butts::butter::xxxx);

    if options.archive && !options.do_not_archive_results {
        add_results(PROJECT_NAME, &repository.unwrap(), &options.output_path, options.size_limit);
    }
}


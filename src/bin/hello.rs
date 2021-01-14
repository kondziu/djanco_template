use djanco::log::{Log, Verbosity};
use djanco::data::Database;
use std::path::PathBuf;
use clap::Clap;
use djanco::Djanco;
#[macro_use] use djanco::*;
use djanco_template;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;

#[derive(Clap)]
#[clap(version = "1.0", author = "Konrad Siek <konrad.siek@gmail.com>")]
struct CommandLineOptions {
    // #[clap(short = 'o', long = "output-path", alias = "output-dir", parse(from_os_str))]
    // pub output_path: Option<PathBuf>,
    //
    // #[clap(short = 'c', long = "cache-path", parse(from_os_str))]
    // pub cache_path: Option<PathBuf>,
    //
    // #[clap(short = 'd', long = "dataset-path", parse(from_os_str))]
    // pub dataset_path: Option<PathBuf>,

    #[clap(long = "archive")]
    pub archive: bool,

    // #[clap(name="FILE", parse(from_os_str))]
    // pub inputs: Vec<PathBuf>,
    //
    // #[clap(long = "as-json")]
    // pub json: bool,
    //
    // #[clap(long = "as-yaml")]
    // pub yaml: bool,
    //
    // #[clap(long = "as-sexpr", alias = "as-lisp")]
    // pub lisp: bool,
}

macro_rules! init_timing_log {
    () => {{
        let mut timing_log = File::create("timing.csv")
            .expect("Cannot create a timing log");
        writeln!(timing_log, "query, elapsed seconds, error")
            .expect("Cannot write to timing log.");
    }}
}

macro_rules! timed_query {
        ($method:path, $database:expr, $log:expr) => {{
            let name: &str = std::stringify!($method);

            eprintln!("Starting query {}...", name);
            let start = std::time::Instant::now();
            let result = $method($database, $log);
            let elapsed_secs = start.elapsed().as_secs();
            eprintln!("Finished query {} in {}s", name, elapsed_secs);

            let error = result.map_or_else(
                |error| { format!("\"{}\"", error) },
                |ok: _| { String::new()            },
            );

            let mut timing_log = OpenOptions::new()
                .append(true)
                .open("timing.csv")
                .expect("Cannot open timing log for appending");

            writeln!(timing_log, "{}, {}, {}", name, elapsed_secs, error)
                .expect("Cannot write to timing log.");

            timing_log.flush()
                .expect("Cannot flush timing log.");
        }}
    }

const DATASET_PATH: &'static str = "/dejacode/tiny-mk2/";
const CACHE_PATH: &'static str = "/dejacode/djanco/cache/tiny-mk2";
//const OUTPUT_PATH: &'static str = "/dejacode/djanco/output-tiny-mk2/";
const SAVEPOINT: i64 = 1606780800; // == 1st December 2020
const SUBSTORES: [&'static str; 4] = ["C++", "C", "Python", "SmallProjects"];
const LOG_LEVEL: Verbosity = Verbosity::Debug;

pub fn main() {
    let options = CommandLineOptions::parse();
    if options.archive {
        // add this project at this commit into our GH repo for repro purposes
    }

    let log = Log::new(LOG_LEVEL);
    let database = Djanco::from_spec(
        DATASET_PATH,
        CACHE_PATH,
        SAVEPOINT,
        SUBSTORES.iter().map(|e| e.to_string()).collect()
    );

    macro_rules! execute_query {
        ($method:path) => { timed_query!($method, &database, &log); }
    }

    init_timing_log!();
    execute_query!(djanco_template::hello_world);
    execute_query!(djanco_template::inner::hello_world);
    execute_query!(djanco_template::mymod::queryrrr);
    execute_query!(djanco_template::butts::xxxx1);
    execute_query!(djanco_template::butts::xxxx2);
    execute_query!(djanco_template::butts::butter::not_omitted);
    execute_query!(djanco_template::butts::butter::xxxx);

    // compile elapsed times, check for successes, write all that to a file   if options.archive() {
    // add the results of the query into our GH repo or another location for repro purposes
}
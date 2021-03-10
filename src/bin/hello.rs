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
use std::env::temp_dir;
use git2::{BranchType, Branch};

// These are automatically generated for the query.
const PROJECT_NAME: &'static str = "djanco_template";
const DATASET_PATH: &'static str = "/data/djcode/example/dataset/";
const CACHE_PATH: &'static str = "/data/djcode/example/cache/";
const OUTPUT_PATH: &'static str = "/data/djcode/example/output/";
const SAVEPOINT: i64 = 1606780800; // 1st December 2020
const SUBSTORES: [Store; 1] = [Store::Large(store::Language::JavaScript)];
const LOG_LEVEL: Verbosity = Verbosity::Debug;
const REPRO_REPO: &'static str = "https://github.com/kondziu/repro-test.git"; // FIXME


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
        ($method:path[$database:expr, $log:expr]) => {{
            let name: &str = std::stringify!($method);

            eprintln!("Starting query {}...", name);
            let start = std::time::Instant::now();
            let result = $method($database, $log);
            let elapsed_secs = start.elapsed().as_secs();
            eprintln!("Finished query {} in {}s", name, elapsed_secs);

            let error = result.map_or_else(
                |error| { format!("\"{}\"", error) },
                |_    | { String::new()            },
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

pub fn main() {
    let options = CommandLineOptions::parse();
    //if options.archive {
    create_project_archive();
    //}

    let log = Log::new(LOG_LEVEL);
    let database = Djanco::from_spec(
        DATASET_PATH,
        CACHE_PATH,
        SAVEPOINT,
        SUBSTORES.iter().map(|store| store.clone()).collect(),
        log.clone()
    ).unwrap(); // TODO handle error

    macro_rules! execute_query {
        ($method:path) => {
            timed_query!($method[&database, &log]);
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

    // compile elapsed times, check for successes, write all that to a file   if options.archive() {
    // add the results of the query into our GH repo or another location for repro purposes
}

pub fn clone_repository(url: &str) -> (git2::Repository, PathBuf) {
    let repository_path = tempfile::tempdir()
        .expect("Cannot create a directory for repository").into_path();

    println!("Reproduction repository cloned into {:?} from {}", &repository_path, url);

    let repository = git2::Repository::clone(url, &repository_path)
        .expect(&format!("Cannot clone repository {} into directory {:?}", url, repository_path));

    (repository, repository_path)
}

pub fn find_or_create_branch<'a>(repository: &'a git2::Repository, url: &str, branch_name: &str) -> Branch<'a> {
    repository.find_branch(branch_name, BranchType::Local).map_or_else(
        |error| {
            println!("Creating new branch {} in repository {}", branch_name, url);

            let head = repository.head().unwrap();
            let head_oid = head.target().unwrap();
            let head_commit = repository.find_commit(head_oid).unwrap();

            repository.branch(PROJECT_NAME, &head_commit, false)
                .expect(&format!("Cannot create a new branch {} in repository {}",
                                 branch_name, url))
        },

        |branch| {
            println!("Found branch {} in repository {}", branch_name, url);
            branch
        },
    )
}

pub fn checkout_branch(repository: &git2::Repository, branch: &git2::Branch) {

    let branch_name = branch.name().unwrap().unwrap();
    let branch_spec = format!("refs/heads/{}", branch_name);

    repository.checkout_tree(&repository.revparse_single(&branch_spec).unwrap(), None).unwrap();
    repository.set_head(&branch_spec);
}

pub fn create_project_archive() {
    // add this project at this commit into our GH repo for repro purposes

    // git clone REPRO ../repro # clone repro archive into PATH
    // cd repro
    let (repository, repository_path) = clone_repository(REPRO_REPO);
    let branch = find_or_create_branch(&repository, REPRO_REPO, PROJECT_NAME);

    checkout_branch(&repository, &branch);

    unimplemented!()

    // let existing_branches = repo.branches(None)
    //     .expect(&format!("Cannot list branches in repository {}", REPRO_REPO));



    // let branch = repo.find_branch(PROJECT_NAME, BranchType::Local)
    //     .unwrap_or_else(|error| {
    //         println!("Creating new branch {} in repository {}", PROJECT_NAME, REPRO_REPO);
    //         repo.branch(PROJECT_NAME, &head_commit, false)
    //             .expect(&format!("Cannot create a new branch {} in repository {}",
    //                              PROJECT_NAME, REPRO_REPO))
    //     });



    // let obj = repo.revparse_single(&branch_spec).unwrap();
    // repo.checkout_tree(&repo.revparse_single(&branch_spec).unwrap(), None).unwrap();
    // repo.set_head(&branch_spec);

    // let already_exists = existing_branches
    //     .map(|branch| {
    //         branch
    //             .expect(&format!("Cannot read branch in repository {}", REPRO_REPO))
    //     })
    //     .map(|(branch, _branch_type)| {
    //         println!("{:?}", _branch_type);
    //         branch.name().map(|e| e.map(|e| e.to_owned()))
    //             .expect(&format!("Cannot read branch name in repository {}", REPRO_REPO))
    //     })
    //     .map(|branch_name| { println!("{:?}", branch_name); branch_name })
    //     .any(|branch_name| branch_name.map_or(false, |name| name == PROJECT_NAME));

    // let repository.
    //
    // if already_exists {
    //     repo.branch()
    // } else {
    //     repo.
    // }
    // git checkout -m "PACKAGE_NAME" # if already exists just checkout, if not, create and checkout
    // rm everything from repro
    // copy everything from PACKAGE_ROOT to repro
    // commit repro with message
    //
}
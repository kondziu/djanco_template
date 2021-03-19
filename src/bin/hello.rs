use djanco::log::{Log, Verbosity};
#[allow(unused_imports)] use djanco::data::Database;
use std::path::PathBuf;
use clap::Clap;
use djanco::Djanco;
#[allow(unused_attributes)]
#[macro_use] use djanco::*;
use djanco_template;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use git2::{BranchType, Branch};

// These are automatically generated for the query.
const PROJECT_NAME: &'static str = "djanco_template";                   // FIXME
const DATASET_PATH: &'static str = "/data/djcode/example/dataset/";     // FIXME
const CACHE_PATH: &'static str = "/data/djcode/example/cache/";         // FIXME
const SAVEPOINT: i64 = 1606780800; // 1st December 2020
const SUBSTORES: [Store; 1] = [Store::Large(store::Language::JavaScript)];
const LOG_LEVEL: Verbosity = Verbosity::Debug;
const REPRO_REPO: &'static str = "git@github.com:kondziu/repro-test.git"; // FIXME

#[derive(Clap)]
#[clap(version = "1.0", author = "Konrad Siek <konrad.siek@gmail.com>")]
struct CommandLineOptions {
    #[clap(short = 'o', long = "output-path", alias = "output-dir", parse(from_os_str))]
    pub output_path: PathBuf,

    // #[clap(short = 'c', long = "cache-path", parse(from_os_str))]
    // pub cache_path: Option<PathBuf>,
    //
    // #[clap(short = 'd', long = "dataset-path", parse(from_os_str))]
    // pub dataset_path: Option<PathBuf>,

    #[clap(long = "archive")]
    pub archive: bool,

    #[clap(long = "skip-results")]
    pub do_not_archive_results: bool,

    #[clap(long = "size-limit-mb")]
    pub size_limit: Option<u32>,

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
        ($method:path[$database:expr, $log:expr, $output:expr]) => {{
            let name: &str = std::stringify!($method);

            eprintln!("Starting query {}...", name);
            let start = std::time::Instant::now();
            let result = $method($database, $log, $output);
            let elapsed_secs = start.elapsed().as_secs();
            eprintln!("Finished query {} in {}s", name, elapsed_secs);

            let error = result.map_or_else(
                |error| { format!("\"{}\"", error) },
                |_    | { String::new()            },
            );

            let mut timing_log_path = $output.clone();
            timing_log_path.push("timing.csv");
            std::fs::create_dir(&$output)
                .expect(&format!("Cannot create directory {:?}.", &$output));

            let mut timing_log = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&timing_log_path)
                .expect(&format!("Cannot open timing log for appending {:?}.", timing_log_path));

            writeln!(timing_log, "{}, {}, {}", name, elapsed_secs, error)
                .expect(&format!("Cannot write to timing log {:?}.", timing_log_path));

            timing_log.flush()
                .expect(&format!("Cannot flush timing log {:?}.", timing_log_path));
        }}
    }

pub fn main() {
    let options = CommandLineOptions::parse();

    //if options.archive {
    let repository = create_project_archive(PROJECT_NAME, REPRO_REPO);
    //}

    let log = Log::new(LOG_LEVEL);
    let database = Djanco::from_spec(
        DATASET_PATH,
        CACHE_PATH,
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

    // compile elapsed times, check for successes, write all that to a file   if options.archive() {
    // add the results of the query into our GH repo or another location for repro purposes
    //if options.archive && !options.do_not_archive_results {
        add_results(PROJECT_NAME, &repository, &options.output_path, options.size_limit);
    //}

}

fn clone_repository(url: &str) -> (git2::Repository, PathBuf) {
    let repository_path = tempfile::tempdir()
        .expect("Cannot create a directory for repository").into_path();

    println!("Reproduction repository cloned into {:?} from {}", &repository_path, url);

    let git_config = git2::Config::open_default().unwrap();
    let mut credential_handler = git2_credentials::CredentialHandler::new(git_config);

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(move |url, username, allowed| {
        credential_handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let repository = builder.clone(url, &repository_path)
        .expect(&format!("Cannot clone repository {} into directory {:?}", url, repository_path));

    (repository, repository_path)
}

fn find_or_create_branch<'a>(repository: &'a git2::Repository, url: &str, branch_name: &str) -> Branch<'a> {
    repository.find_branch(branch_name, BranchType::Local).map_or_else(
        |_error| {
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

fn checkout_branch(repository: &git2::Repository, branch: &git2::Branch) {

    let branch_name = branch.name().unwrap().unwrap();
    let branch_spec = format!("refs/heads/{}", branch_name);

    repository.checkout_tree(&repository.revparse_single(&branch_spec).unwrap(), None).unwrap();
    repository.set_head(&branch_spec).unwrap();
}

fn wipe_repository_contents(repository_path: &PathBuf) {
    println!("Removing current contents of repository at {:?}", repository_path);
    std::fs::read_dir(&repository_path)
        .expect(&format!("Cannot read directory {:?}", repository_path))
        .map(|entry| {
            entry.expect(&format!("Cannot read entry from directory {:?}", repository_path))
        })
        .filter(|entry| entry.file_name() != ".git")
        .map(|entry| entry.path())
        .for_each(|path| {
            println!("  - {:?}", path);
            if path.is_dir() {
                std::fs::remove_dir_all(&path).expect(&format!("Cannot remove directory {:?}", path))
            } else {
                std::fs::remove_file(&path).expect(&format!("Cannot remove file {:?}", path))
            }
        });
}

fn populate_directory_from(repository_path: &PathBuf, project_path: &PathBuf) {
    println!("Populating directory {:?} from {:?}", repository_path, project_path);

    let copy_options = fs_extra::dir::CopyOptions::new();
    std::fs::read_dir(&project_path)
        .expect(&format!("Cannot read directory {:?}", repository_path))
        .map(|entry| {
            entry.expect(&format!("Cannot read entry from directory {:?}", repository_path))
        })
        .filter(|entry| entry.file_name() != ".git")
        .map(|entry| (entry.file_name(), entry.path()))
        .for_each(|(filename, source_path)| {
            let mut target_path = PathBuf::new();
            target_path.push(repository_path.clone());
            target_path.push(filename.to_str().unwrap().to_owned());

            println!("  - {:?} -> {:?}", source_path, target_path);
            if source_path.is_dir() {
                fs_extra::dir::copy(source_path, repository_path, &copy_options)
                    .expect("Failed to copy directory");
            } else {
                std::fs::copy(source_path, target_path)
                    .expect("Failed to copy file.");
            }
        });
}

fn commit_all<S>(repository: &git2::Repository, message: S) where S: Into<String> {
    let message = message.into();

    let signature = repository.signature().unwrap();
    let mut index = repository.index().unwrap();

    let mut status_options = git2::StatusOptions::new();
    status_options.include_ignored(false);
    status_options.include_untracked(true);
    status_options.recurse_untracked_dirs(true);
    let statuses = repository.statuses(Some(&mut status_options)).unwrap();

    let filenames = statuses.iter().map(|e| e.path().unwrap().to_owned());
    index.add_all(filenames, git2::IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repository.find_tree(tree_id).unwrap();

    let head = repository.head().unwrap();
    let head_oid = head.target().unwrap();
    let parent = repository.find_commit(head_oid).unwrap();

    repository.commit(Some("HEAD"), &signature, &signature, &message, &tree, &[&parent]).unwrap();
}

fn push<S>(repository: &git2::Repository, branch: S) where S: Into<String> {
    let mut remote = repository.find_remote("origin").expect("No `origin` remote in repository");

    let git_config = git2::Config::open_default().unwrap();
    let mut credential_handler = git2_credentials::CredentialHandler::new(git_config);

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(move |url, username, allowed| {
        credential_handler.try_next_credential(url, username, allowed)
    });

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.refspecs().for_each(|e| println!("{:?}", e.str()));

    remote.push(&[&format!("refs/heads/{}", branch.into())], Some(&mut push_options))
        .expect(&format!("Error pushing to {}", remote.url().unwrap()));
}

pub fn create_project_archive(project_name: &str, repository_url: &str) -> PathBuf {
    let (repository, repository_path) = clone_repository(repository_url);
    let branch = find_or_create_branch(&repository, repository_url, project_name);
    checkout_branch(&repository, &branch);
    wipe_repository_contents(&repository_path);
    populate_directory_from(&repository_path, &std::env::current_dir().unwrap());
    commit_all(&repository, project_name);
    push(&repository, project_name); // FIXME
    repository_path
}

pub fn add_results(project_name: &str, repository_path: &PathBuf, results_dir: &PathBuf, size_limit: Option<u32>) {
    let repository = git2::Repository::open(repository_path)
        .expect(&format!("Cannot re-open repository {:?}", repository_path));

    // // Find branch, mostly for pushing later.
    // let branch = find_or_create_branch(&repository, repository_url, project_name);
    //
    // // We should already be on this branch, but just in case.
    // checkout_branch(&repository, &branch);
    let size = fs_extra::dir::get_size(results_dir)
        .expect(&format!("Cannot measure size of directory {:?}", results_dir));

    if let Some(size_limit) = size_limit {
        if (size_limit as u64) * 1024 * 1024 < size {
            panic!("Size of {:?} [~{}MB] exceeds the output size limit of {}MB.",
                   results_dir, size / 1024 / 1024, size_limit);
        }
    }

    let mut output_in_repository = repository_path.clone();
    output_in_repository.push("output");

    std::fs::create_dir_all(&output_in_repository)
        .expect(&format!("Cannot create directory {:?}", output_in_repository));

    // fs_extra::dir::create(output_in_repository, false)
    //
    let copy_options = fs_extra::dir::CopyOptions::new();

    fs_extra::dir::copy(&results_dir, &output_in_repository, &copy_options)
        .expect(&format!("Failed to copy directory {:?} to {:?}",
                         results_dir, output_in_repository));

    // add -u, commit, push
    commit_all(&repository, &format!("{}: output", project_name));
    push(&repository, project_name); // FIXME
}
use std::{net::SocketAddr, path::PathBuf};

use clap::{self, Parser};

use libafl_bolts::core_affinity::Cores;

/// The commandline args this fuzzer accepts
#[derive(Debug, Parser)]
#[command(
    name = "zephyr_net_fuzzer",
    about = "A fuzzer for the TCP/IP stack of Zephyr",
    author = "Valentin Huber <contact@valentinhuber.me>"
)]
pub struct Cli {
    #[arg(
    short,
    long,
    value_parser = Cores::from_cmdline,
    help = "Spawn a client in each of the provided cores. Broker runs in the 0th core. 'all' to select all available cores. 'none' to run a client without binding to any core. eg: '1,2-4,6' selects the cores 1,2,3,4,6.",
    name = "CORES",
    default_value = "0"
    )]
    cores: Cores,

    #[arg(
        short,
        long,
        help = "Spawn n clients on each of the provided cores.",
        name = "OVERCOMMIT",
        default_value = "1"
    )]
    overcommit: usize,

    #[arg(
        short,
        long,
        action,
        help = "Only run a single iteration of the fuzzer. Overrides cores and overcommmit to 1 each.",
        name = "FUZZ_ONE"
    )]
    fuzz_one: bool,

    #[arg(
        short,
        long,
        action,
        help = "Only load/generate the corpus. Do not perform any fuzzing.",
        name = "LOAD_ONLY"
    )]
    load_only: bool,

    #[arg(
        short = 'p',
        long,
        help = "Choose the broker TCP port, default is 1337",
        name = "PORT",
        default_value = "1337"
    )]
    broker_port: u16,

    #[arg(short = 'a', long, help = "Specify a remote broker", name = "REMOTE")]
    remote_broker_addr: Option<SocketAddr>,

    #[arg(
        short,
        long,
        help = "Set the Zephyr executable path",
        name = "ZEPHYR_EXEC_PATH",
        required = true
    )]
    zephyr_exec_dir: PathBuf,

    #[arg(
        short,
        long,
        help = "Set the path for the monitor .toml file",
        name = "MONITOR_PATH",
        default_value = "/dev/null"
    )]
    monitor_path: PathBuf,

    #[arg(
        short,
        long,
        help = "Set the output directory, default is ./solutions",
        name = "SOLUTIONS_DIR",
        default_value = "./solutions"
    )]
    solutions_dir: PathBuf,

    #[arg(
        short,
        long,
        help = "Store the corpus on disk at the specified part",
        name = "CORPUS_DIR"
    )]
    corpus_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        action,
        help = "Force clear the corpus if it already exists",
        name = "CLEAR_CORPUS"
    )]
    clear_corpus: bool,

    #[arg(
        short,
        long,
        action,
        help = "Resume with corpus from last run. Requires that the corpus direction is specified using --corpus-dir",
        name = "RESUME"
    )]
    resume: bool,

    #[arg(short, long, help = "Set the stdout path", name = "STDOUT")]
    stdout: Option<PathBuf>,

    #[arg(short, long, help = "Set the stderr path", name = "STDERR")]
    stderr: Option<PathBuf>,
}

impl Cli {
    pub fn remote_broker_addr(&self) -> Option<SocketAddr> {
        self.remote_broker_addr
    }

    pub fn broker_port(&self) -> u16 {
        self.broker_port
    }

    pub fn cores(&self) -> &Cores {
        &self.cores
    }

    pub fn zephyr_exec_dir(&self) -> &PathBuf {
        &self.zephyr_exec_dir
    }

    pub fn monitor_path(&self) -> &PathBuf {
        &self.monitor_path
    }

    pub fn stdout(&self) -> Option<&PathBuf> {
        self.stdout.as_ref()
    }

    pub fn stderr(&self) -> Option<&PathBuf> {
        self.stderr.as_ref()
    }

    pub fn solutions_dir(&self) -> &PathBuf {
        &self.solutions_dir
    }

    pub fn corpus_dir(&self) -> Option<&PathBuf> {
        self.corpus_dir.as_ref()
    }

    pub fn resume(&self) -> bool {
        self.resume
    }

    pub fn overcommit(&self) -> usize {
        self.overcommit
    }

    pub fn fuzz_one(&self) -> bool {
        self.fuzz_one
    }

    pub fn load_only(&self) -> bool {
        self.load_only
    }

    pub fn clear_corpus(&self) -> bool {
        self.clear_corpus
    }
}
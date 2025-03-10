use crate::common::reporter::TestResultReporter;
use cargo_msrv::check::RustupToolchainCheck;
use cargo_msrv::cli::CargoCli;
use cargo_msrv::config::test_config_from_cli;
use cargo_msrv::errors::TResult;
use cargo_msrv::{SubCommand, Verify};
use rust_releases::{Release, ReleaseIndex};
use std::ffi::OsString;
use std::iter::FromIterator;

pub fn run_verify<I, T, S>(with_args: I, releases: S) -> TResult<()>
where
    T: Into<OsString> + Clone,
    I: IntoIterator<Item = T>,
    S: IntoIterator<Item = Release>,
{
    let matches = CargoCli::parse_args(with_args);
    let config = test_config_from_cli(&matches).expect("Unable to parse cli arguments");

    // Limit the available versions: this ensures we don't need to incrementally install more toolchains
    //  as more Rust toolchains become available.
    let available_versions = ReleaseIndex::from_iter(releases);

    let reporter = TestResultReporter::default();
    let runner = RustupToolchainCheck::new(&reporter);

    // Determine the MSRV from the index of available releases.
    let cmd = Verify::new(&available_versions, runner);

    cmd.run(&config, &reporter)
}

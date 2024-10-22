use std::ops::Not;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::process::Termination;

use crate::report::FnResult;
use crate::report::Report;
use crate::tcx::TestContext;

use clap::Parser;
use colored::ColoredString;
use colored::Colorize;
use regex::RegexSet;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn setup_tracing() {
    use std::io::IsTerminal;
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::from_default_env();
    let enable_color = std::io::stdout().is_terminal();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init();
}

#[derive(Debug, Parser)]
struct Opt {
    #[clap(long)]
    json: Option<PathBuf>,

    #[clap(long)]
    filter: Vec<String>,
}

fn status(passed: bool) -> ColoredString {
    if passed {
        "PASSED".green()
    } else {
        "FAILED".red()
    }
}

fn write_report(json_path: &Path, report: &Report) -> Result<(), StdError> {
    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write(json_path, report_json)?;
    Ok(())
}

fn print_summary(report: &Report) {
    let w = format!("{:.3}", report.duration_ms).len();

    for suite in &report.suites {
        let suite_name = suite.name.as_str().magenta();
        for fixture in &suite.fixtures {
            let fixture_name = fixture.name.as_str().blue();
            for case in &fixture.cases {
                let case_name = case.name.as_str().cyan();
                let status = status(case.passed);
                let duration = case.duration_ms;
                println!("{status} {duration:>w$.3}ms [{suite_name}/{fixture_name}/{case_name}]");
                if !case.passed {
                    if let Some(ref run) = case.run {
                        let hint = match run.result {
                            FnResult::Ok => "".normal(),
                            FnResult::Err(_) => "ERROR".red(),
                            FnResult::Panicked => "PANICKED".red().bold(),
                        };
                        let msg = if let FnResult::Err(ref e) = run.result {
                            e.as_str()
                        } else {
                            ""
                        };
                        println!("  {hint} {msg}");
                    }
                }
            }
            let status = status(fixture.case_count.all_passed());
            let duration = fixture.duration_ms;
            println!("{status} {duration:>w$.3}ms [{suite_name}/{fixture_name}]");
        }
        let status = status(suite.fixture_count.all_passed());
        let duration = suite.duration_ms;
        println!("{status} {duration:>w$.3}ms [{suite_name}]");
    }
    let status = status(report.suite_count.all_passed());
    let duration = report.duration_ms;
    println!("{status} {duration:>w$.3}ms");
}

#[tokio::main]
async fn async_main(opt: &Opt, register: impl FnOnce(&mut TestContext)) -> ExitCode {
    let mut tcx = TestContext::new();
    register(&mut tcx);

    if opt.filter.is_empty().not() {
        let filter_set = match RegexSet::new(&opt.filter) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("Failed to build filter set: {err}");
                return ExitCode::from(2);
            }
        };
        tcx.filter(&filter_set);
    }

    let report = crate::runner::run(&mut tcx).await;

    if let Some(ref json_path) = opt.json {
        if let Err(err) = write_report(json_path, &report) {
            eprintln!("Failed to write report: {err}");
            return ExitCode::from(2);
        }
    }

    print_summary(&report);

    if report.suite_count.all_passed() {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    }
}

#[must_use]
pub fn main(register: impl FnOnce(&mut TestContext)) -> impl Termination {
    dotenvy::dotenv().ok();
    setup_tracing();
    let opt = Opt::parse();
    async_main(&opt, register)
}

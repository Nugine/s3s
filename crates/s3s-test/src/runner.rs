#![allow(clippy::wildcard_imports)]
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::time::Instant;

use crate::report::*;
use crate::tcx::*;

use tracing::info;
use tracing::instrument;

macro_rules! run_fn {
    ($call:expr) => {{
        let t0 = std::time::Instant::now();
        let result = $call.await;
        let duration_ns = t0.elapsed().as_nanos() as u64;
        let duration_ms = duration_ns as f64 / 1e6;
        let summary = match result {
            Ok(_) => FnSummary {
                result: FnResult::Ok,
                duration_ns,
                duration_ms,
            },
            Err(ref e) => FnSummary {
                result: FnResult::Err(e.to_string()),
                duration_ns,
                duration_ms,
            },
        };
        (result, summary)
    }};
}

fn count(iter: impl IntoIterator<Item = bool>) -> CountSummary {
    let mut passed = 0;
    let mut failed = 0;
    for p in iter {
        if p {
            passed += 1;
        } else {
            failed += 1;
        }
    }
    CountSummary {
        total: passed + failed,
        passed,
        failed,
    }
}

pub async fn run(tcx: &mut TestContext) -> Report {
    let total_suites = tcx.suites.len();
    info!(total_suites, "Test start");

    let mut suites: Vec<SuiteReport> = Vec::with_capacity(tcx.suites.len());

    let t0 = Instant::now();

    for suite in tcx.suites.values() {
        let report = run_suite(suite).await;
        suites.push(report);
    }

    let duration_ns = t0.elapsed().as_nanos() as u64;
    let suite_count = count(suites.iter().map(|r| r.fixture_count.all_passed()));

    info!(duration_ns, ?suite_count, "Test end");

    Report {
        suite_count,
        duration_ns,
        duration_ms: duration_ns as f64 / 1e6,
        suites,
    }
}

#[instrument(skip(suite), fields(suite_name = suite.name))]
async fn run_suite(suite: &SuiteInfo) -> SuiteReport {
    let total_fixtures = suite.fixtures.len();
    info!(total_fixtures, "Test suite start");

    let setup_summary;
    let mut teardown_summary = None;
    let mut fixtures: Vec<FixtureReport> = Vec::with_capacity(suite.fixtures.len());

    let t0 = Instant::now();

    'run: {
        let (result, summary) = run_fn!((suite.setup)());
        setup_summary = Some(summary);
        let Ok(mut suite_data) = result else { break 'run };

        for fixture in suite.fixtures.values() {
            let report = run_fixture(fixture, &suite_data).await;
            fixtures.push(report);
        }

        let (_, summary) = run_fn!((suite.teardown)(&mut suite_data));
        teardown_summary = Some(summary);
    }

    let duration_ns = t0.elapsed().as_nanos() as u64;
    let fixture_count = count(fixtures.iter().map(|r| r.case_count.all_passed()));

    info!(duration_ns, ?fixture_count, "Test suite end");

    SuiteReport {
        name: suite.name.clone(),
        setup: setup_summary,
        teardown: teardown_summary,
        fixture_count,
        duration_ns,
        duration_ms: duration_ns as f64 / 1e6,
        fixtures,
    }
}

#[instrument(skip(fixture, suite_data), fields(fixture_name = fixture.name))]
async fn run_fixture(fixture: &FixtureInfo, suite_data: &ArcAny) -> FixtureReport {
    let total_cases = fixture.cases.len();
    info!(total_cases, "Test fixture start");

    let setup_summary;
    let mut teardown_summary = None;
    let mut cases: Vec<CaseReport> = Vec::with_capacity(fixture.cases.len());

    let t0 = Instant::now();

    'run: {
        info!("Test fixture setup");
        let (result, summary) = run_fn!((fixture.setup)(suite_data));
        setup_summary = Some(summary);
        let Ok(mut fixture_data) = result else { break 'run };

        for case in fixture.cases.values() {
            let report = run_case(case, &fixture_data).await;
            cases.push(report);
        }

        info!("Test fixture teardown");
        let (_, summary) = run_fn!((fixture.teardown)(&mut fixture_data));
        teardown_summary = Some(summary);
    }

    let duration_ns = t0.elapsed().as_nanos() as u64;
    let case_count = count(cases.iter().map(|r| r.passed));

    info!(duration_ns, ?case_count, "Test fixture end");

    FixtureReport {
        name: fixture.name.clone(),
        setup: setup_summary,
        teardown: teardown_summary,
        case_count,
        duration_ns,
        duration_ms: duration_ns as f64 / 1e6,
        cases,
    }
}

#[instrument(skip(case, fixture_data), fields(case_name = case.name))]
async fn run_case(case: &CaseInfo, fixture_data: &ArcAny) -> CaseReport {
    info!("Test case start");

    let t0 = Instant::now();
    let (_, summary) = run_fn!((case.run)(fixture_data));

    info!(?summary, "Test case end");

    let duration_ns = t0.elapsed().as_nanos() as u64;
    let duration_ms = duration_ns as f64 / 1e6;
    let passed = summary.result.is_ok();

    CaseReport {
        name: case.name.clone(),
        passed,
        duration_ns,
        duration_ms,
        run: Some(summary),
    }
}

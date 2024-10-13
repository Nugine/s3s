use crate::error::Failed;
use crate::error::Result;
use crate::traits::TestCase;
use crate::traits::TestFixture;
use crate::traits::TestSuite;

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use indexmap::IndexMap;

pub(crate) type ArcAny = Arc<dyn std::any::Any + Send + Sync + 'static>;
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

type SuiteSetupFn = Box<dyn Fn() -> BoxFuture<'static, Result<ArcAny, Failed>>>;
type SuiteTeardownFn = Box<dyn for<'a> Fn(&'a mut ArcAny) -> BoxFuture<'a, Result>>;

type FixtureSetupFn = Box<dyn for<'a> Fn(&'a ArcAny) -> BoxFuture<'a, Result<ArcAny, Failed>>>;
type FixtureTeardownFn = Box<dyn for<'a> Fn(&'a mut ArcAny) -> BoxFuture<'a, Result>>;

type CaseRunFn = Box<dyn for<'a> Fn(&'a ArcAny) -> BoxFuture<'a, Result>>;

pub struct TestContext {
    pub(crate) suites: IndexMap<String, SuiteInfo>,
}

pub(crate) struct SuiteInfo {
    pub(crate) name: String,
    pub(crate) setup: SuiteSetupFn,
    pub(crate) teardown: SuiteTeardownFn,
    pub(crate) fixtures: IndexMap<String, FixtureInfo>,
}

pub(crate) struct FixtureInfo {
    pub(crate) name: String,
    pub(crate) setup: FixtureSetupFn,
    pub(crate) teardown: FixtureTeardownFn,
    pub(crate) cases: IndexMap<String, CaseInfo>,
}

pub(crate) struct CaseInfo {
    pub(crate) name: String,
    pub(crate) run: CaseRunFn,
    pub(crate) tags: Vec<CaseTag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseTag {
    Ignored,
    ShouldPanic,
}

fn downcast_ref<T: 'static>(any: &ArcAny) -> &T {
    (*any).downcast_ref().unwrap()
}

fn downcast_mut<T: 'static>(any: &mut ArcAny) -> &mut T {
    Arc::get_mut(any).unwrap().downcast_mut().unwrap()
}

impl TestContext {
    pub(crate) fn new() -> Self {
        Self { suites: IndexMap::new() }
    }

    pub fn suite<S: TestSuite>(&mut self, name: impl Into<String>) -> SuiteBuilder<'_, S> {
        let name = name.into();
        if !self.suites.contains_key(&name) {
            self.suites.insert(
                name.clone(),
                SuiteInfo {
                    name: name.clone(),
                    setup: Box::new(|| Box::pin(async { S::setup().await.map(|x| Arc::new(x) as ArcAny) })),
                    teardown: Box::new(|any| Box::pin(S::teardown(downcast_mut(any)))),
                    fixtures: IndexMap::new(),
                },
            );
        }
        SuiteBuilder {
            suite: &mut self.suites[&name],
            _marker: PhantomData,
        }
    }
}

pub struct SuiteBuilder<'a, S> {
    suite: &'a mut SuiteInfo,
    _marker: PhantomData<S>,
}

impl<S: TestSuite> SuiteBuilder<'_, S> {
    pub fn fixture<X: TestFixture<S>>(&mut self, name: impl Into<String>) -> FixtureBuilder<'_, X, S> {
        let name = name.into();
        if !self.suite.fixtures.contains_key(&name) {
            self.suite.fixtures.insert(
                name.clone(),
                FixtureInfo {
                    name: name.clone(),
                    setup: Box::new(|any| {
                        Box::pin(async move { X::setup(downcast_ref(any)).await.map(|x| Arc::new(x) as ArcAny) })
                    }),
                    teardown: Box::new(|any| Box::pin(X::teardown(downcast_mut(any)))),
                    cases: IndexMap::new(),
                },
            );
        }
        FixtureBuilder {
            fixture: &mut self.suite.fixtures[&name],
            _marker: PhantomData,
        }
    }
}

pub struct FixtureBuilder<'a, X, S> {
    fixture: &'a mut FixtureInfo,
    _marker: PhantomData<(X, S)>,
}

impl<X, S> FixtureBuilder<'_, X, S>
where
    X: TestFixture<S>,
    S: TestSuite,
{
    pub fn case<C: TestCase<X, S>>(&mut self, name: impl Into<String>, case: C) -> CaseBuilder<'_, C, X, S> {
        let name = name.into();
        self.fixture.cases.insert(
            name.clone(),
            CaseInfo {
                name: name.clone(),
                run: Box::new(move |any| Box::pin(case.run(downcast_ref(any)))),
                tags: Vec::new(),
            },
        );
        CaseBuilder {
            case: &mut self.fixture.cases[&name],
            _marker: PhantomData,
        }
    }
}

pub struct CaseBuilder<'a, C, X, S> {
    case: &'a mut CaseInfo,
    _marker: PhantomData<(C, X, S)>,
}

impl<C, X, S> CaseBuilder<'_, C, X, S> {
    pub fn tag(&mut self, tag: CaseTag) -> &mut Self {
        self.case.tags.push(tag);
        self
    }
}

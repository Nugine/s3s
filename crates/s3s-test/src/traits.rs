use crate::error::Result;

use std::future::Future;

pub trait TestSuite: Sized + Send + Sync + 'static {
    fn setup() -> impl Future<Output = Result<Self>> + Send + 'static;

    fn teardown(&mut self) -> impl Future<Output = Result> + Send + '_ {
        async { Ok(()) }
    }
}

pub trait TestFixture<S: TestSuite>: Sized + Send + Sync + 'static {
    fn setup(suite: &S) -> impl Future<Output = Result<Self>> + Send + '_;

    fn teardown(&mut self) -> impl Future<Output = Result> + Send + '_ {
        async { Ok(()) }
    }
}

pub trait TestCase<X, S>: Sized + Send + Sync + 'static
where
    Self: Sized + Send + Sync + 'static,
    X: TestFixture<S>,
    S: TestSuite,
{
    fn run<'a>(&self, fixture: &'a X) -> impl Future<Output = Result> + Send + 'a;
}

trait AsyncFn<'a, A> {
    type Output;
    type Future: Future<Output = Self::Output> + Send + 'a;

    fn call(&self, args: A) -> Self::Future;
}

impl<'a, F, U, O, A> AsyncFn<'a, (A,)> for F
where
    F: Fn(A) -> U,
    U: Future<Output = O> + Send + 'a,
{
    type Output = O;

    type Future = U;

    fn call(&self, args: (A,)) -> Self::Future {
        (self)(args.0)
    }
}

impl<C, X, S> TestCase<X, S> for C
where
    C: for<'a> AsyncFn<'a, (&'a X,), Output = Result>,
    C: Send + Sync + 'static,
    X: TestFixture<S>,
    S: TestSuite,
{
    fn run<'a>(&self, fixture: &'a X) -> impl Future<Output = Result> + Send + 'a {
        AsyncFn::call(self, (fixture,))
    }
}

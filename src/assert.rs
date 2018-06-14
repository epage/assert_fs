use std::path;

use predicates;
use predicates::path::PredicateFileContentExt;

use fs;

/// Assert the state of files within `TempDir`.
///
/// # Examples
///
/// ```rust,ignore
/// use assert_fs::*;
/// use predicates::*;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
/// // ... do something with input_file ...
/// input_file.assert(predicate::str::is_empty().from_utf8());
/// temp.child("bar.txt").assert(predicate::path::missing());
/// temp.close().unwrap();
/// ```
pub trait PathAssert {
    /// Wrap with an interface for that provides assertions on the `TempDir`.
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>;
}

impl PathAssert for fs::TempDir {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for fs::ChildPath {
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoPathPredicate<P>,
        P: predicates::Predicate<path::Path>,
    {
        assert(self.path(), pred);
        self
    }
}

fn assert<I, P>(path: &path::Path, pred: I)
where
    I: IntoPathPredicate<P>,
    P: predicates::Predicate<path::Path>,
{
    let pred = pred.into_path();
    if !pred.eval(path) {
        panic!("Predicate {} failed for {:?}", pred, path);
    }
}

/// Used by `PathAssert` to convert Self into the needed `Predicate<Path>`.
pub trait IntoPathPredicate<P>
where
    P: predicates::Predicate<path::Path>,
{
    /// Convert to a predicate for testing a path.
    fn into_path(self) -> P;
}

impl<P> IntoPathPredicate<P> for P
where
    P: predicates::Predicate<path::Path>,
{
    fn into_path(self) -> P {
        self
    }
}

impl<P> IntoPathPredicate<predicates::path::FileContentPredicate<P>> for P
where
    P: predicates::Predicate<[u8]>,
{
    fn into_path(self) -> predicates::path::FileContentPredicate<P> {
        self.from_file_path()
    }
}

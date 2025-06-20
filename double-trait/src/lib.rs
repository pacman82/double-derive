// Reexport the double macro from our derive crate
pub use double_derive::double;

/// An empty Dummy type. Each trait generated by the [`double`] macro will be implemented for this
/// type.
///
/// ```no_run
/// use double_trait::{double, Dummy};
///
/// #[double(DoubleTrait)]
/// trait OrgTrait {
///     fn answer(&self) -> i32;
/// }
///
/// OrgTrait::answer(&Dummy); // Compiles, but raises panic with `unimplemented!()`
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Dummy;

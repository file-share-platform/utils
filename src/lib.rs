//! This package aims to collate all of the utility tools used by different elements of the file-share-platform
//! All features are togglable via flags, by default all features are turned on. It is recommended to specify
//! no-default-features when importing this crate.

#[cfg(not(any(
    feature = "database",
    feature = "derive-macros",
    feature = "hash",
    feature = "hex",
    feature = "macros"
)))]
#[cfg(not(debug_assertions))]
compile_error!("ERROR: This crate does nothing unless you specify a flag! This error will only be shown when building for release. The utils dependency is not being used and should be removed.");

// #[cfg(feature = "database")]
// pub mod database;
// #[cfg(feature = "derive-macros")]
// pub mod derive_macros;
#[cfg(feature = "hash")]
pub mod hash;
#[cfg(feature = "hex")]
pub mod hex;
// #[cfg(feature = "macros")]
// pub mod macros;

#[cfg(test)]
mod tests {}

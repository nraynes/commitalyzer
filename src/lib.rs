//! Simple commit linter. Compares a commit message against a set of pre-defined
//! rules. Designed to be used in git hook to lint commit messages.
//!
//! To install, just grab the binary and put in your preferred location. Make a git
//! hook that calls the binary and supplies it with the commit message as the first
//! argument, and a path to the directory containing the rulesets to use for comparison.
mod api;
mod utils;

pub use crate::api::{analyze_rules, load_rules, parse_args};

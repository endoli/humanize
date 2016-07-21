// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Human-friendly Parsing
//!
//! This module provides support for finding human-friendly values
//! within some source text. This is typically used for parsing
//! input text for prompts that need common data types such as
//! instants in time, durations, booleans, etc.
//!
//! # Upcoming Breaking Changes
//!
//! The system of using `ValueType` and `HumanValue` as seen in
//! the current implementation of this library will be going away
//! in favor of something else (and depending on another crate)
//! in the near future so that we can interoperate with other
//! things that need a richer type system.
//!
//! # Match Input
//!
//! First, you'll want to construct a parser:
//!
//! ```
//! use humanize::parse::Parser;
//!
//! let parser = Parser::new();
//! ```
//!
//! Then, you can use that parser to examine some input. You'll want
//! to specify what [sort of value] you are looking for. You may also
//! limit the matchers run to a specific language. (Here, we don't limit
//! the languages, so we pass `Default::default()`.)
//!
//! ```
//! # use humanize::parse::{HumanValue, Parser, ValueType};
//! #
//! # let parser = Parser::new();
//! let matches = parser.parse("on", ValueType::Boolean, Default::default());
//! assert_eq!(matches.len(), 1);
//! assert_eq!(matches[0].value, HumanValue::Boolean(true));
//! ```
//!
//! # Register Matchers
//!
//! Matchers can be provided to augment the built-in capabilities
//! of this library.
//!
//! _We will expand upon this in the future once our own infrastructure
//! for doing matchers well is in place._
//!
//! ...
//!
//! [sort of value]: trait.ValueType.html

use language_tags::LanguageTag;
use std::time::{Duration, Instant};

pub mod english;

/// A location within a body of text.
#[derive(Debug)]
pub struct SourceLocation {
    /// The starting offset into the text of the match.
    pub start: usize,

    /// The ending offset into the text of the match.
    pub end: usize,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    Boolean,
    Duration,
    Instant,
    Integer,
    Ordinal,
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum HumanValue {
    Boolean(bool),
    Duration(Duration),
    Instant(Instant),
    Integer(i64),
    Ordinal(i64),
}

/// A possible match for a value within some text.
///
/// A `Match` result is obtained by calling [`parse`] on
/// some input text.
///
/// [`parse`]: fn.parse.html
#[derive(Debug)]
pub struct Match {
    /// The value determined for this match.
    pub value: HumanValue,

    /// Strength of the match.
    ///
    /// This is useful when there is more than one possible match.
    ///
    /// TODO: Should be this be a percentage? What is the range?
    /// Should it be an enum with values like 'Likely', 'Unlikely',
    /// and 'Certain'?
    pub weight: i32,

    /// Location of the match within the text.
    pub location: SourceLocation,
}

#[allow(missing_docs)]
pub struct Matcher<'m> {
    pub name: &'m str,
    pub language: LanguageTag,
    pub result_type: ValueType,
    pub matcher: Box<Fn(&str) -> Option<Match>>,
}

#[allow(missing_docs)]
pub struct Parser<'p> {
    /// The matchers which have been registered with this parser.
    ///
    /// Use `Parser.register_matcher` to add a new matcher.
    matchers: Vec<Matcher<'p>>,
}

impl<'p> Parser<'p> {
    /// Construct a new parser, including the default matchers.
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct a new parser, but without any of the default matchers.
    pub fn new_without_default_matchers() -> Self {
        Parser { matchers: vec![] }
    }

    #[allow(missing_docs)]
    pub fn register_matcher(&mut self, matcher: Matcher<'p>) {
        self.matchers.push(matcher);
    }

    /// Parse `text`, looking for a value of the [desired type],  using
    /// the optionally provided language.
    ///
    /// The resulting collection of matches will be ordered by their
    /// weight of likelihood with the most likely first.
    ///
    ///
    /// TODO: Should we return a `Result` with specific errors for
    /// things like 'No valid matchers'?
    ///
    /// [desired type]: enum.ValueType.html
    pub fn parse(&self, text: &str, desired: ValueType, language: LanguageTag) -> Vec<Match> {
        let mut matches = vec![];
        for matcher in &self.matchers {
            if matcher.result_type == desired && language.matches(&matcher.language) {
                if let Some(m) = (matcher.matcher)(text) {
                    matches.push(m);
                }
            }
        }
        matches
    }
}

impl<'p> Default for Parser<'p> {
    fn default() -> Self {
        let mut p = Parser { matchers: vec![] };
        english::register(&mut p);
        p
    }
}

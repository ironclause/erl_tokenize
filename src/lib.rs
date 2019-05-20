//! Erlang source code tokenizer.
//!
//! # Examples
//!
//! Tokenizes the Erlang code `io:format("Hello").`:
//!
//! ```
//! use erl_tokenize::Tokenizer;
//!
//! let src = r#"io:format("Hello")."#;
//! let tokenizer = Tokenizer::new(src);
//! let tokens = tokenizer.collect::<Result<Vec<_>, _>>().unwrap();
//!
//! assert_eq!(tokens.iter().map(|t| t.text()).collect::<Vec<_>>(),
//!            ["io", ":", "format", "(", r#""Hello""#, ")", "."]);
//! ```
//!
//! # References
//!
//! - [`erl_scan`][erl_scan] module
//! - [Erlang Data Types][Data Types]
//!
//! [erl_scan]: http://erlang.org/doc/man/erl_scan.html
//! [Data Types]: http://erlang.org/doc/reference_manual/data_types.html
#![warn(missing_docs)]

#[macro_use]
extern crate trackable;

pub use crate::error::{Error, ErrorKind};
pub use crate::hidden_token::HiddenToken;
pub use crate::lexer::Lexer;
pub use crate::lexical_token::LexicalToken;
pub use crate::position::{Position, PositionRange};
pub use crate::token::Token;
pub use crate::tokenizer::Tokenizer;

pub mod tokens;
pub mod values;

mod error;
mod hidden_token;
mod lexer;
mod lexical_token;
mod position;
mod token;
mod tokenizer;
mod util;

/// This crate specific `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;

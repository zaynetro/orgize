//! A Rust library for parsing orgmode files.
//!
//! Live demo: https://orgize.herokuapp.com/
//!
//! # Parse
//!
//! To parse a orgmode string, simply invoking the [`Org::parse`] function:
//!
//! [`Org::parse`]: org/struct.Org.html#method.parse
//!
//! ```rust
//! use orgize::Org;
//!
//! Org::parse("* DONE Title :tag:");
//! ```
//!
//! or [`Org::parse_with_config`]:
//!
//! [`Org::parse_with_config`]: org/struct.Org.html#method.parse_with_config
//!
//! ``` rust
//! use orgize::{Org, ParseConfig};
//!
//! Org::parse_with_config(
//!     "* TASK Title 1",
//!     &ParseConfig {
//!         // custom todo keywords
//!         todo_keywords: vec!["TASK".to_string()],
//!         ..Default::default()
//!     },
//! );
//! ```
//!
//! # Iter
//!
//! [`Org::iter`] function will returns an iteractor of [`Event`]s, which is
//! a simple wrapper of [`Element`].
//!
//! [`Org::iter`]: org/struct.Org.html#method.iter
//! [`Event`]: iter/enum.Event.html
//! [`Element`]: elements/enum.Element.html
//!
//! ```rust
//! use orgize::Org;
//!
//! for event in Org::parse("* DONE Title :tag:").iter() {
//!     // handling the event
//! }
//! ```
//!
//! **Note**: whether an element is container or not, it will appears twice in one loop.
//! One as [`Event::Start(element)`], one as [`Event::End(element)`].
//!
//! [`Event::Start(element)`]: iter/enum.Event.html#variant.Start
//! [`Event::End(element)`]: iter/enum.Event.html#variant.End
//!
//! # Render html
//!
//! You can call the [`Org::html`] function to generate html directly, which
//! uses the [`DefaultHtmlHandler`] internally:
//!
//! [`Org::html`]: org/struct.Org.html#method.html
//! [`DefaultHtmlHandler`]: export/html/struct.DefaultHtmlHandler.html
//!
//! ```rust
//! use orgize::Org;
//!
//! let mut writer = Vec::new();
//! Org::parse("* title\n*section*").html(&mut writer).unwrap();
//!
//! assert_eq!(
//!     String::from_utf8(writer).unwrap(),
//!     "<main><h1>title</h1><section><p><b>section</b></p></section></main>"
//! );
//! ```
//!
//! # Render html with custom `HtmlHandler`
//!
//! To customize html rendering, simply implementing [`HtmlHandler`] trait and passing
//! it to the [`Org::html_with_handler`] function.
//!
//! [`HtmlHandler`]: export/html/trait.HtmlHandler.html
//! [`Org::html_with_handler`]: org/struct.Org.html#method.html_with_handler
//!
//! The following code demonstrates how to add a id for every headline and return
//! own error type while rendering.
//!
//! ```rust
//! use std::convert::From;
//! use std::io::{Error as IOError, Write};
//! use std::string::FromUtf8Error;
//!
//! use orgize::export::{DefaultHtmlHandler, HtmlHandler};
//! use orgize::{Element, Org};
//! use slugify::slugify;
//!
//! #[derive(Debug)]
//! enum MyError {
//!     IO(IOError),
//!     Heading,
//!     Utf8(FromUtf8Error),
//! }
//!
//! // From<std::io::Error> trait is required for custom error type
//! impl From<IOError> for MyError {
//!     fn from(err: IOError) -> Self {
//!         MyError::IO(err)
//!     }
//! }
//!
//! impl From<FromUtf8Error> for MyError {
//!     fn from(err: FromUtf8Error) -> Self {
//!         MyError::Utf8(err)
//!     }
//! }
//!
//! struct MyHtmlHandler(DefaultHtmlHandler);
//!
//! impl HtmlHandler<MyError> for MyHtmlHandler {
//!     fn start<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
//!         if let Element::Title(title) = element {
//!             if title.level > 6 {
//!                 return Err(MyError::Heading);
//!             } else {
//!                 write!(
//!                     w,
//!                     "<h{0}><a id=\"{1}\" href=\"#{1}\">",
//!                     title.level,
//!                     slugify!(&title.raw),
//!                 )?;
//!             }
//!         } else {
//!             // fallthrough to default handler
//!             self.0.start(w, element)?;
//!         }
//!         Ok(())
//!     }
//!
//!     fn end<W: Write>(&mut self, mut w: W, element: &Element<'_>) -> Result<(), MyError> {
//!         if let Element::Title(title) = element {
//!             write!(w, "</a></h{}>", title.level)?;
//!         } else {
//!             self.0.end(w, element)?;
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> Result<(), MyError> {
//!     let mut writer = Vec::new();
//!     let mut handler = MyHtmlHandler(DefaultHtmlHandler);
//!     Org::parse("* title\n*section*").html_with_handler(&mut writer, &mut handler)?;
//!
//!     assert_eq!(
//!         String::from_utf8(writer)?,
//!         "<main><h1><a id=\"title\" href=\"#title\">title</a></h1>\
//!          <section><p><b>section</b></p></section></main>"
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! **Note**: as I mentioned above, each element will appears two times while iterating.
//! And handler will silently ignores all end events from non-container elements.
//!
//! So if you want to change how a non-container element renders, just redefine the `start`
//! function and leave the `end` function unchanged.
//!
//! # Serde
//!
//! `Org` struct have already implemented serde's `Serialize` trait. It means you can
//! serialize it into any format supported by serde, such as json:
//!
//! ```rust
//! use orgize::Org;
//! use serde_json::{json, to_string};
//!
//! let org = Org::parse("I 'm *bold*.");
//! println!("{}", to_string(&org).unwrap());
//!
//! // {
//! //     "type": "document",
//! //     "children": [{
//! //         "type": "section",
//! //         "children": [{
//! //             "type": "paragraph",
//! //             "children":[{
//! //                 "type": "text",
//! //                 "value":"I 'm "
//! //             }, {
//! //                 "type": "bold",
//! //                 "children":[{
//! //                     "type": "text",
//! //                     "value": "bold"
//! //                 }]
//! //             }, {
//! //                 "type":"text",
//! //                 "value":"."
//! //             }]
//! //         }]
//! //     }]
//! // }
//! ```
//!
//! # Features
//!
//! By now, orgize provides two features:
//!
//! + `ser`: adds the ability to serialize `Org` and other elements using `serde`, enabled by default.
//!
//! + `chrono`: adds the ability to convert `Datetime` into `chrono` structs, disabled by default.
//!
//! + `syntect`: provides `SyntectHtmlHandler` for highlighting code block, disabled by default.
//!
//! # License
//!
//! MIT

#![allow(clippy::range_plus_one)]

mod config;
pub mod elements;
pub mod export;
mod node;
mod org;
mod parsers;

mod error;

pub use config::ParseConfig;
pub use elements::Element;
pub use error::OrgizeError;
pub use node::{DocumentNode, HeadlineNode};
pub use org::{Event, Org};

//! Org-mode elements module

mod block;
mod clock;
mod cookie;
mod drawer;
mod dyn_block;
mod emphasis;
mod fn_def;
mod fn_ref;
mod inline_call;
mod inline_src;
mod keyword;
mod link;
mod list;
mod macros;
mod planning;
mod radio_target;
mod rule;
mod snippet;
mod target;
mod timestamp;
mod title;

pub(crate) use emphasis::parse as parse_emphasis;

pub use self::{
    block::Block,
    clock::Clock,
    cookie::Cookie,
    drawer::Drawer,
    dyn_block::DynBlock,
    fn_def::FnDef,
    fn_ref::FnRef,
    inline_call::InlineCall,
    inline_src::InlineSrc,
    keyword::{BabelCall, Keyword},
    link::Link,
    list::{List, ListItem},
    macros::Macros,
    planning::Planning,
    radio_target::RadioTarget,
    rule::Rule,
    snippet::Snippet,
    target::Target,
    timestamp::{Date, Time, Timestamp},
    title::Title,
};

/// Org-mode element enum
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum Element<'a> {
    Block(Block<'a>),
    BabelCall(BabelCall<'a>),
    Section,
    Clock(Clock<'a>),
    Cookie(Cookie<'a>),
    RadioTarget(RadioTarget),
    Drawer(Drawer<'a>),
    Document,
    DynBlock(DynBlock<'a>),
    FnDef(FnDef<'a>),
    FnRef(FnRef<'a>),
    Headline,
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Keyword(Keyword<'a>),
    Link(Link<'a>),
    List(List),
    ListItem(ListItem<'a>),
    Macros(Macros<'a>),
    Planning(Planning<'a>),
    Snippet(Snippet<'a>),
    Text { value: &'a str },
    Paragraph,
    Rule,
    Timestamp(Timestamp<'a>),
    Target(Target<'a>),
    Bold,
    Strike,
    Italic,
    Underline,
    Verbatim { value: &'a str },
    Code { value: &'a str },
    Comment { value: &'a str },
    FixedWidth { value: &'a str },
    Title(Title<'a>),
}

impl Element<'_> {
    pub fn is_container(&self) -> bool {
        match self {
            Element::Block(_)
            | Element::Bold
            | Element::Document
            | Element::DynBlock(_)
            | Element::Headline
            | Element::Italic
            | Element::List(_)
            | Element::ListItem(_)
            | Element::Paragraph
            | Element::Section
            | Element::Strike
            | Element::Underline
            | Element::Title(_) => true,
            _ => false,
        }
    }
}

macro_rules! impl_from {
    ($ident:ident) => {
        impl<'a> From<$ident<'a>> for Element<'a> {
            fn from(ele: $ident<'a>) -> Element<'a> {
                Element::$ident(ele)
            }
        }
    };
}

impl_from!(Block);
impl_from!(BabelCall);
impl_from!(Clock);
impl_from!(Cookie);
impl_from!(Drawer);
impl_from!(DynBlock);
impl_from!(FnDef);
impl_from!(FnRef);
impl_from!(InlineCall);
impl_from!(InlineSrc);
impl_from!(Keyword);
impl_from!(Link);
impl_from!(ListItem);
impl_from!(Macros);
impl_from!(Planning);
impl_from!(Snippet);
impl_from!(Timestamp);
impl_from!(Target);

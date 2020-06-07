use crate::attributes;
use crate::tag::Tag;
use crate::text::Text;
use attributes::Attribute;
use derive_more::{From, Into};
use std::collections::HashMap;
///! A loose representation of the tree structure HTML follows
///! This can still be used to generate 'invalid' html.
///! Eg a br tag with children would be possible. However most browsers
///! are incredibly lax, as is HTML somewhat, so this is at least highly useable
///! If you know what tags you need
///!
///! This file attempts to be the full set of tools needed, so there is no usage
///! Of traits allowing further 'custom' types. It is more Data driven in it's approach
///!
///! Note: All datastructures here are fairly public, allowing them to be manipulated
///! as desired

type Void = ();
type Normal<'a> = Vec<Node<'a>>;
pub type Attributes<'a> = HashMap<Attribute<'a>, Option<attributes::Value<'a>>>;

/// Describes all potential shapes of a html element
/// Note that there are only three kinds, text nodes, comment nodes, and element nodes
/// but an element node can be void, or have children
#[derive(Clone, From,Debug,PartialEq, Eq)]
pub enum Node<'a> {
    Text(Text),
    Comment(Comment),
    Element(Element<'a, Normal<'a>>),
    Void(Element<'a, Void>),
}

/// A Html comment node. <!---- Text --->
#[derive(From, Into, Clone, Debug,PartialEq, Eq)]
pub struct Comment(String);

/// The html element type. This is the most common
/// Note: if children is None, then it is handled as an empty
/// element, this is different than having no children
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Element<'a, T>
where
    T: ElementType,
{
    pub name: Tag<'a>,
    pub attributes: Attributes<'a>,
    pub children: T,
}

/// Represents the different element types.
/// Note: This is sealed so cannot be implemented other this crate
/// Implementations
pub trait ElementType: private::Sealed + Default {}

mod private {
    impl super::ElementType for super::Void {}
    impl<'a> super::ElementType for super::Normal<'a> {}
    pub trait Sealed {}
    impl Sealed for () {}
    impl<'a> Sealed for Vec<super::Node<'a>> {}
}

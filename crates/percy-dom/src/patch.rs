//! Our Patch enum is intentionally kept in it's own file for easy inclusion into
//! The Percy Book.

use crate::{AttributeValue, VText, VirtualNode};
use std::collections::HashMap;

mod apply_patches;
pub use apply_patches::patch;

/// A Patch encodes an operation that modifies a real DOM element.
///
/// To update the real DOM that a user sees you'll want to first diff your
/// old virtual dom and new virtual dom.
///
/// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
/// applied to your real DOM, will make your real DOM look like your new virtual dom.
///
/// Each Patch has a u32 node index that helps us identify the real DOM node that it applies to.
///
/// Our old virtual dom's nodes are indexed depth first, as shown in this illustration
/// (0 being the root node, 1 being it's first child, 2 being it's first child's first child).
///
/// ```text
///             .─.
///            ( 0 )
///             `┬'
///         ┌────┴──────┐
///         │           │
///         ▼           ▼
///        .─.         .─.
///       ( 1 )       ( 4 )
///        `┬'         `─'
///    ┌────┴───┐       │
///    │        │       ├─────┬─────┐
///    ▼        ▼       │     │     │
///   .─.      .─.      ▼     ▼     ▼
///  ( 2 )    ( 3 )    .─.   .─.   .─.
///   `─'      `─'    ( 5 ) ( 6 ) ( 7 )
///                    `─'   `─'   `─'
/// ```
///
/// The patching process is tested in a real browser in crates/percy-dom/tests/diff_patch.rs
#[derive(Debug, PartialEq)]
pub enum Patch<'a> {
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(NodeIdx, Vec<&'a VirtualNode>),
    /// For a `node_i32`, remove all children besides the first `len`
    TruncateChildren(NodeIdx, usize),
    /// Replace a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    Replace(NodeIdx, &'a VirtualNode),
    /// The value attribute of a textarea or input element has not changed, but we will still patch
    /// it anyway in case something was typed into the field.
    ValueAttributeUnchanged(NodeIdx, &'a AttributeValue),
    /// Add attributes that the new node has that the old node does not
    AddAttributes(NodeIdx, HashMap<&'a str, &'a AttributeValue>),
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes(NodeIdx, Vec<&'a str>),
    /// Change the text of a Text node.
    ChangeText(NodeIdx, &'a VText),
    /// Patches that apply to [`SpecialAttributes`].
    SpecialAttribute(PatchSpecialAttribute<'a>),
}

/// Patches that apply to [`SpecialAttributes`].
#[derive(Debug, PartialEq)]
pub enum PatchSpecialAttribute<'a> {
    /// Call the [`SpecialAttributes.on_create_elem`] function on the node.
    CallOnCreateElem(NodeIdx, &'a VirtualNode),
    /// Set the node's innerHTML using the [`SpecialAttributes.dangerous_inner_html`].
    SetDangerousInnerHtml(NodeIdx, &'a VirtualNode),
    /// Set the node's innerHTML to an empty string.
    RemoveDangerousInnerHtml(NodeIdx),
}

// TODO: u32 instead of usize
type NodeIdx = usize;

impl<'a> Patch<'a> {
    /// Every Patch is meant to be applied to a specific node within the DOM. Get the
    /// index of the DOM node that this patch should apply to. DOM nodes are indexed
    /// depth first with the root node in the tree having index 0.
    pub fn node_idx(&self) -> NodeIdx {
        match self {
            Patch::AppendChildren(node_idx, _) => *node_idx,
            Patch::TruncateChildren(node_idx, _) => *node_idx,
            Patch::Replace(node_idx, _) => *node_idx,
            Patch::AddAttributes(node_idx, _) => *node_idx,
            Patch::RemoveAttributes(node_idx, _) => *node_idx,
            Patch::ChangeText(node_idx, _) => *node_idx,
            Patch::ValueAttributeUnchanged(node_idx, _) => *node_idx,
            Patch::SpecialAttribute(special) => match special {
                PatchSpecialAttribute::CallOnCreateElem(node_idx, _) => *node_idx,
                PatchSpecialAttribute::SetDangerousInnerHtml(node_idx, _) => *node_idx,
                PatchSpecialAttribute::RemoveDangerousInnerHtml(node_idx) => *node_idx,
            },
        }
    }
}

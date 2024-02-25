use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::style::graphic::GraphicStateId;

use super::{Attribute, ComputedGraphicItem, GraphicItem, GraphicItemId};

// An item that creates a context around an entry.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct EntryContext {
    pub id: GraphicItemId,
    pub name: String,
    pub selection: Attribute<EntrySelection>,
    pub items: Vec<GraphicItem>,
}

impl EntryContext {
    pub fn new() -> Self {
        Self {
            id: GraphicItemId(Uuid::new_v4()),
            name: String::from("Entry context"),
            selection: EntrySelection::Focus.into(),
            items: Vec::new(),
        }
    }
    pub fn compute_for_state(&self, state: Option<&GraphicStateId>) -> ComputedEntryContext {
        ComputedEntryContext {
            id: self.id,
            selection: self.selection.get_state_or_template(state),
            items: self
                .items
                .iter()
                .map(|item| item.compute_for_state(state))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum EntrySelection {
    First,
    Second,
    Third,
    AheadOfFocus,
    #[default]
    Focus,
    BehindFocus,
}

pub struct ComputedEntryContext {
    pub id: GraphicItemId,
    pub selection: EntrySelection,
    pub items: Vec<ComputedGraphicItem>,
}

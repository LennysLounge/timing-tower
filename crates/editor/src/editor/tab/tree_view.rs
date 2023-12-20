use backend::style::{iterator::NodeIterator, StyleNode};
use bevy_egui::egui::{ScrollArea, Ui};
use uuid::Uuid;

use crate::{
    editor::command::{
        insert_node::InsertNode, move_node::MoveNode, remove_node::RemoveNode, UndoRedoManager,
    },
    style::visitors::{
        drop_allowed,
        tree_view::{self, TreeViewVisitorResult},
    },
};

pub fn tree_view(
    ui: &mut Ui,
    _selected_node: &mut Option<Uuid>,
    base_node: &mut impl StyleNode,
    undo_redo_manager: &mut UndoRedoManager,
) -> bool {
    let mut changed = false;
    let TreeViewVisitorResult {
        response,
        nodes_to_add,
        nodes_to_remove,
    } = ScrollArea::vertical()
        .show(ui, |ui| tree_view::show(ui, base_node))
        .inner;

    // Add nodes
    for (target_node, position, node) in nodes_to_add {
        undo_redo_manager.queue(InsertNode {
            target_node,
            position,
            node,
        });
    }
    // remove nodes
    for id in nodes_to_remove {
        undo_redo_manager.queue(RemoveNode { id });
    }

    if response.selected_node.is_some() {
        *_selected_node = response.selected_node;
    }

    if let Some(drop_action) = &response.drag_drop_action {
        let drop_allowed = base_node
            .as_node()
            .search(&drop_action.drag_id, |dragged| {
                base_node.as_node().search(&drop_action.drop_id, |dropped| {
                    drop_allowed::drop_allowed(dropped, dragged)
                })
            })
            .flatten()
            .unwrap_or(false);

        if !drop_allowed {
            response.remove_drop_marker(ui);
        }

        if response.dropped && drop_allowed {
            undo_redo_manager.queue(MoveNode {
                id: drop_action.drag_id,
                target_id: drop_action.drop_id,
                position: drop_action.position,
            });
            changed = true;
        }
    }
    changed
}

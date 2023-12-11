use backend::style::StyleDefinition;

pub mod assets;
pub mod variables;
pub mod visitors {
    pub mod drop_allowed;
    pub mod insert;
    pub mod property_editor;
    pub mod remove;
    pub mod search;
    pub mod tree_view;
}

pub struct StyleModel {
    pub def: StyleDefinition,
}
impl StyleModel {
    pub fn new(style_def: &StyleDefinition) -> Self {
        Self {
            def: style_def.clone(),
        }
    }
}

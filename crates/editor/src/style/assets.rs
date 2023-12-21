use backend::style::assets::AssetDefinition;

use crate::editor::reference_store::{IntoProducerData, ProducerData};

impl IntoProducerData for AssetDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id.clone(),
            name: self.name.clone(),
            value_type: self.value_type.clone(),
        }
    }
}

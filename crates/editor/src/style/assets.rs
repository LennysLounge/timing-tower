use crate::reference_store::{IntoProducerData, ProducerData};
use backend::style::assets::AssetDefinition;

impl IntoProducerData for AssetDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id.clone(),
            name: self.name.clone(),
            value_type: self.value_type.clone(),
        }
    }
}

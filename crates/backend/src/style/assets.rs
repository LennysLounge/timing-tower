use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, TypedValueProducer},
    value_types::{Texture, ValueType},
};

use super::variables::StaticValueProducer;

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: Uuid,
    pub name: String,
    pub value_type: ValueType,
    pub path: String,
}

impl IntoValueProducer for AssetDefinition {
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer) {
        let typed_value_producer = match self.value_type {
            ValueType::Texture => TypedValueProducer::Texture(Box::new(StaticValueProducer(
                Texture::Handle(self.path.clone()),
            ))),
            _ => unreachable!(),
        };
        (self.id, typed_value_producer)
    }
}

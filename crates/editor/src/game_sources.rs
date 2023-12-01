use std::sync::OnceLock;

use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::{
    reference_store::{ProducerData, IntoProducerData},
    value_store::{IntoValueProducer, TypedValueProducer, ValueProducer, ValueStore},
    value_types::{Boolean, Number, Text, ValueType},
};

static GAME_SOURCES: OnceLock<Vec<GameSource>> = OnceLock::new();

pub fn get_game_sources() -> Vec<&'static GameSource> {
    GAME_SOURCES
        .get_or_init(|| {
            vec![
                GameSource::new_number(
                    uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52"),
                    "Position",
                    |_: &ValueStore, entry: Option<&Entry>| entry.map(|e| *e.position as f32),
                ),
                GameSource::new_number(
                    uuid!("171d7438-3179-4c70-b818-811cf86d514e"),
                    "Car number",
                    |_: &ValueStore, entry: Option<&Entry>| entry.map(|e| *e.car_number as f32),
                ),
                GameSource::new_text(
                    uuid!("8abcf9d5-60f7-4886-a716-139d62ad83ac"),
                    "Driver name",
                    |_: &ValueStore, entry: Option<&Entry>| {
                        entry.and_then(|e| {
                            e.drivers.get(&e.current_driver).map(|driver| {
                                driver
                                    .first_name
                                    .chars()
                                    .into_iter()
                                    .next()
                                    .map(|letter| format!("{}. {}", letter, driver.last_name))
                                    .unwrap_or_else(|| format!(". {}", driver.last_name))
                            })
                        })
                    },
                ),
                GameSource::new_bool(
                    uuid!("de909160-f54b-40cf-a987-6a8453df0914"),
                    "Is focused",
                    |_: &ValueStore, entry: Option<&Entry>| entry.map(|e| e.focused),
                ),
                GameSource::new_bool(
                    uuid!("c16f71b9-dcc9-4f04-9579-ea5211fa99be"),
                    "Is in pits",
                    |_: &ValueStore, entry: Option<&Entry>| entry.map(|e| *e.in_pits),
                ),
                GameSource::new_text(
                    uuid!("4507167c-4c78-4686-b7a2-44809d969cee"),
                    "Car name",
                    |_: &ValueStore, entry: Option<&Entry>| entry.map(|e| e.car.name().to_owned()),
                ),
                GameSource::new_text(
                    uuid!("d1a60628-1ac7-4ad4-a502-95bc649edf07"),
                    "Car manufacturer",
                    |_: &ValueStore, entry: Option<&Entry>| {
                        entry.map(|e| e.car.manufacturer().to_owned())
                    },
                ),
                GameSource::new_number(
                    uuid!("4d519d42-52e9-435c-b614-8d70b42ed3b0"),
                    "ACC: Cup category",
                    |_: &ValueStore, entry: Option<&Entry>| {
                        entry.map(|e| match &e.game_data {
                            unified_sim_model::model::EntryGameData::None => 0 as f32,
                            unified_sim_model::model::EntryGameData::Acc(data) => {
                                data.cup_category as f32
                            }
                        })
                    },
                ),
            ]
        })
        .iter()
        .collect()
}

#[derive(Clone)]
enum Extractor {
    Number(fn(&ValueStore, Option<&Entry>) -> Option<f32>),
    Text(fn(&ValueStore, Option<&Entry>) -> Option<String>),
    Boolean(fn(&ValueStore, Option<&Entry>) -> Option<bool>),
}

impl ValueProducer<Number> for Extractor {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        if let Extractor::Number(f) = self {
            (f)(value_store, entry).map(|n| Number(n))
        } else {
            None
        }
    }
}
impl ValueProducer<Text> for Extractor {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        if let Extractor::Text(f) = self {
            (f)(value_store, entry).map(|t| Text(t))
        } else {
            None
        }
    }
}
impl ValueProducer<Boolean> for Extractor {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        if let Extractor::Boolean(f) = self {
            (f)(value_store, entry).map(|b| Boolean(b))
        } else {
            None
        }
    }
}

pub struct GameSource {
    asset_id: ProducerData,
    extractor: Extractor,
}
impl IntoProducerData for GameSource {
    fn producer_data(&self) -> &ProducerData {
        &self.asset_id
    }
}
impl IntoValueProducer for GameSource {
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer) {
        let producer = match self.extractor {
            Extractor::Number(_) => TypedValueProducer::Number(Box::new(self.extractor.clone())),
            Extractor::Text(_) => TypedValueProducer::Text(Box::new(self.extractor.clone())),
            Extractor::Boolean(_) => TypedValueProducer::Boolean(Box::new(self.extractor.clone())),
        };
        (self.asset_id.id, producer)
    }
}

impl GameSource {
    fn new_number(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<f32>,
    ) -> Self {
        Self {
            asset_id: ProducerData {
                id,
                name: name.to_string(),
                asset_type: ValueType::Number,
            },
            extractor: Extractor::Number(extractor),
        }
    }
    fn new_text(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<String>,
    ) -> Self {
        Self {
            asset_id: ProducerData {
                id,
                name: name.to_string(),
                asset_type: ValueType::Text,
            },
            extractor: Extractor::Text(extractor),
        }
    }
    fn new_bool(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<bool>,
    ) -> Self {
        Self {
            asset_id: ProducerData {
                id,
                name: name.to_string(),
                asset_type: ValueType::Boolean,
            },
            extractor: Extractor::Boolean(extractor),
        }
    }
}

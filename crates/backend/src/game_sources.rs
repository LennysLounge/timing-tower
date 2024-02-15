use std::sync::OnceLock;

use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::{
    value_store::{AnyValueProducer, ProducerId, ValueProducer, ValueStore},
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
impl ValueProducer for fn(&ValueStore, Option<&Entry>) -> Option<f32> {
    type Output = Number;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        (self)(value_store, entry).map(|f| Number(f))
    }
}
impl ValueProducer for fn(&ValueStore, Option<&Entry>) -> Option<String> {
    type Output = Text;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        (self)(value_store, entry).map(|f| Text(f))
    }
}
impl ValueProducer for fn(&ValueStore, Option<&Entry>) -> Option<bool> {
    type Output = Boolean;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        (self)(value_store, entry).map(|f| Boolean(f))
    }
}

pub struct GameSource {
    pub id: Uuid,
    pub name: String,
    pub value_type: ValueType,
    extractor: Extractor,
}
impl GameSource {
    pub fn value_producer(&self) -> AnyValueProducer {
        match self.extractor.clone() {
            Extractor::Number(ex) => ex.into(),
            Extractor::Text(ex) => ex.into(),
            Extractor::Boolean(ex) => ex.into(),
        }
    }
    pub fn value_id(&self) -> ProducerId {
        ProducerId(self.id)
    }
}

impl GameSource {
    fn new_number(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<f32>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            value_type: ValueType::Number,
            extractor: Extractor::Number(extractor),
        }
    }
    fn new_text(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            value_type: ValueType::Text,
            extractor: Extractor::Text(extractor),
        }
    }
    fn new_bool(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<bool>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            value_type: ValueType::Boolean,
            extractor: Extractor::Boolean(extractor),
        }
    }
}

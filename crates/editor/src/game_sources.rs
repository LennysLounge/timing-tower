use std::sync::OnceLock;

use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::value_store::{
    AssetId, ValueStore, AssetSource, AssetType, BooleanSource, IntoAssetSource, NumberSource,
    TextSource,
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

enum Extractor {
    Number(fn(&ValueStore, Option<&Entry>) -> Option<f32>),
    Text(fn(&ValueStore, Option<&Entry>) -> Option<String>),
    Boolean(fn(&ValueStore, Option<&Entry>) -> Option<bool>),
}

pub struct GameSource {
    asset_id: AssetId,
    extractor: Extractor,
}
impl IntoAssetSource for GameSource {
    fn get_asset_source(&self) -> AssetSource {
        match &self.extractor {
            Extractor::Number(f) => AssetSource::Number(Box::new(f.clone())),
            Extractor::Text(f) => AssetSource::Text(Box::new(f.clone())),
            Extractor::Boolean(f) => AssetSource::Boolean(Box::new(f.clone())),
        }
    }

    fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }
}

impl GameSource {
    fn new_number(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, Option<&Entry>) -> Option<f32>,
    ) -> Self {
        Self {
            asset_id: AssetId {
                id,
                name: name.to_string(),
                asset_type: AssetType::Number,
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
            asset_id: AssetId {
                id,
                name: name.to_string(),
                asset_type: AssetType::Text,
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
            asset_id: AssetId {
                id,
                name: name.to_string(),
                asset_type: AssetType::Boolean,
            },
            extractor: Extractor::Boolean(extractor),
        }
    }
}

impl<F> NumberSource for F
where
    F: Fn(&ValueStore, Option<&Entry>) -> Option<f32>,
{
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<f32> {
        (self)(vars, entry)
    }
}

impl<F> TextSource for F
where
    F: Fn(&ValueStore, Option<&Entry>) -> Option<String>,
{
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<String> {
        (self)(vars, entry)
    }
}

impl<F> BooleanSource for F
where
    F: Fn(&ValueStore, Option<&Entry>) -> Option<bool>,
{
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        (self)(vars, entry)
    }
}

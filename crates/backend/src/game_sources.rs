use std::sync::OnceLock;

use unified_sim_model::model::{Day, SessionPhase, SessionType};
use uuid::{uuid, Uuid};

use crate::{
    value_store::{AnyValueProducer, ModelContext, ProducerId, ValueProducer, ValueStore},
    value_types::{AnyProducerRef, Boolean, Number, Text, ValueType},
};

static GAME_SOURCES: OnceLock<Vec<GameSource>> = OnceLock::new();

pub fn get_game_sources() -> Vec<&'static GameSource> {
    GAME_SOURCES
        .get_or_init(|| {
            vec![
                GameSource::new_text(
                    uuid!("061da576-2c4f-46d4-ab90-9fdb8af761dd"),
                    "Session type",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.session_type {
                            SessionType::Practice => String::from("Practice"),
                            SessionType::Qualifying => String::from("Qualifying"),
                            SessionType::Race => String::from("Race"),
                            SessionType::None => String::from("None"),
                        })
                    },
                ),
                GameSource::new_number(
                    uuid!("e3b19274-200e-4ebb-bbdd-a75e2390094c"),
                    "Session type nr",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.session_type {
                            SessionType::Practice => 1.0,
                            SessionType::Qualifying => 2.0,
                            SessionType::Race => 3.0,
                            SessionType::None => 0.0,
                        })
                    },
                ),
                GameSource::new_text(
                    uuid!("b81be9f3-347d-4dae-a826-92d06840bcfb"),
                    "Session phase",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.phase {
                            SessionPhase::None => String::from("None"),
                            SessionPhase::Waiting => String::from("Waiting"),
                            SessionPhase::Preparing => String::from("Preparing"),
                            SessionPhase::Formation => String::from("Formation"),
                            SessionPhase::Active => String::from("Active"),
                            SessionPhase::Ending => String::from("Ending"),
                            SessionPhase::Finished => String::from("Finished"),
                        })
                    },
                ),
                GameSource::new_number(
                    uuid!("b0c6555c-b97d-45c4-be5d-0de1ba4fdd47"),
                    "Session phase nr",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.phase {
                            SessionPhase::None => 0.0,
                            SessionPhase::Waiting => 1.0,
                            SessionPhase::Preparing => 2.0,
                            SessionPhase::Formation => 3.0,
                            SessionPhase::Active => 4.0,
                            SessionPhase::Ending => 5.0,
                            SessionPhase::Finished => 6.0,
                        })
                    },
                ),
                GameSource::new_text(
                    uuid!("6acb7506-7752-4f9a-9af0-8a72061eee4f"),
                    "Session time",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| session.session_time.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("6acb7506-7752-4f9a-9af0-8a72061eee4f"),
                    "Session time sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| session.session_time.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("c396553c-3281-4735-9805-dfd2de6ae3eb"),
                    "Session time remaining",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| session.time_remaining.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("37fee0b8-6ecb-4cfd-80a8-593fd5d1c357"),
                    "Session time remaining sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| session.time_remaining.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_number(
                    uuid!("d633e60b-ff0b-4eaf-8931-21da1bc0969d"),
                    "Session laps",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| *session.laps as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("8ac057df-8f8b-4c9b-a3ef-ae9a73753d90"),
                    "Session laps remaining",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| *session.laps_remaining as f32)
                    },
                ),
                GameSource::new_text(
                    uuid!("c13448a6-2a06-4531-8707-ddd5c5ab1b30"),
                    "Session time of day",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| session.time_of_day.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("f61c88d7-5eb7-4a07-a2a5-80cfdcad78e7"),
                    "Session time of day sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| session.time_of_day.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("cef1824f-6979-4275-9bc4-7895cbc53278"),
                    "Session day",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.day {
                            Day::Monday => String::from("Monday"),
                            Day::Thuesday => String::from("Thuesday"),
                            Day::Wednesday => String::from("Wednesday"),
                            Day::Thrusday => String::from("Thrusday"),
                            Day::Friday => String::from("Friday"),
                            Day::Saturday => String::from("Saturday"),
                            Day::Sunday => String::from("Sunday"),
                        })
                    },
                ),
                GameSource::new_number(
                    uuid!("14d6eef5-8a5b-4ce3-bf37-16f163b1490e"),
                    "Session day nr",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| match *session.day {
                            Day::Monday => 0.0,
                            Day::Thuesday => 1.0,
                            Day::Wednesday => 2.0,
                            Day::Thrusday => 3.0,
                            Day::Friday => 4.0,
                            Day::Saturday => 5.0,
                            Day::Sunday => 6.0,
                        })
                    },
                ),
                GameSource::new_number(
                    uuid!("a8a027a1-2809-4efd-b6cf-aed0d711b000"),
                    "Ambient temp",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| session.ambient_temp.c)
                    },
                ),
                GameSource::new_number(
                    uuid!("a116074b-e02b-4ea6-81e9-a56e2ceb5760"),
                    "Track temp",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| session.track_temp.c)
                    },
                ),
                GameSource::new_text(
                    uuid!("bc83008a-8378-4648-9715-14b818ae9b86"),
                    "Session best lap",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| {
                            session
                                .best_lap
                                .as_ref()
                                .as_ref()
                                .map(|lap| lap.time.format())
                                .unwrap_or(String::from("-"))
                        })
                    },
                ),
                GameSource::new_text(
                    uuid!("f14a59d1-039a-489d-b727-775d830ee4e4"),
                    "Track name",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| (*session.track_name).clone())
                    },
                ),
                GameSource::new_number(
                    uuid!("b1243a33-e210-4290-b255-36ead3b0ea44"),
                    "Track length km",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .map(|session| session.track_length.meter / 1000.0)
                    },
                ),
                GameSource::new_number(
                    uuid!("bc83008a-8378-4648-9715-14b818ae9b86"),
                    "Track length meter",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.session.map(|session| session.track_length.meter)
                    },
                ),
                //
                //   Driver
                //
                GameSource::new_number(
                    uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52"),
                    "Position",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.position as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("171d7438-3179-4c70-b818-811cf86d514e"),
                    "Car number",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.car_number as f32)
                    },
                ),
                GameSource::new_text(
                    uuid!("8abcf9d5-60f7-4886-a716-139d62ad83ac"),
                    "Driver name",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.and_then(|e| {
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
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| e.focused),
                ),
                GameSource::new_bool(
                    uuid!("c16f71b9-dcc9-4f04-9579-ea5211fa99be"),
                    "Is in pits",
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| *e.in_pits),
                ),
                GameSource::new_text(
                    uuid!("4507167c-4c78-4686-b7a2-44809d969cee"),
                    "Car name",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.car.name().to_owned())
                    },
                ),
                GameSource::new_text(
                    uuid!("d1a60628-1ac7-4ad4-a502-95bc649edf07"),
                    "Car manufacturer",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.car.manufacturer().to_owned())
                    },
                ),
                GameSource::new_number(
                    uuid!("4d519d42-52e9-435c-b614-8d70b42ed3b0"),
                    "ACC: Cup category",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| match &e.game_data {
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
    Number(fn(&ValueStore, ModelContext<'_>) -> Option<f32>),
    Text(fn(&ValueStore, ModelContext<'_>) -> Option<String>),
    Boolean(fn(&ValueStore, ModelContext<'_>) -> Option<bool>),
}
impl ValueProducer for fn(&ValueStore, ModelContext<'_>) -> Option<f32> {
    type Output = Number;
    fn get(&self, value_store: &ValueStore, context: ModelContext<'_>) -> Option<Number> {
        (self)(value_store, context).map(|f| Number(f))
    }
}
impl ValueProducer for fn(&ValueStore, ModelContext<'_>) -> Option<String> {
    type Output = Text;
    fn get(&self, value_store: &ValueStore, context: ModelContext<'_>) -> Option<Text> {
        (self)(value_store, context).map(|f| Text(f))
    }
}
impl ValueProducer for fn(&ValueStore, context: ModelContext<'_>) -> Option<bool> {
    type Output = Boolean;
    fn get(&self, value_store: &ValueStore, context: ModelContext<'_>) -> Option<Boolean> {
        (self)(value_store, context).map(|f| Boolean(f))
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
    pub fn producer_ref(&self) -> AnyProducerRef {
        AnyProducerRef::new(ProducerId(self.id), self.value_type)
    }
}

impl GameSource {
    fn new_number(
        id: Uuid,
        name: &str,
        extractor: fn(&ValueStore, ModelContext<'_>) -> Option<f32>,
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
        extractor: fn(&ValueStore, ModelContext<'_>) -> Option<String>,
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
        extractor: fn(&ValueStore, ModelContext<'_>) -> Option<bool>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            value_type: ValueType::Boolean,
            extractor: Extractor::Boolean(extractor),
        }
    }
}

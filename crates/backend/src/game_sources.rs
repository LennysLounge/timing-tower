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
                        context
                            .session
                            .map(|session| session.session_time.fmt_no_ms())
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
                            .map(|session| session.time_remaining.fmt_no_ms())
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
                        context
                            .session
                            .map(|session| session.time_of_day.fmt_no_s_ms())
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
                GameSource::new_text(
                    uuid!("fcfa2406-6088-47f4-b5f3-db75488e896d"),
                    "Car category",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.car.category().name.to_owned())
                    },
                ),
                GameSource::new_number(
                    uuid!("171d7438-3179-4c70-b818-811cf86d514e"),
                    "Car number",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.car_number as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52"),
                    "Position",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.position as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("6cd92029-6f37-4c19-b392-9c8f723a1ac5"),
                    "Spline position",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.spline_pos as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("30cd7503-df5f-4fcf-9175-e7b6b469b189"),
                    "Lap count",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.lap_count as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("1045e491-f5c5-4551-9239-7a79f6a3a8c6"),
                    "Current lap sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.current_lap.time.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("b835a422-d98e-4e84-9942-8abf478f0c49"),
                    "Current lap",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.current_lap.time.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("9b42568b-6fbd-4ae3-9290-5151e6336ad4"),
                    "Best lap sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .entry
                            .and_then(|e| e.best_lap.as_ref().as_ref())
                            .map(|best_lap| best_lap.time.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("db85f061-0d27-4d8c-9acc-44de629760bc"),
                    "Best lap",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .entry
                            .and_then(|e| e.best_lap.as_ref().as_ref())
                            .map(|best_lap| best_lap.time.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("0de05742-c101-41d1-88db-57ca293a0ab9"),
                    "Performance delta sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .entry
                            .map(|e| e.performance_delta.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("6d1e55e3-11b4-4792-94c9-facdd7463d7c"),
                    "Best lap",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.performance_delta.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("29081b33-0525-4d4c-b077-87c90680b45f"),
                    "Time behind leader sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .entry
                            .map(|e| e.time_behind_leader.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("bf09365b-0f51-4482-b3c3-1c2c6e273bfc"),
                    "Time behind leader",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.time_behind_leader.format())
                    },
                ),
                GameSource::new_number(
                    uuid!("9f0b3c0c-0379-4808-a99d-d3a416cc8f1c"),
                    "Time behind position ahead sec",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .entry
                            .map(|e| e.time_behind_position_ahead.ms as f32 / 1000.0)
                    },
                ),
                GameSource::new_text(
                    uuid!("dba48b29-6fb7-4a24-98e1-9066c7cde3b6"),
                    "Time behind position ahead",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| e.time_behind_position_ahead.format())
                    },
                ),
                GameSource::new_bool(
                    uuid!("c16f71b9-dcc9-4f04-9579-ea5211fa99be"),
                    "Is in pits",
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| *e.in_pits),
                ),
                GameSource::new_number(
                    uuid!("8cc69329-3dbd-4283-8c9f-38ce4cceef46"),
                    "Gear",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.gear as f32)
                    },
                ),
                GameSource::new_number(
                    uuid!("f5cfbdd0-bc01-4c6c-9d80-5b8b0db8cf8e"),
                    "Speed m/s",
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| *e.speed),
                ),
                GameSource::new_number(
                    uuid!("095c3af8-3971-4bc9-982b-cc1dd7a3166f"),
                    "Speed km/h",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.speed * 3.6)
                    },
                ),
                GameSource::new_number(
                    uuid!("32145d78-3b0a-4108-ba9d-505f54a8e7bc"),
                    "Speed mp/h",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context.entry.map(|e| *e.speed * 2.23694)
                    },
                ),
                GameSource::new_bool(
                    uuid!("f4c61514-a6f3-4f11-a70e-9743b387cb8e"),
                    "Connected",
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| *e.connected),
                ),
                GameSource::new_bool(
                    uuid!("de909160-f54b-40cf-a987-6a8453df0914"),
                    "Is focused",
                    |_: &ValueStore, context: ModelContext<'_>| context.entry.map(|e| e.focused),
                ),
                GameSource::new_bool(
                    uuid!("ea52ea28-01c3-4a1b-9a9e-40919ea48f2d"),
                    "has session best lap",
                    |_: &ValueStore, context: ModelContext<'_>| {
                        context
                            .session
                            .and_then(|session| session.best_lap.as_ref().as_ref())
                            .and_then(|best_lap| best_lap.entry_id)
                            .map(|best_lap_entry_id| {
                                context
                                    .entry
                                    .is_some_and(|entry| entry.id == best_lap_entry_id)
                            })
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

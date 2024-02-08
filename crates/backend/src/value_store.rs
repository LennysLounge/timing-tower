use std::collections::HashMap;

use crate::{
    game_sources,
    savefile::{Savefile, SavefileChanged},
    value_types::{Boolean, Font, Number, Property, Text, Texture, Tint, ValueRef},
};
use bevy::{
    app::{First, Plugin},
    ecs::{
        event::EventReader,
        system::{Res, ResMut},
    },
    prelude::Resource,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use unified_sim_model::model::Entry;
use uuid::Uuid;

use self::private::PrivateValueResolver;

pub struct ValueStorePlugin;
impl Plugin for ValueStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ValueStore>()
            .add_systems(First, savefile_changed);
    }
}

/// Identifies a value producer.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ValueId(pub Uuid);

/// Implementors of this trait can produce a value for a [`ValueStore`].
pub trait ValueProducer {
    #[allow(unused)]
    fn get_number(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        None
    }
    #[allow(unused)]
    fn get_text(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        None
    }
    #[allow(unused)]
    fn get_boolean(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        None
    }
    #[allow(unused)]
    fn get_tint(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        None
    }
    #[allow(unused)]
    fn get_texture(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        None
    }
    #[allow(unused)]
    fn get_font(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Font> {
        None
    }
}

/// The value store that holds all [`ValueProducer`]s and can resolve
/// value requests.
#[derive(Resource, Default)]
pub struct ValueStore {
    values: HashMap<ValueId, Box<dyn ValueProducer + Sync + Send>>,
}
impl ValueStore {
    pub fn get<T>(&self, value_ref: &ValueRef<T>, entry: Option<&Entry>) -> Option<T>
    where
        Self: ValueResolver<T>,
    {
        self.values
            .get(&value_ref.id)
            .and_then(|p| self.get_typed(p, entry))
    }

    pub fn get_property<T>(&self, property: &Property<T>, entry: Option<&Entry>) -> Option<T>
    where
        Self: ValueResolver<T>,
        T: Clone,
    {
        match property {
            Property::Fixed(v) => Some(v.clone()),
            Property::ValueRef(value_ref) => self.get(value_ref, entry),
        }
    }
}

/// This trait signals that the [`ValueStore`] can resolve values of type T.
///
/// It is necessary so that [`ValueStore`] can implement different behavior for
/// different types `<T>`. Since rust lacks specialization, it is necessairy
/// for [`ValueStore`] to implement this trait for all type it would like to be able to resolve.
///
/// To avoid leaking a private trait in the public api the [`ValueResolver`] is only
/// a marker trait and requires the private inner trait to be implemented.
pub trait ValueResolver<T>: PrivateValueResolver<T> {}
impl<T> ValueResolver<T> for ValueStore where ValueStore: PrivateValueResolver<T> {}
mod private {
    use unified_sim_model::model::Entry;

    use crate::value_types::{Boolean, Font, Number, Text, Texture, Tint};

    use super::{ValueProducer, ValueStore};

    pub trait PrivateValueResolver<T> {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<T>;
    }
    impl PrivateValueResolver<Number> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Number> {
            producer.get_number(self, entry)
        }
    }
    impl PrivateValueResolver<Text> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Text> {
            producer
                .get_text(self, entry)
                .or(producer
                    .get_number(self, entry)
                    .map(|number| Text(format!("{}", number.0))))
                .or(producer.get_boolean(self, entry).map(|bool| {
                    if bool.0 {
                        Text(String::from("Yes"))
                    } else {
                        Text(String::from("No"))
                    }
                }))
        }
    }
    impl PrivateValueResolver<Tint> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Tint> {
            producer.get_tint(self, entry)
        }
    }
    impl PrivateValueResolver<Boolean> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Boolean> {
            producer.get_boolean(self, entry)
        }
    }
    impl PrivateValueResolver<Texture> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Texture> {
            producer.get_texture(self, entry)
        }
    }
    impl PrivateValueResolver<Font> for ValueStore {
        fn get_typed(
            &self,
            producer: &Box<dyn ValueProducer + Sync + Send>,
            entry: Option<&Entry>,
        ) -> Option<Font> {
            producer.get_font(self, entry)
        }
    }
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut value_store: ResMut<ValueStore>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    info!("Reload value store");
    value_store.values.clear();
    for var in savefile.style().vars.contained_variables() {
        value_store
            .values
            .insert(var.value_id(), var.value_producer());
    }
    for asset in savefile.style().assets.contained_assets() {
        value_store
            .values
            .insert(asset.value_id(), asset.value_producer());
    }
    for game in game_sources::get_game_sources() {
        value_store
            .values
            .insert(game.value_id(), game.value_producer());
    }
}

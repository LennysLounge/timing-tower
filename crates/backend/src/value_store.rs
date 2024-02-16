use std::{any::Any, collections::HashMap};

use crate::{
    game_sources,
    savefile::{Savefile, SavefileChanged},
    value_types::{ProducerRef, Property},
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
pub struct ProducerId(pub Uuid);

/// This trait must be implemeneted for something to produce a value in the value store.
/// A value producer may only produce one type of value. If necessary and if possible that
/// value is then transformed into the needed value.
/// To be able to produce more than one value a second instance of the producer and
/// value producer must be created.
pub trait ValueProducer {
    type Output;

    /// Get the produced value.
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Self::Output>;
}

/// Any kind of value producer.  
///
/// Erases the associated type of a value producer.
pub struct AnyValueProducer {
    inner: Box<dyn Any + Sync + Send>,
}
impl AnyValueProducer {
    /// Get the produced value of this producer.
    ///
    /// Forwards the call directly to the actual producer. If the actual producer is of
    /// a different type than the requested type, `None` is returned.
    fn get<T: 'static>(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T> {
        self.inner
            .downcast_ref::<Box<dyn ValueProducer<Output = T> + Sync + Send>>()
            .and_then(|producer| producer.as_ref().get(value_store, entry))
    }
}
impl<U, T> From<T> for AnyValueProducer
where
    U: 'static,
    T: ValueProducer<Output = U> + Sync + Send + 'static,
{
    fn from(value: T) -> Self {
        let as_trait_obj: Box<dyn ValueProducer<Output = U> + Sync + Send> = Box::new(value);
        Self {
            inner: Box::new(as_trait_obj),
        }
    }
}

/// The value store that holds all [`ValueProducer`]s and can resolve
/// value requests.
#[derive(Resource, Default)]
pub struct ValueStore {
    values: HashMap<ProducerId, AnyValueProducer>,
}
impl ValueStore {
    pub fn get<T>(&self, value_ref: &ProducerRef<T>, entry: Option<&Entry>) -> Option<T>
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
            Property::Producer(producer_id) => self
                .values
                .get(&producer_id)
                .and_then(|p| self.get_typed(p, entry)),
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

    use super::{AnyValueProducer, ValueStore};

    pub trait PrivateValueResolver<T> {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<T>;
    }
    impl PrivateValueResolver<Number> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Number> {
            producer.get(self, entry)
        }
    }
    impl PrivateValueResolver<Text> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Text> {
            producer
                .get(self, entry)
                .or(producer
                    .get::<Number>(self, entry)
                    .map(|number| Text(format!("{}", number.0))))
                .or(producer.get::<Boolean>(self, entry).map(|bool| {
                    if bool.0 {
                        Text(String::from("Yes"))
                    } else {
                        Text(String::from("No"))
                    }
                }))
        }
    }
    impl PrivateValueResolver<Tint> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Tint> {
            producer.get(self, entry)
        }
    }
    impl PrivateValueResolver<Boolean> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Boolean> {
            producer.get(self, entry)
        }
    }
    impl PrivateValueResolver<Texture> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Texture> {
            producer.get(self, entry)
        }
    }
    impl PrivateValueResolver<Font> for ValueStore {
        fn get_typed(&self, producer: &AnyValueProducer, entry: Option<&Entry>) -> Option<Font> {
            producer.get(self, entry)
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

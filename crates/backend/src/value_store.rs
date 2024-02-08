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
use tracing::info;
use unified_sim_model::model::Entry;
use uuid::Uuid;

use self::private::Unimplementable;

pub struct ValueStorePlugin;
impl Plugin for ValueStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ValueStore>()
            .add_systems(First, savefile_changed);
    }
}

/// Implementors of this trait can produce a value for a [`ValueStore`].
pub trait ValueProducer<T> {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T>;
}

pub trait IntoValueProducer {
    fn get_value_producer(&self) -> (Uuid, UntypedValueProducer);
}

pub enum UntypedValueProducer {
    Number(Box<dyn ValueProducer<Number> + Send + Sync>),
    Text(Box<dyn ValueProducer<Text> + Send + Sync>),
    Tint(Box<dyn ValueProducer<Tint> + Send + Sync>),
    Boolean(Box<dyn ValueProducer<Boolean> + Send + Sync>),
    Texture(Box<dyn ValueProducer<Texture> + Send + Sync>),
    Font(Box<dyn ValueProducer<Font> + Send + Sync>),
}

#[derive(Resource, Default)]
pub struct ValueStore {
    pub assets: HashMap<Uuid, UntypedValueProducer>,
}

impl ValueStore {
    pub fn reload_repo(
        &mut self,
        vars: Vec<&impl IntoValueProducer>,
        assets: Vec<&impl IntoValueProducer>,
    ) {
        self.assets.clear();
        self.convert(vars);
        self.convert(assets);
        self.convert(game_sources::get_game_sources());
    }

    fn convert(&mut self, asset_defs: Vec<&impl IntoValueProducer>) {
        for var_def in asset_defs {
            let (id, value_producer) = var_def.get_value_producer();
            self.assets.insert(id, value_producer);
        }
    }

    pub fn get<T>(&self, value_ref: &ValueRef<T>, entry: Option<&Entry>) -> Option<T>
    where
        Self: ValueResolver<T>,
    {
        self.assets
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

/// This trait is implemented by the [`ValueStore`] and is necessary for it to
/// resolve values of type <T>.
/// Since rust lacks specialization, it is necessairy for [`ValueStore`] to implement this
/// trait for all type it would like to be able to resolve.
///
/// It is public so that others can require [`ValueStore`] to implement it aswell in a generic context.
/// However it should not be implemented by others and therefore requires the Unimplementable trait.
pub trait ValueResolver<T>: Unimplementable {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<T>;
}
mod private {
    pub trait Unimplementable {}
    impl Unimplementable for super::ValueStore {}
}
impl ValueResolver<Number> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Number> {
        match producer {
            UntypedValueProducer::Number(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl ValueResolver<Text> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Text> {
        match producer {
            UntypedValueProducer::Number(p) => p.get(self, entry).map(|n| Text(format!("{}", n.0))),
            UntypedValueProducer::Boolean(p) => p.get(self, entry).map(|b| {
                if b.0 {
                    Text(String::from("Yes"))
                } else {
                    Text(String::from("No"))
                }
            }),
            UntypedValueProducer::Text(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl ValueResolver<Tint> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Tint> {
        match producer {
            UntypedValueProducer::Tint(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl ValueResolver<Boolean> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Boolean> {
        match producer {
            UntypedValueProducer::Boolean(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl ValueResolver<Texture> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Texture> {
        match producer {
            UntypedValueProducer::Texture(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl ValueResolver<Font> for ValueStore {
    fn get_typed(&self, producer: &UntypedValueProducer, entry: Option<&Entry>) -> Option<Font> {
        match producer {
            UntypedValueProducer::Font(p) => p.get(self, entry),
            _ => None,
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
    value_store.reload_repo(
        savefile.style().vars.contained_variables(),
        savefile.style().assets.contained_assets(),
    );
}

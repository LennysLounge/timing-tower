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

use self::private::PrivateValueResolver;

pub struct ValueStorePlugin;
impl Plugin for ValueStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ValueStore>()
            .add_systems(First, savefile_changed);
    }
}

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

pub trait IntoValueProducer {
    fn get_value_producer(&self) -> (Uuid, Box<dyn ValueProducer + Sync + Send>);
}

#[derive(Resource, Default)]
pub struct ValueStore {
    assets: HashMap<Uuid, Box<dyn ValueProducer + Sync + Send>>,
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
    value_store.reload_repo(
        savefile.style().vars.contained_variables(),
        savefile.style().assets.contained_assets(),
    );
}

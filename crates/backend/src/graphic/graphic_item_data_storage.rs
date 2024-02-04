use std::{
    any::{Any, TypeId},
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

/// A entry in the graphic item data storage.
///
/// Holds the stored data and a flag if it has been used.
struct StorageEntry {
    used: bool,
    data: Box<dyn Any + Send + Sync>,
}

/// Holds graphic item data and meta data about the usage of that data
/// in a type + key map.
/// If some graphic item data has not been used, it can be removed
/// from storage.
#[derive(Default)]
pub struct GraphicItemDataStorage {
    data: HashMap<u64, StorageEntry>,
}
impl GraphicItemDataStorage {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Returns a reference to data of type `T` associated with the key `key`.
    /// If this entry does not exists in storage, a default value of it is created.
    ///
    /// See [`Self::get_or_create()`].
    #[allow(unused)]
    pub fn get_or_default<T>(&mut self, key: impl Hash) -> &mut T
    where
        T: Default + Send + Sync + 'static,
    {
        self.get_or_create(key, || T::default())
    }

    /// Returns a reference to data of type `T` associated with the key `key`.
    /// If this entry does not exists in storage, it is created with the create closure.
    ///
    /// The returned data is dependant on both the key used aswell as the type that was requested.
    /// A single key can produce different data depending on the requested type.
    /// A single type can produce different data depending on the key used.
    ///
    /// Accessing an entry this way marks the entry as `used` which will preven it from being
    /// removed from storage.
    pub fn get_or_create<T>(&mut self, key: impl Hash, mut create: impl FnMut() -> T) -> &mut T
    where
        T: Send + Sync + 'static,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        TypeId::of::<T>().hash(&mut hasher);
        let real_key = hasher.finish();

        let entry = self.data.entry(real_key).or_insert_with(|| StorageEntry {
            used: false,
            data: Box::new(create()),
        });
        entry.used = true;
        entry
            .data
            .downcast_mut::<T>()
            .expect("The type of entry has to be T")
    }

    /// Remove all stale entries from storage.
    ///
    /// An entry is stale if it has not been accessed since the last time
    /// the storage was cleared.
    pub fn clear_stale_data(&mut self) {
        self.data.retain(|_, entry| entry.used);
        self.data.values_mut().for_each(|entry| entry.used = false);
    }

    /// Make a new storage context.
    pub fn make_context<'a>(&'a mut self, context: impl Hash) -> GraphicItemDataStorageContext<'a> {
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        GraphicItemDataStorageContext {
            storage: self,
            context: hasher.finish(),
        }
    }
}

/// A [`GraphicItemDataStorage`] that has some context attached to it.  
/// The context is added to every call to get.
pub struct GraphicItemDataStorageContext<'a> {
    storage: &'a mut GraphicItemDataStorage,
    context: u64,
}
impl<'a> GraphicItemDataStorageContext<'a> {
    #[allow(unused)]
    pub fn get_or_default<T>(&mut self, key: impl Hash) -> &mut T
    where
        T: Default + Send + Sync + 'static,
    {
        self.storage
            .get_or_create((self.context, key), || T::default())
    }

    pub fn get_or_create<T>(&mut self, key: impl Hash, create: impl FnMut() -> T) -> &mut T
    where
        T: Send + Sync + 'static,
    {
        self.storage.get_or_create((self.context, key), create)
    }

    pub fn make_context(&mut self, context: impl Hash) -> GraphicItemDataStorageContext<'_> {
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        self.context.hash(&mut hasher);
        GraphicItemDataStorageContext {
            storage: self.storage,
            context: hasher.finish(),
        }
    }
}

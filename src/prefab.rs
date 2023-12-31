use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{self, Read},
    marker::PhantomData,
    path::Path,
};

use bevy::{
    ecs::{
        component::Component,
        system::Command,
        world::{EntityWorldMut, World},
    },
    hierarchy::BuildWorldChildren,
    log::{error, warn},
    prelude::{App, Bundle, Commands, Entity, Event, Plugin, Resource},
};

pub struct Prefab<T> {
    entidades: Vec<PrefabEntityBuilder<T>>,
}
impl<T> Prefab<T> {
    pub fn new() -> Self {
        Self {
            entidades: vec![PrefabEntityBuilder::default()],
        }
    }

    pub fn from_data(data: Option<T>) -> Self {
        Self {
            entidades: vec![PrefabEntityBuilder::new(None, data)],
        }
    }

    pub fn add(&mut self, parent: Option<usize>, data: Option<T>) -> usize {
        let index = self.entidades.len();
        self.entidades.push(PrefabEntityBuilder::new(parent, data));
        index
    }

    /// Return a Hash for accessing the entities with parent relation
    pub fn all_parents_childs(&self) -> HashMap<usize, Vec<usize>> {
        let mut hash: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index, entidade) in self.entidades.iter().enumerate() {
            if let Some(parent) = entidade.parent {
                if let Some(vec) = hash.get_mut(&parent) {
                    vec.push(index);
                } else {
                    hash.insert(parent, vec![index]);
                }
            }
            hash.insert(index, vec![]);
        }
        hash
    }

    pub fn get_entity(&self, index: usize) -> Option<&PrefabEntityBuilder<T>> {
        self.entidades.get(index)
    }

    pub fn get_entity_mut(&mut self, index: usize) -> Option<&mut PrefabEntityBuilder<T>> {
        self.entidades.get_mut(index)
    }

    /// Function must be used only after prepared
    pub fn spawn(
        &mut self,
        processed_children: &HashMap<usize, Vec<usize>>,
        init: &usize,
        world: &mut World,
    ) -> Option<Entity>
    where
        T: PrefabData,
    {
        let mut root_entity = world.spawn_empty();
        if let Some(c) = processed_children.get(init) {
            if let Some(parent) = self.entidades.get_mut(*init) {
                if let Some(data) = parent.take_data() {
                    data.insert_into_entity(&mut root_entity);
                }
                root_entity.insert(PrefabEntity::<T>(PhantomData::default()));
            }
            for child in c {
                // Safety: Its safe because the location is updated after the operation.
                unsafe {
                    let child = self.spawn(processed_children, child, root_entity.world_mut());
                    root_entity.update_location();
                    if let Some(child) = child {
                        root_entity.add_child(child);
                    }
                }
            }
        } else {
            // Probably an error with wrong adding the children.
            warn!(target: "prefab", "[warning] Entity not found in the prefab with index {}", init);
            return None;
        }
        Some(root_entity.id())
    }

    // The first entity will have your specified root
    fn spawn_with_root(
        &mut self,
        processed_children: &HashMap<usize, Vec<usize>>,
        init: &usize,
        world: &mut World,
        root: Entity,
    ) -> Option<Entity>
    where
        T: PrefabData,
    {
        //Entity must exist
        let mut root = world.entity_mut(root);
        if let Some(c) = processed_children.get(init) {
            if let Some(parent) = self.entidades.get_mut(*init) {
                if let Some(data) = parent.take_data() {
                    data.insert_into_entity(&mut root);
                }
            }
            for child in c {
                // Safety: Its safe because the location is updated after the operation.
                unsafe {
                    let child = self.spawn(processed_children, child, root.world_mut());
                    root.update_location();
                    if let Some(child) = child {
                        root.add_child(child);
                    }
                }
            }
        } else {
            // Probably an error with wrong adding the children.
            warn!(target: "prefab", "[warning] Entity not found in the prefab with index {}", init);
            return None;
        }
        Some(root.id())
    }

    /// This function will load all the assets
    /// If you want to be performatic, store the assets and just borrow then afterwards
    pub fn prepare_entities(&mut self, world: &mut World) -> bool
    where
        T: PrefabData,
    {
        let mut loaded = false;
        for entidade in self.entidades.iter_mut() {
            loaded |= entidade.prepare_data(world)
        }
        loaded
    }
}
impl<T> Default for Prefab<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Clone for Prefab<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            entidades: self.entidades.clone(),
        }
    }
}

pub struct PrefabEntityBuilder<T> {
    parent: Option<usize>,
    data: Option<T>,
}
impl<T> PrefabEntityBuilder<T> {
    pub fn new(parent: Option<usize>, data: Option<T>) -> Self {
        Self { parent, data }
    }

    pub fn get_data_or_insert_with(&mut self, f: impl FnOnce() -> T) -> &mut T {
        self.data.get_or_insert_with(f)
    }

    pub fn get_data_or_default(&mut self) -> &mut T
    where
        T: Default,
    {
        self.data.get_or_insert_with(T::default)
    }

    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn take_data(&mut self) -> Option<T> {
        self.data.take()
    }

    pub fn prepare_data(&mut self, world: &mut World) -> bool
    where
        T: PrefabData,
    {
        if let Some(data) = self.get_data_mut() {
            data.load_sub_assets(world)
        } else {
            false
        }
    }
}
impl<T> Default for PrefabEntityBuilder<T> {
    fn default() -> Self {
        Self {
            parent: None,
            data: None,
        }
    }
}
impl<T> Clone for PrefabEntityBuilder<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            data: self.data.clone(),
        }
    }
}

pub trait PrefabData: Send + Sync + 'static {
    /// Transform self into a component and insert into an entity
    fn insert_into_entity(self, entidade: &mut EntityWorldMut);
    fn load_sub_assets(&mut self, _world: &mut World) -> bool {
        false
    }
}

/// Suport trait for simple data that can be easily converted to a Component
/// PrefabData is implemented for those who implement this trait + Clone
/// Mustn't implement this trait the Components based on assets loading.
pub trait IntoComponent {
    type Component: Component;
    fn into_component(self) -> Self::Component;
}

/// The formatter used for deserialize purposes
pub trait Format<PD>
where
    Self: Send + Sync,
{
    fn load_from_bytes(bytes: Vec<u8>) -> Result<Prefab<PD>, Box<dyn Error>>;
}

#[derive(Resource, Default)]
/// Helper to some prefab functions
pub struct PrefabLoader;

impl PrefabLoader {
    pub fn create_prefab<T: PrefabData, F: Format<T>, S: AsRef<Path>>(
        nome: S,
    ) -> Result<Prefab<T>, Box<dyn Error>> {
        let prefab = F::load_from_bytes(Self::load_prefab(nome)?)?;
        Ok(prefab)
    }

    /// Instead of spawning the prefab, just create the prefab
    /// - T: The prefab data that all the entities are bound for.
    /// - F: The formatter for loading the prefab.
    /// - S: The relative path of the prefab
    // pub fn lazy_create<T: PrefabData, F: Format<T>, S: AsRef<Path>>(&self, world: &mut World, nome: S) -> Result<Prefab<T>, Box<dyn Error>>{
    //     let mut prefab = F::load_from_bytes(Self::load_prefab(nome)?)?;

    //     prefab.prepare_entities(world);
    //     Ok(prefab)
    // }

    fn load_prefab<S: AsRef<Path>>(path: S) -> Result<Vec<u8>, io::Error> {
        let path = path.as_ref();
        let content: Vec<u8> = {
            let mut file = fs::File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            buffer
        };
        Ok(content)
    }
}
pub struct PrefabCommands<'c, 'w, 's, PD: PrefabData> {
    prefab: Prefab<PD>,
    commands: &'c mut Commands<'w, 's>,
}
impl<'c, 'w, 's, PD: PrefabData> PrefabCommands<'c, 'w, 's, PD> {
    pub fn get_entity_mut(&mut self, index: usize) -> Option<&mut PrefabEntityBuilder<PD>> {
        self.prefab.get_entity_mut(index)
    }

    pub fn get_entity(&self, index: usize) -> Option<&PrefabEntityBuilder<PD>> {
        self.prefab.get_entity(index)
    }

    pub fn add(&mut self, parent: Option<usize>, data: Option<PD>) -> usize {
        self.prefab.add(parent, data)
    }

    pub fn spawn<B: Bundle>(self, bundle: B) -> Entity {
        self.commands.spawn_prefab_with(self.prefab, bundle)
    }

    pub fn spawn_empty(self) -> Entity {
        self.commands.spawn_prefab(self.prefab)
    }

    pub fn prepare_spawn_empty(self) -> Entity {
        self.commands.prepare_and_spawn_prefab(self.prefab)
    }

    pub fn prepare_spawn<B: Bundle>(self, bundle: B) -> Entity {
        self.commands
            .prepare_and_spawn_prefab_with(self.prefab, bundle)
    }

    pub fn prepare(self) {
        self.commands.prepare_prefab(self.prefab)
    }

    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.commands
    }

    pub fn prefab(self) -> Prefab<PD> {
        self.prefab
    }
}

pub trait PrefabCommandsExt<'c, 'w, 's> {
    /// Load the prefab and create the commands
    fn load_prefab<PD: PrefabData, F: Format<PD>, S: AsRef<Path>>(
        &'c mut self,
        name: S,
    ) -> PrefabCommands<'c, 'w, 's, PD>;

    /// Spawn an external prefab
    fn spawn_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>) -> Entity;

    /// Spawn an external prefab with a bundle in the parent
    fn spawn_prefab_with<PD: PrefabData, B: Bundle>(
        &mut self,
        prefab: Prefab<PD>,
        bundle: B,
    ) -> Entity;

    /// Prepare the assets and then Spawn
    fn prepare_and_spawn_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>) -> Entity;

    /// Prepare the assets and then Spawn with the given bundle
    fn prepare_and_spawn_prefab_with<PD: PrefabData, B: Bundle>(
        &mut self,
        prefab: Prefab<PD>,
        bundle: B,
    ) -> Entity;

    /// Take the prefab and will prepare, when loaded, the prefab will be returned as a [`LoadedPrefab`] event
    fn prepare_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>);

    /// Create prefab commands from a external prefab
    fn prefab_into_commands<PD: PrefabData>(
        &'c mut self,
        prefab: Prefab<PD>,
    ) -> PrefabCommands<'c, 'w, 's, PD>;
}

impl<'c, 'w, 's> PrefabCommandsExt<'c, 'w, 's> for Commands<'w, 's> {
    fn load_prefab<PD: PrefabData, F: Format<PD>, S: AsRef<Path>>(
        &'c mut self,
        name: S,
    ) -> PrefabCommands<'c, 'w, 's, PD> {
        let prefab = PrefabLoader::create_prefab::<PD, F, S>(name);
        let prefab = if let Err(e) = prefab {
            error!(target: "prefab", "[error] Prefab Error: {e}");
            Default::default()
        } else {
            prefab.unwrap()
        };
        PrefabCommands {
            prefab,
            commands: self,
        }
    }

    fn spawn_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>) -> Entity {
        let entity = self.spawn_empty().id();

        self.add(SpawnPrefab(prefab, entity));
        entity
    }

    fn spawn_prefab_with<PD: PrefabData, B: Bundle>(
        &mut self,
        prefab: Prefab<PD>,
        bundle: B,
    ) -> Entity {
        let entity = self.spawn(bundle).id();

        self.add(SpawnPrefab(prefab, entity.clone()));
        entity
    }

    fn prefab_into_commands<PD: PrefabData>(
        &'c mut self,
        prefab: Prefab<PD>,
    ) -> PrefabCommands<'c, 'w, 's, PD> {
        PrefabCommands {
            prefab,
            commands: self,
        }
    }

    fn prepare_and_spawn_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>) -> Entity {
        let entity = self.spawn_empty().id();

        self.add(PrepareAndSpawnPrefab(prefab, entity.clone()));
        entity
    }

    fn prepare_and_spawn_prefab_with<PD: PrefabData, B: Bundle>(
        &mut self,
        prefab: Prefab<PD>,
        bundle: B,
    ) -> Entity {
        let entity = self.spawn(bundle).id();

        self.add(PrepareAndSpawnPrefab(prefab, entity.clone()));
        entity
    }

    fn prepare_prefab<PD: PrefabData>(&mut self, prefab: Prefab<PD>) {
        self.add(PreparePrefab(prefab));
    }
}

pub struct SpawnPrefab<PD: PrefabData>(Prefab<PD>, Entity);
impl<PD: PrefabData> Command for SpawnPrefab<PD> {
    fn apply(mut self, world: &mut World) {
        let processed_children = self.0.all_parents_childs();

        self.0
            .spawn_with_root(&processed_children, &0, world, self.1)
            .unwrap();
    }
}

/// This command will consume the prefab after the spawning.
/// If you want to prepare and retrieve the prefab, use PreparePrefab
pub struct PrepareAndSpawnPrefab<PD: PrefabData>(Prefab<PD>, Entity);
impl<PD: PrefabData> Command for PrepareAndSpawnPrefab<PD> {
    fn apply(self, world: &mut World) {
        let mut prefab = self.0;
        let resultado = prefab.prepare_entities(world);

        if resultado {
            warn!(target: "prefab", "[warn] Not all the prefab assets was loaded")
        } else {
            prefab.spawn_with_root(&prefab.all_parents_childs(), &0, world, self.1);
        }
    }
}

pub struct PreparePrefab<PD: PrefabData>(Prefab<PD>);
impl<PD: PrefabData> Command for PreparePrefab<PD> {
    fn apply(self, world: &mut World) {
        let mut prefab = self.0;
        let resultado = prefab.prepare_entities(world);
        let event = LoadedPrefab {
            prefab,
            succefully_loaded: resultado,
        };
        world.send_event(event);
    }
}

#[derive(Component)]
pub struct PrefabEntity<PD>(PhantomData<PD>);

#[derive(Default)]
pub struct PrefabPlugin<PD: PrefabData> {
    marker: PhantomData<PD>,
}
impl<PD: PrefabData> PrefabPlugin<PD> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData::default(),
        }
    }
}

impl<PD: PrefabData> Plugin for PrefabPlugin<PD> {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadedPrefab<PD>>();
    }
}

#[derive(Event)]
pub struct LoadedPrefab<PD> {
    pub prefab: Prefab<PD>,
    pub succefully_loaded: bool,
}

pub mod implements {
    use super::{IntoComponent, PrefabData};
    use bevy::ecs::world::{EntityWorldMut, World};

    impl<T> PrefabData for Option<T>
    where
        T: PrefabData,
    {
        fn insert_into_entity(self, entidade: &mut EntityWorldMut) {
            if let Some(data) = self {
                data.insert_into_entity(entidade)
            }
        }
        fn load_sub_assets(&mut self, world: &mut World) -> bool {
            if let Some(data) = self {
                return data.load_sub_assets(world);
            };
            false
        }
    }

    impl<T> PrefabData for T
    where
        T: IntoComponent + Send + Sync + 'static,
    {
        fn insert_into_entity(self, entidade: &mut EntityWorldMut) {
            entidade.insert(self.into_component());
        }
    }

    macro_rules! impl_prefab_data {
        ( $($ty:ident:$i:tt),* ) => {
            #[allow(unused)]
            impl<$($ty),*> PrefabData for ( $( $ty , )* )
                where $( $ty : PrefabData ),*
            {

                fn insert_into_entity(
                    self,
                    entidade: &mut EntityWorldMut
                ) {
                    #![allow(unused_variables)]
                    $(
                        self.$i.insert_into_entity(entidade);
                    )*
                }

                fn load_sub_assets(
                    &mut self, world: &mut World
                ) -> bool {
                    let mut ret = false;
                    $(
                        ret |= self.$i.load_sub_assets(world);
                    )*
                    ret
                }

            }
        };
    }

    impl_prefab_data!();
    impl_prefab_data!(A:0);
    impl_prefab_data!(A:0, B:1);
    impl_prefab_data!(A:0, B:1, C:2);
    impl_prefab_data!(A:0, B:1, C:2, D:3);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19);
    impl_prefab_data!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19, U:20);
}

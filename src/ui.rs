use std::{marker::PhantomData, path::Path, collections::HashMap};

use bevy::{prelude::{Plugin, Event, Entity, Update}, ecs::{system::{Commands, SystemId, Resource, Query}, component::Component, event::EventWriter, query::Changed}, ui::Interaction, log::{warn, error}};

use serde::{Serialize, Deserialize};

use crate::prefab::{PrefabData, Format, Prefab, PrefabPlugin, PrefabCommandsExt, PrefabCommands};

use super::components::ui::{NodeBundlePrefab, ImageBundlePrefab, ButtonBundlePrefab, TextBundlePrefab};

#[derive(Deserialize, Serialize)]
pub enum Ui<C = NoCustomWidget>
where C: CustomWidget
{

    /// A box, used for root purposes
    Container {
        node: NodeBundlePrefab,
        #[serde(default = "empty_vec")] // Will add no children if required
        children: Vec<Ui<C>>,
        #[serde(default)]
        custom_data: Option<C::CustomData>
    },

    /// Simple label with text
    Text {
        text_data: TextBundlePrefab,
        #[serde(default)]
        custom_data: Option<C::CustomData>
    },

    /// Simple image box
    Image {
        image_data: ImageBundlePrefab,
        #[serde(default)]
        custom_data: Option<C::CustomData>
    },

    /// A button.
    Button {
        button: ButtonBundlePrefab,
        #[serde(default)]
        label: Option<TextBundlePrefab>,
        #[serde(default)]
        callback: Option<CallbackPrefab>,
        #[serde(default)]
        custom_data: Option<C::CustomData>
    },

    /// Custom Ui
    Custom(Box<C>)
}

pub fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}

pub type UiData<CD = <NoCustomWidget as CustomWidget>::CustomData> = (
    Option<NodeBundlePrefab>,
    Option<TextBundlePrefab>,
    Option<ImageBundlePrefab>,
    Option<ButtonBundlePrefab>,
    Option<CallbackPrefab>,
    Option<CD>
);

pub trait CustomWidget
where Self: Sized + Send + Sync + 'static
{
    /// Type to define your own component
    type CustomData: PrefabData + for<'a> Deserialize<'a> + Serialize;
    /// Function that return the custom as native.
    fn into_native(&self) -> Ui<Self>;
}

#[derive(Deserialize, Serialize)]
/// Disabled Custom Widget
pub struct NoCustomWidget;
impl CustomWidget for NoCustomWidget {
    type CustomData = ();

    fn into_native(&self) -> Ui<Self> {
        unreachable!()
    }
}

struct UiFormat<C: CustomWidget>(PhantomData<C>);

impl <C>Format<UiData<C::CustomData>> for UiFormat<C>
where C: CustomWidget + for<'a> Deserialize<'a>
{
    fn load_from_bytes(bytes: Vec<u8>) -> Result<Prefab<UiData<C::CustomData>>,  Box<dyn std::error::Error>> {
        let mut de = ron::de::Deserializer::from_bytes(&bytes)?;
        let valor = Ui::deserialize(&mut de)?;
        let mut prefab = Prefab::default();

        ui_tree::<C>(valor, 0, &mut prefab);
        Ok(prefab)
    }
}

fn ui_tree<C: CustomWidget>(
    widget: Ui<C>,
    index: usize,
    prefab: &mut Prefab<UiData<C::CustomData>>,
) {
    match widget {
        Ui::Custom(custom_widget) => {
            let widget = custom_widget.into_native();
            ui_tree(widget, index, prefab);
        }
        Ui::Text { text_data, custom_data } => {
            prefab.get_entity_mut(index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    None,
                    Some(text_data),
                    None,
                    None,
                    None,
                    custom_data
                ));
        }
        Ui::Image { image_data, custom_data } => {
            prefab.get_entity_mut(index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    None,
                    None,
                    Some(image_data),
                    None,
                    None,
                    custom_data
                ));
        }
        Ui::Button { button, label, callback,custom_data } => {
            prefab.get_entity_mut(index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((None, None, None, Some(button), callback, custom_data));

            if let Some(text) = label {
                prefab.add(
                    Some(index),
                    Some((
                        None,
                        Some(text),
                        None,
                        None,
                        None,
                        None
                    ))
                );
            }
        }
        Ui::Container { node, children, custom_data } => {
            prefab.get_entity_mut(index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(node),
                    None,
                    None,
                    None,
                    None,
                    custom_data
                ));

            for child in children {
                let child_index = prefab.add(Some(index), None);
                ui_tree(child, child_index, prefab);
            }
        }
    }
}

pub struct UiPrefabPlugin<C = NoCustomWidget>(PhantomData<C>) where C: CustomWidget;
impl <C: CustomWidget>Plugin for UiPrefabPlugin<C> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugins(PrefabPlugin::<UiData<C::CustomData>>::new())
        .insert_resource(UiButtonCallbacks::new())
        .add_systems(Update, handle_prefab_events)
        .add_event::<PressedButtonEvent>();
    }
} impl Default for UiPrefabPlugin {
    fn default() -> Self {
        Self(PhantomData)
    }
} impl <C: CustomWidget>UiPrefabPlugin<C> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

pub struct UiCreator<S, C = NoCustomWidget>(pub S, pub PhantomData<C>) where C: CustomWidget, S: AsRef<Path>;
impl <S: AsRef<Path>>UiCreator<S>
{
    pub fn load_simple(name: S) -> Self {
        Self(name, Default::default())
    }
} impl <S: AsRef<Path>, C: CustomWidget> UiCreator<S, C> {
    pub fn load_with_custom(name: S) -> Self {
        Self(name, Default::default())
    }
}

pub trait CreateUiExt<'c, 'w, 's, C>
where C: CustomWidget + for<'a> Deserialize<'a>
{
    fn load_ui<S: AsRef<Path>>(&'c mut self, creator: UiCreator<S, C>) -> PrefabCommands<'c, 'w, 's, UiData<C::CustomData>>;

}

impl <'c, 'w, 's, C: CustomWidget + for<'a> Deserialize<'a>>CreateUiExt<'c, 'w, 's, C> for Commands<'w, 's> {
    fn load_ui<S: AsRef<Path>>(&'c mut self, creator: UiCreator<S, C>) -> PrefabCommands<'c, 'w, 's, UiData<C::CustomData>> {
        self.load_prefab::<UiData<C::CustomData>, UiFormat<C>, S>(creator.0)
    }
}

/// Wrapper around callback given to prefab created buttons
#[derive(Component)]
pub enum CallBack {
    /// When clicked, will call the callback function
    RegisteredSystem(SystemId),
    /// If the callback is EventSender, it's the user responsability to treat that
    EventSender(PressedButtonEvent)
}

#[derive(Event, Clone)]
pub struct PressedButtonEvent {
    pub button_name: String,
    pub entity: Entity,
    pub interaction: Interaction
} impl PressedButtonEvent {
    pub fn new(name: String, entity: Entity) -> Self {
        Self { button_name: name, entity, interaction: Default::default() }
    }
    /// Compare button name with given name
    pub fn is_name(&self, name: String) -> bool {
        self.button_name == name
    }
    /// Compare button entity with given entity
    pub fn is_entity(&self, entity: Entity) -> bool {
        self.entity == entity
    }
}

#[derive(Resource)]
pub struct UiButtonCallbacks {
    pub callbacks: HashMap<String, SystemId>
} impl UiButtonCallbacks {
    pub fn new() -> Self {
        Self { callbacks: HashMap::new() }
    }

    pub fn new_with_hash(callbacks: HashMap<String, SystemId>) -> Self {
        Self { callbacks }
    }

    pub fn push_callback(&mut self, name: String, callback: SystemId) -> Option<SystemId> {
        self.callbacks.insert(name, callback)
    }

    pub fn remove_callback(&mut self, name: String) -> Option<SystemId> {
        self.callbacks.remove(&name)
    }

    pub fn get_system(&self, name: &String) -> Option<SystemId> {
        if let Some(system) = self.callbacks.get(name) {
            // This will change in the furure
            Some(system.clone())
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum CallbackPrefab {
    System(String),
    Event {
        name: String
    },
    #[serde(skip)]
    LoadedSystem(SystemId)
} impl PrefabData for CallbackPrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let callback = match self {
            Self::LoadedSystem(s) => {
                Some(CallBack::RegisteredSystem(s))
            }
            Self::Event { name } => {
                Some(CallBack::EventSender(PressedButtonEvent::new(name, entidade.id())))
            }
            Self::System(_) => {
                warn!(target: "structs::ui", "Tried to insert unloaded Callback into entity");
                None
            }
        };

        if let Some(callback) = callback {
            entidade.insert(callback);
        }
    }

    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        match self {
            CallbackPrefab::System(name) => {
                let callbacks: Option<&UiButtonCallbacks> = _world.get_resource();

                let Some(callbacks) = callbacks else {
                    error!(target: "structs::ui", "The Resource UiButtonCallbacks is not inserted");
                    return true;
                };
                if let Some(system) = callbacks.get_system(name) {
                    *self = CallbackPrefab::LoadedSystem(system);
                    false
                } else {
                    warn!(target: "structs::ui", "Tried to get system {}, but failed", name);
                    true
                }
            }
            //Nothing to load
            _ => false
        }
    }
}

pub fn handle_prefab_events(mut commands: Commands, mut events: EventWriter<PressedButtonEvent>, buttons_callback: Query<(&CallBack, &Interaction), Changed<Interaction>>) {
    for (callback, interaction) in buttons_callback.iter() {
        match callback {
            CallBack::EventSender(e) => {
                events.send(PressedButtonEvent { button_name: e.button_name.clone(), entity: e.entity, interaction:  interaction.clone()})
            }
            // Enquanto não temos input, temos que runar isso sempre
            CallBack::RegisteredSystem(s) => {
                commands.run_system(s.clone());
            }
        }
    }
}
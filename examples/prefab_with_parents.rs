use bevy::prelude::*;
use bevy_prfb::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
    .add_systems(Startup, setup)
    .add_systems(Update, assert_is_loaded)
    .run();
}

#[derive(Component)]
struct Marker;
fn setup(mut cmd: Commands) {
    let prefab = cmd.load_prefab::<ValuePrefab, ValueFormatter, _>("assets/prefab/prefab_with_parents.ron");
    prefab.spawn(Marker);
}

fn assert_is_loaded(query_parent: Query<Entity, With<Marker>>, children: Query<&Children>, value_query: Query<&Value>) {
    let entity = query_parent.get_single().unwrap();
    if let Ok(v) = value_query.get(entity) {
        println!("Parent with value: {}", v.0);
    }

    for child in children.iter_descendants(entity) {
        if let Ok(v) = value_query.get(child) {
            println!("Child with value: {}", v.0);
        }
    }
}

#[derive(Component)]
struct Value(i32);
#[derive(Deserialize, Serialize)]
struct ValuePrefab(i32);
impl IntoComponent for ValuePrefab {
    type Component = Value;
    fn into_component(self) -> Self::Component {
        Value(self.0)
    }
}
impl ValuePrefab {
    fn add(mut self, val: i32) -> Self {
        self.0 += val;
        self
    }
}

#[derive(Deserialize, Serialize)]
enum ValueTree {
    Unique(ValuePrefab),
    Parent {
        children: Vec<ValueTree>,
        value: ValuePrefab
    }
}

struct ValueFormatter;
impl Format<ValuePrefab> for ValueFormatter {
    fn load_from_bytes(bytes: Vec<u8>) -> Result<Prefab<ValuePrefab>,  Box<dyn std::error::Error>> {
        let mut de = ron::de::Deserializer::from_bytes(&bytes)?;
        let valor = ValueTree::deserialize(&mut de)?;
        let mut prefab = Prefab::new();
        value_tree(valor, &mut prefab, 0, 0);
        Ok(prefab)
    }
}

fn value_tree(
    value: ValueTree,
    prefab: &mut Prefab<ValuePrefab>,
    index: usize,
    parent_value: i32
) {
    match value {
        ValueTree::Unique(v) => {
            prefab.get_entity_mut(index)
                .expect("Unreachable: Entity should be set")
                .set_data(v.add(parent_value));
        }
        ValueTree::Parent { children, value } => {
            let parent_val = value.0 + parent_value;
            prefab.get_entity_mut(index)
                .expect("Unreachable: Entity should be set")
                .set_data(value.add(parent_value));

            for child in children {
                let child_index = prefab.add(Some(index), None);
                value_tree(child, prefab, child_index, parent_val)
            }
        }
    }
}
use bevy::prelude::*;
use bevy_prfb::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_systems(Startup, load_prefab)
        .add_systems(Update, assert_is_loaded)
        .run();
}

fn load_prefab(mut cmd: Commands) {
    let prefab =
        cmd.load_prefab::<ValuePrefab, ValueFormatter, _>("assets/prefab/custom_prefab.ron");
    prefab.spawn_empty();
}

fn assert_is_loaded(query: Query<&Value>) {
    for value in query.iter() {
        println!("Loaded value: {}", value.0)
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

struct ValueFormatter;
impl Format<ValuePrefab> for ValueFormatter {
    fn load_from_bytes(bytes: Vec<u8>) -> Result<Prefab<ValuePrefab>, Box<dyn std::error::Error>> {
        let mut de = ron::de::Deserializer::from_bytes(&bytes)?;
        let valor = ValuePrefab::deserialize(&mut de)?;
        let prefab = Prefab::from_data(Some(valor));

        Ok(prefab)
    }
}

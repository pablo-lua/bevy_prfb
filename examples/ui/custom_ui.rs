use bevy::prelude::*;
use bevy_prfb::{
    a11y::AccessibilityNodePrefab,
    components::ui::{General, LabelPrefab, NodeBundlePrefab},
    ui::{CreateUiExt, CustomWidget, Ui, UiCreator},
    *,
};
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

type MyUi = Ui<MyCustomWidget>;
#[derive(Clone, Deserialize, Serialize)]
enum MyCustomWidget {
    Repeat {
        //Parent configuration
        #[serde(default)]
        node: NodeBundlePrefab,
        custom_data: Option<<Self as CustomWidget>::CustomData>,
        other_data: Option<General>,
        ui_to_repeat: MyUi,
        times: usize,
    },
}
impl CustomWidget for MyCustomWidget {
    type CustomData = UiData;
    fn into_native(self) -> Ui<Self> {
        match self {
            MyCustomWidget::Repeat {
                node,
                ui_to_repeat,
                times,
                custom_data,
                other_data,
            } => {
                let mut children = Vec::new();
                for i in 0..times {
                    children.push(take_ui(ui_to_repeat.clone(), i + 1))
                }
                Ui::Container {
                    node,
                    children,
                    custom_data,
                    other_data,
                }
            }
        }
    }
}
#[derive(Clone, Deserialize, PrefabData, Serialize)]
struct UiData {
    #[serde(default)]
    accessibility: Option<AccessibilityNodePrefab>,
    #[serde(default)]
    label: Option<LabelPrefab>,
}

fn take_ui(child: MyUi, i: usize) -> MyUi {
    if let MyUi::Text {
        mut text_data,
        custom_data,
    } = child
    {
        let mut_text = text_data.text.sections.get_mut(0);
        if let Some(text) = mut_text {
            text.text.push_str(format!(" {i}").as_str());
        }
        MyUi::Text {
            text_data,
            custom_data,
        }
    } else {
        unreachable!()
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
    let menu = cmd.load_ui(UiCreator::<_, MyCustomWidget>::load_with_custom(
        "assets/ui/custom_ui.ron",
    ));
    menu.prepare_spawn_empty();
}

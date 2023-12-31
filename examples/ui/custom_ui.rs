use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};
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
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_scroll)
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
                    children.push(take_ui(ui_to_repeat.clone(), i))
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
    list: Option<ScrollingList>,
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

#[derive(Clone, Deserialize, Serialize, Component, Default)]
pub struct ScrollingList {
    position: f32,
}
impl IntoComponent for ScrollingList {
    type Component = Self;
    fn into_component(self) -> Self::Component {
        self
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

use bevy::{
    prelude::*,
    //includes to fix error spam
    render::{settings::{Backends, WgpuSettings}, RenderPlugin},
};

fn main() {
    App::new()
        .add_plugins(
            //fix error spam
            DefaultPlugins.set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
            }),
        )
        .run();
}
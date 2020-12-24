use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_resource(Msaa { samples: 4 })
            .add_resource(WindowDescriptor {
                title: "#!> Quantum  Force <!#".to_string(),
                ..Default::default()
            });
    }
}

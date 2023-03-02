use bevy::prelude::*;

mod block;

fn main() {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins).add_startup_system(setup);
  block::init(&mut app);
  app.run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

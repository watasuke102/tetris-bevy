use bevy::prelude::*;

mod mino;
mod field;

fn main() {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins).add_startup_system(setup);
  field::init(&mut app);
  mino::init(&mut app);
  app.run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

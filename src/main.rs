use bevy::prelude::*;

mod entry;
mod field;
mod mino;

const WINDOW_WIDTH: f32 = field::BLOCK_SIZE * (field::FIELD_COLUMN + 4) as f32;
const WINDOW_HEIGHT: f32 = field::BLOCK_SIZE * field::FIELD_ROW as f32;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..default()
      },
      ..default()
    }))
    .add_startup_system(setup);
  entry::init(&mut app);
  field::init(&mut app);
  mino::init(&mut app);
  app.run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

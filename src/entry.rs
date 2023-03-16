use bevy::prelude::*;

use crate::{field, mino};

pub fn init(app: &mut App) {
  app.add_system(press_key_to_start);
}

fn press_key_to_start(
  commands: Commands,
  key_input: Res<Input<KeyCode>>,
  query: Query<&mino::Mino>,
  field: ResMut<field::Field>,
  field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  timer: ResMut<mino::MinoDropTimer>,
) {
  if !key_input.just_pressed(KeyCode::Space) {
    return;
  }
  if let Err(_) = query.get_single() {
    mino::startup_mino(commands, field, field_block_query, timer);
  }
}

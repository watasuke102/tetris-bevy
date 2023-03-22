use std::time::Duration;

use bevy::prelude::*;

use crate::{field, mino_type, next};
#[derive(Component, Default)]
pub struct Mino {
  pos:       IVec2,
  mino_type: usize,
  blocks:    mino_type::MinoBlocks,
}

impl Mino {
  fn set_type(
    &mut self,
    mut next_mino: ResMut<next::NextMino>,
    mut field: ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
    timer: &mut ResMut<MinoDropTimer>,
  ) {
    field.delete_filled_line(&mut field_block_query);
    self.pos = IVec2::new(5, field::FIELD_ROW_HIDDEN);
    self.mino_type = next_mino.pop();
    self.blocks = mino_type::MINO_TYPES[self.mino_type].blocks.clone();
    for e in self.blocks {
      field.set_block(
        &mut field_block_query,
        self.pos + e,
        Color::hex(mino_type::MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }
    timer.0.reset();
  }

  fn move_mino(
    &mut self,
    diff: IVec2,
    field: &mut ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
  ) -> Result<(), ()> {
    for e in self.blocks {
      field.unset_block(field_block_query, self.pos + e);
    }

    for e in self.blocks {
      let pos = self.pos + e + diff;
      if !field.is_movable_pos(&pos) {
        for e in self.blocks {
          field.set_block(
            field_block_query,
            self.pos + e,
            Color::hex(mino_type::MINO_TYPES[self.mino_type].color).unwrap(),
          );
        }
        return Err(());
      }
    }

    self.pos += diff;
    for e in self.blocks {
      field.set_block(
        &mut field_block_query,
        self.pos + e,
        Color::hex(mino_type::MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }

    Ok(())
  }

  // FIXME: wrong behavior on I + near the wall
  fn rotate(
    &mut self,
    ccw: bool,
    field: &mut ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
  ) {
    // skip O mino
    if self.mino_type == 5 {
      return;
    }

    let origin = self.blocks.clone();
    let mut failed = false;

    for e in &mut self.blocks {
      field.unset_block(field_block_query, self.pos + *e);
    }

    for e in &mut self.blocks {
      let tmp = e.x;
      if ccw {
        e.x = e.y;
        e.y = -tmp;
      } else {
        e.x = -e.y;
        e.y = tmp;
      }
      let new_pos = self.pos + *e;
      if !field.is_movable_pos(&new_pos) {
        failed = true;
        break;
      }
    }

    if failed {
      self.blocks = origin.clone();
    }

    for e in self.blocks {
      field.set_block(
        &mut field_block_query,
        self.pos + e,
        Color::hex(mino_type::MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }
  }
}

#[derive(Resource, Default)]
pub struct MinoDropTimer(pub Timer);

#[derive(Resource, Default)]
pub struct KeyRepeat {
  duration:     Duration,
  repeat_timer: Timer,
  previous:     Option<KeyCode>,
}
impl KeyRepeat {
  fn new(repeat_timer: Timer) -> Self {
    KeyRepeat {
      repeat_timer,
      ..default()
    }
  }
}

pub fn init(app: &mut App) {
  app.add_system(drop_mino);
  app.add_system(move_mino);
  app.add_system(rotate_mino);

  let mut drop_timer = Timer::new(Duration::from_millis(300), TimerMode::Repeating);
  drop_timer.pause();
  app.insert_resource(MinoDropTimer(drop_timer));

  let mut repeat_timer = Timer::new(Duration::from_millis(50), TimerMode::Repeating);
  repeat_timer.pause();
  app.insert_resource(KeyRepeat::new(repeat_timer));
}

fn drop_mino(
  mut query: Query<&mut Mino>,
  next_mino: ResMut<next::NextMino>,
  mut field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  mut timer: ResMut<MinoDropTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta());
  if !timer.0.finished() {
    return;
  }

  let Ok(mut mino) = query.get_single_mut() else {return;};
  if let Err(_) = mino.move_mino(IVec2::new(0, 1), &mut field, &mut field_block_query) {
    field.delete_filled_line(&mut field_block_query);
    mino.set_type(next_mino, field, &mut field_block_query, &mut timer);
  }
}

fn move_mino(
  key_input: Res<Input<KeyCode>>,
  mut keyrepeat: ResMut<KeyRepeat>,
  time: Res<Time>,
  next_mino: ResMut<next::NextMino>,
  mut query: Query<&mut Mino>,
  mut field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  mut drop_timer: ResMut<MinoDropTimer>,
) {
  let Ok(mut mino) = query.get_single_mut() else {return;};

  let iter = [
    (KeyCode::Left, IVec2::new(-1, 0)),
    (KeyCode::Right, IVec2::new(1, 0)),
  ];

  let mut pressed = false;
  for e in iter {
    if key_input.pressed(e.0) {
      pressed = true;
      match keyrepeat.previous {
        Some(code) => {
          if code == e.0 {
            keyrepeat.duration += time.delta();
            if keyrepeat.duration > Duration::from_millis(150) {
              if keyrepeat.repeat_timer.paused() {
                keyrepeat.repeat_timer.reset();
                keyrepeat.repeat_timer.unpause();
              }
              keyrepeat.repeat_timer.tick(time.delta());
            }
          } else {
            keyrepeat.repeat_timer.pause();
            keyrepeat.duration = time.delta();
          }
        }
        None => keyrepeat.duration = time.delta(),
      }
      keyrepeat.previous = Some(e.0);
      if keyrepeat.duration == time.delta() ||
        (keyrepeat.duration > Duration::from_millis(200) && keyrepeat.repeat_timer.finished())
      {
        let _ = mino.move_mino(e.1, &mut field, &mut field_block_query);
        break;
      }
    }
  }

  if !pressed {
    keyrepeat.repeat_timer.pause();
    keyrepeat.previous = None;
  }

  if key_input.just_pressed(KeyCode::Up) {
    loop {
      if let Err(_) = mino.move_mino(IVec2::new(0, 1), &mut field, &mut field_block_query) {
        mino.set_type(next_mino, field, &mut field_block_query, &mut drop_timer);
        break;
      }
    }
  }
}

fn rotate_mino(
  key_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Mino>,
  mut field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
) {
  let Ok(mut mino) = query.get_single_mut() else {return;};
  if key_input.just_pressed(KeyCode::X) {
    mino.rotate(false, &mut field, &mut field_block_query);
  } else if key_input.just_pressed(KeyCode::Z) {
    mino.rotate(true, &mut field, &mut field_block_query);
  }
}

pub fn startup_mino(
  mut commands: Commands,
  next_mino: ResMut<next::NextMino>,
  field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  mut timer: ResMut<MinoDropTimer>,
) {
  let mut mino = Mino::default();
  mino.set_type(next_mino, field, &mut field_block_query, &mut timer);
  commands.spawn(mino);

  timer.0.reset();
  timer.0.unpause();
}

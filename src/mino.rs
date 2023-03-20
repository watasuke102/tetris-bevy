use bevy::prelude::*;

use crate::field;

type MinoBlocks = [IVec2; 4];
struct MinoType {
  // 1(-1, -1)  3(0, -1)  5(1, -1)
  // 2(-1,  0)  4(0,  0)  6(1,  0)
  blocks: MinoBlocks,
  color:  &'static str,
}
const MINO_TYPES: [MinoType; 7] = [
  // I
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
      IVec2 { x: 2, y: 0 },
    ],
    color:  "56b6c2",
  },
  // J
  MinoType {
    blocks: [
      IVec2 { x: -1, y: -1 },
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "61afef",
  },
  // L
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
      IVec2 { x: 1, y: -1 },
    ],
    color:  "d69363",
  },
  // S
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 0, y: 1 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "98c379",
  },
  // Z
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 0, y: 1 },
      IVec2 { x: 1, y: 1 },
    ],
    color:  "e06c75",
  },
  // O
  MinoType {
    blocks: [
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: -1 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "e5c07b",
  },
  // T
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "c678dd",
  },
];

#[derive(Component, Default)]
pub struct Mino {
  pos:       IVec2,
  mino_type: usize,
  blocks:    MinoBlocks,
}

impl Mino {
  fn set_type(
    &mut self,
    mino_type: usize,
    mut field: ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
    timer: &mut ResMut<MinoDropTimer>,
  ) {
    self.pos = IVec2::new(5, field::FIELD_ROW_HIDDEN);
    self.mino_type = mino_type;
    self.blocks = MINO_TYPES[self.mino_type].blocks.clone();
    for e in self.blocks {
      field.set_block(
        &mut field_block_query,
        self.pos + e,
        Color::hex(MINO_TYPES[self.mino_type].color).unwrap(),
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
            Color::hex(MINO_TYPES[self.mino_type].color).unwrap(),
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
        Color::hex(MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }

    Ok(())
  }

  // FIXME: wrong behavior on O / I + near the wall
  fn rotate(
    &mut self,
    ccw: bool,
    field: &mut ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
  ) {
    let origin = self.blocks.clone();
    let mut failed = false;

    for e in &mut self.blocks {
      field.unset_block(field_block_query, self.pos + *e);
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
        Color::hex(MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }
  }
}

#[derive(Resource, Default)]
pub struct MinoDropTimer(pub Timer);

pub fn init(app: &mut App) {
  app.add_system(drop_mino);
  app.add_system(move_mino);
  app.add_system(rotate_mino);

  let mut timer = Timer::new(std::time::Duration::from_millis(300), TimerMode::Repeating);
  timer.pause();
  app.insert_resource(MinoDropTimer(timer));
}

fn drop_mino(
  mut query: Query<&mut Mino>,
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
    let mino_type = mino.mino_type;
    mino.set_type(
      (mino_type + 1) % MINO_TYPES.len(),
      field,
      &mut field_block_query,
      &mut timer,
    );
  }
}

fn move_mino(
  key_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Mino>,
  mut field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  mut timer: ResMut<MinoDropTimer>,
) {
  let Ok(mut mino) = query.get_single_mut() else {return;};
  if key_input.just_pressed(KeyCode::Left) {
    let _ = mino.move_mino(IVec2::new(-1, 0), &mut field, &mut field_block_query);
  } else if key_input.just_pressed(KeyCode::Right) {
    let _ = mino.move_mino(IVec2::new(1, 0), &mut field, &mut field_block_query);
  }
  if key_input.just_pressed(KeyCode::Up) {
    loop {
      if let Err(_) = mino.move_mino(IVec2::new(0, 1), &mut field, &mut field_block_query) {
        let mino_type = mino.mino_type;
        mino.set_type(
          (mino_type + 1) % MINO_TYPES.len(),
          field,
          &mut field_block_query,
          &mut timer,
        );
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
  field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
  mut timer: ResMut<MinoDropTimer>,
) {
  let mut mino = Mino::default();
  mino.set_type(0, field, &mut field_block_query, &mut timer);
  commands.spawn(mino);

  timer.0.reset();
  timer.0.unpause();
}

use bevy::prelude::*;

use crate::field;

struct MinoType {
  // 1(-1, -1)  3(0, -1)  5(1, -1)
  // 2(-1,  0)  4(0,  0)  6(1,  0)
  blocks: [IVec2; 4],
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
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: -1 },
    ],
    color:  "98c379",
  },
  // Z
  MinoType {
    blocks: [
      IVec2 { x: -1, y: -1 },
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
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
}

impl Mino {
  fn set_type(
    &mut self,
    mino_type: usize,
    mut field: ResMut<field::Field>,
    mut field_block_query: &mut Query<(&mut Sprite, &mut field::FieldBlock)>,
  ) {
    self.pos = IVec2::new(5, 5);
    self.mino_type = mino_type;
    for e in MINO_TYPES[self.mino_type].blocks {
      field.set_block(
        &mut field_block_query,
        self.pos + e,
        Color::hex(MINO_TYPES[self.mino_type].color).unwrap(),
      );
    }
  }
}

#[derive(Resource, Default)]
struct MinoDropTimer(Timer);

pub fn init(app: &mut App) {
  app.add_system(move_mino);

  app.insert_resource(MinoDropTimer(Timer::new(
    std::time::Duration::from_millis(300),
    TimerMode::Repeating,
  )));
}

fn move_mino(
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

  let mut mino = query.single_mut();
  for e in MINO_TYPES[mino.mino_type].blocks {
    field.unset_block(&mut field_block_query, mino.pos + e);
  }

  for e in MINO_TYPES[mino.mino_type].blocks {
    let mut pos = mino.pos + e;
    pos.y += 1;
    if !field.is_movable_pos(&pos) {
      for e in MINO_TYPES[mino.mino_type].blocks {
        field.set_block(
          &mut field_block_query,
          mino.pos + e,
          Color::hex(MINO_TYPES[mino.mino_type].color).unwrap(),
        );
      }
      let mino_type = mino.mino_type;
      mino.set_type(
        (mino_type + 1) % MINO_TYPES.len(),
        field,
        &mut field_block_query,
      );
      return;
    }
  }

  mino.pos.y += 1;
  for e in MINO_TYPES[mino.mino_type].blocks {
    field.set_block(
      &mut field_block_query,
      mino.pos + e,
      Color::hex(MINO_TYPES[mino.mino_type].color).unwrap(),
    );
  }
}

pub fn startup_mino(
  mut commands: Commands,
  field: ResMut<field::Field>,
  mut field_block_query: Query<(&mut Sprite, &mut field::FieldBlock)>,
) {
  let mut mino = Mino::default();
  mino.set_type(0, field, &mut field_block_query);
  commands.spawn(mino);
}

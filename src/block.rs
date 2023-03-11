use bevy::prelude::*;

use crate::field;

// 1(-1, -1)  3(0, -1)  5(1, -1)
// 2(-1,  0)  4(0,  0)  6(1,  0)
const MINO_TYPES: [[IVec2; 4]; 7] = [
  [IVec2{x: -1, y:  0}, IVec2{x:  0, y:  0}, IVec2{x: 1, y:  0}, IVec2{x: 2, y:  0}], // I
  [IVec2{x: -1, y: -1}, IVec2{x: -1, y:  0}, IVec2{x: 0, y:  0}, IVec2{x: 1, y:  0}], // J
  [IVec2{x: -1, y:  0}, IVec2{x:  0, y:  0}, IVec2{x: 1, y:  0}, IVec2{x: 1, y: -1}], // L
  [IVec2{x: -1, y:  0}, IVec2{x:  0, y: -1}, IVec2{x: 0, y:  0}, IVec2{x: 1, y: -1}], // S
  [IVec2{x: -1, y: -1}, IVec2{x:  0, y: -1}, IVec2{x: 0, y:  0}, IVec2{x: 1, y:  0}], // Z
  [IVec2{x:  0, y: -1}, IVec2{x:  0, y:  0}, IVec2{x: 1, y: -1}, IVec2{x: 1, y:  0}], // O
  [IVec2{x: -1, y: 0},  IVec2{x:  0, y: -1}, IVec2{x: 0, y:  0}, IVec2{x: 1, y:  0}], // T
];

#[derive(Component, Default)]
struct Mino {
  pos:       IVec2,
  mino_type: usize,
}

impl Mino {
  fn set_type(
    &mut self,
    mino_type: usize,
    mut field_query: &mut Query<(&mut Sprite, &mut field::Field)>,
  ) {
    self.pos = IVec2::new(5, 5);
    self.mino_type = mino_type;
    for e in MINO_TYPES[self.mino_type] {
      field::set_block(
        &mut field_query,
        self.pos + e,
        Color::hex("98c379").unwrap(),
      );
    }
  }
}

#[derive(Resource, Default)]
struct MinoDropTimer(Timer);

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
  app.add_system(move_mino);

  app.insert_resource(MinoDropTimer(Timer::new(
    std::time::Duration::from_millis(500),
    TimerMode::Repeating,
  )));
}

fn move_mino(
  mut query: Query<&mut Mino>,
  mut field_query: Query<(&mut Sprite, &mut field::Field)>,
  mut timer: ResMut<MinoDropTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta());
  if !timer.0.finished() {
    return;
  }

  for mut mino in &mut query {
    if mino.pos.y == field::FIELD_ROW - 1 {
      let mino_type = mino.mino_type;
      mino.set_type((mino_type + 1) % MINO_TYPES.len(), &mut field_query);
      continue;
    }
    for e in MINO_TYPES[mino.mino_type] {
      field::unset_block(&mut field_query, mino.pos + e);
    }
    mino.pos.y += 1;
    for e in MINO_TYPES[mino.mino_type] {
      field::set_block(
        &mut field_query,
        mino.pos + e,
        Color::hex("98c379").unwrap(),
      );
    }
  }
}

fn startup(mut commands: Commands, mut field_query: Query<(&mut Sprite, &mut field::Field)>) {
  let mut mino = Mino::default();
  mino.set_type(0, &mut field_query);
  commands.spawn(mino);
}

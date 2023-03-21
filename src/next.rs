use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct NextMino {
  types: Vec<usize>,
}

impl NextMino {
  pub fn pop(&mut self) -> usize {
    let res = self.types.pop();
    if self.types.len() < 7 {
      self.append();
    }
    res.unwrap_or(0)
  }
  pub fn append(&mut self) {
    let mut seq: Vec<usize> = (0..7).collect();
    seq.shuffle(&mut rand::thread_rng());
    self.types.append(&mut seq);
  }
}

pub fn init(app: &mut App) {
  let mut next_mino = NextMino { types: Vec::new() };
  next_mino.append();
  next_mino.append();
  app.insert_resource(next_mino);
}

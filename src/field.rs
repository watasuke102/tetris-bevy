use bevy::prelude::*;

pub const BLOCK_SIZE: f32 = 24.0;

pub const FIELD_ROW_HIDDEN: i32 = 4;
pub const FIELD_ROW: i32 = 20 + FIELD_ROW_HIDDEN;
pub const FIELD_COLUMN: i32 = 10;

pub const FIELD_X: f32 = -(BLOCK_SIZE * FIELD_COLUMN as f32) / 2.;
pub const FIELD_Y: f32 = (BLOCK_SIZE * FIELD_ROW as f32) / 2. - 60.;

#[derive(Component, Default)]
pub struct FieldBlock {
  pos:     IVec2,
  visible: bool,
}

#[derive(Clone, Copy)]
struct FieldStatus {
  exist: bool,
  color: Color,
}

impl Default for FieldStatus {
  fn default() -> Self {
    FieldStatus {
      exist: false,
      color: Color::hex("282c34").unwrap(),
    }
  }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct Field {
  blocks: [[FieldStatus; FIELD_COLUMN as usize]; FIELD_ROW as usize],
}

impl Field {
  pub fn is_movable_pos(&self, pos: &IVec2) -> bool {
    if pos.x < 0 || pos.x >= FIELD_COLUMN {
      return false;
    }
    if pos.y >= FIELD_ROW {
      return false;
    }
    if self.blocks[pos.y as usize][pos.x as usize].exist {
      return false;
    }
    true
  }

  pub fn set_block(
    &mut self,
    query: &mut Query<(&mut Sprite, &mut FieldBlock)>,
    pos: IVec2,
    color: Color,
  ) -> bool {
    self.blocks[pos.y as usize][pos.x as usize] = FieldStatus { exist: true, color };
    for (mut sprite, field_block) in query {
      if field_block.pos != pos {
        continue;
      }
      if !field_block.visible {
        break;
      }
      sprite.color = color;
      break;
    }

    if pos.y == FIELD_ROW - 1 {
      return true;
    }
    return false;
  }

  pub fn unset_block(&mut self, query: &mut Query<(&mut Sprite, &mut FieldBlock)>, pos: IVec2) {
    let color = Color::hex("282c34").unwrap();
    self.blocks[pos.y as usize][pos.x as usize] = FieldStatus {
      exist: false,
      color,
    };
    for (mut sprite, field_block) in query {
      if field_block.pos != pos {
        continue;
      }
      if !field_block.visible {
        break;
      }
      sprite.color = color;
      break;
    }
  }

  fn reflesh(&mut self, query: &mut Query<(&mut Sprite, &mut FieldBlock)>) {
    for (mut sprite, field_block) in query {
      if field_block.visible {
        sprite.color = self.blocks[field_block.pos.y as usize][field_block.pos.x as usize].color;
      }
    }
  }

  pub fn delete_filled_line(&mut self, query: &mut Query<(&mut Sprite, &mut FieldBlock)>) {
    let mut filled_line: Vec<usize> = Vec::new();
    for (i, e) in self.blocks.iter().enumerate() {
      if let None = e.iter().filter(|a| !a.exist).next() {
        filled_line.push(i);
      }
    }
    if filled_line.is_empty() {
      return;
    }

    let mut insert_dst = self.blocks.len() - 1;
    for i in (0..self.blocks.len() - 1).rev() {
      if filled_line.contains(&i) {
        continue;
      }
      self.blocks[insert_dst] = self.blocks[i];
      insert_dst -= 1;
    }

    self.reflesh(query);
  }
}

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
}

fn startup(mut commands: Commands) {
  commands.init_resource::<Field>();

  for i in 0..FIELD_COLUMN {
    for j in 0..FIELD_ROW {
      let visible = j >= FIELD_ROW_HIDDEN;
      commands.spawn((
        SpriteBundle {
          sprite: Sprite {
            color: Color::hex(if visible { "282c34" } else { "00000000" }).unwrap(),
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            ..default()
          },
          transform: Transform::from_xyz(
            (BLOCK_SIZE / 2.) + (FIELD_X + i as f32 * BLOCK_SIZE),
            (BLOCK_SIZE / 2.) + (FIELD_Y - (j - FIELD_ROW_HIDDEN + 1) as f32 * BLOCK_SIZE),
            0.0,
          ),
          ..default()
        },
        FieldBlock {
          pos:     IVec2 { x: i, y: j },
          visible: visible,
        },
      ));
    }
  }
}

use bevy::prelude::*;

const BLOCK_SIZE: f32 = 24.0;
const FIELD_X: f32 = -200.0;
const FIELD_Y: f32 = 400.0;
pub const FIELD_ROW: i32 = 20 + 5;
const FIELD_COLUMN: i32 = 10;

#[derive(Component, Default)]
pub struct FieldBlock {
  pos:       IVec2,
  has_block: bool,
  visible:   bool,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct Field {
  blocks: [[bool; FIELD_ROW as usize]; FIELD_COLUMN as usize],
}

impl Field {
  pub fn is_movable_pos(&self, pos: &IVec2) -> bool {
    if pos.y == FIELD_ROW {
      return false;
    }
    if self.blocks[pos.x as usize][pos.y as usize] {
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
    self.blocks[pos.x as usize][pos.y as usize] = true;
    for (mut sprite, mut field) in query {
      if field.pos != pos {
        continue;
      }
      if !field.visible {
        break;
      }
      field.has_block = true;
      sprite.color = color;
      break;
    }

    if pos.y == FIELD_ROW - 1 {
      return true;
    }
    return false;
  }

  pub fn unset_block(&mut self, query: &mut Query<(&mut Sprite, &mut FieldBlock)>, pos: IVec2) {
    self.blocks[pos.x as usize][pos.y as usize] = false;
    for (mut sprite, mut field) in query {
      if field.pos != pos {
        continue;
      }
      if !field.visible {
        break;
      }
      field.has_block = false;
      sprite.color = Color::hex("282c34").unwrap();
      break;
    }
  }
}

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
}

fn startup(mut commands: Commands) {
  commands.init_resource::<Field>();

  for i in 0..FIELD_COLUMN {
    for j in 0..FIELD_ROW {
      let visible = j >= 5;
      commands.spawn((
        SpriteBundle {
          sprite: Sprite {
            color: Color::hex(if visible { "282c34" } else { "00000000" }).unwrap(),
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            ..default()
          },
          transform: Transform::from_xyz(
            FIELD_X + i as f32 * BLOCK_SIZE,
            FIELD_Y - j as f32 * BLOCK_SIZE,
            0.0,
          ),
          ..default()
        },
        FieldBlock {
          pos:       IVec2 { x: i, y: j },
          has_block: false,
          visible:   visible,
        },
      ));
    }
  }
}

use bevy::prelude::*;

pub const BLOCK_SIZE: f32 = 32.0;
const FIELD_X: f32 = -200.0;
const FIELD_Y: f32 = 400.0;

#[derive(Component, Default)]
pub struct Field {
  pos:       IVec2,
  has_block: bool,
  visible:   bool,
}

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
}

fn startup(mut commands: Commands) {
  for i in 0..10 {
    for j in 0..20 + 5 {
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
        Field {
          pos:       IVec2 { x: i, y: j },
          has_block: false,
          visible:   visible,
        },
      ));
    }
  }
}

pub fn set_block(query: &mut Query<(&mut Sprite, &mut Field)>, pos: IVec2, color: Color) {
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
}

pub fn unset_block(query: &mut Query<(&mut Sprite, &mut Field)>, pos: IVec2) {
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

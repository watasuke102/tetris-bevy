use bevy::prelude::*;

#[derive(Component)]
pub struct Block {
  pos: IVec2,
}

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
  app.add_system(move_block);
}

fn move_block(mut query: Query<(&mut Transform, &mut Block)>) {
  for (mut transform, mut block) in &mut query {
    block.pos.y -= 1;
    transform.translation.y = block.pos.y as f32 * 0.5;
  }
}

fn startup(mut commands: Commands) {
  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("98c379").unwrap(),
        custom_size: Some(Vec2::new(44.0, 44.0)),
        ..default()
      },
      transform: Transform::from_xyz(100.0, 400.0, 0.0),
      ..default()
    },
    Block {
      pos: IVec2::new(5, 10),
    },
  ));
}

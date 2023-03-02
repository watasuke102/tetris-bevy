use bevy::prelude::*;

#[derive(Component)]
pub struct Block {
  pos: IVec2,
}

#[derive(Resource, Default)]
struct BlockDropTimer(Timer);

const BLOCK_SIZE: f32 = 44.0;

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
  app.add_system(move_block);

  app.add_system(block_drop_timer);
  app.insert_resource(BlockDropTimer(Timer::new(
    std::time::Duration::from_millis(1000),
    TimerMode::Repeating,
  )));
}

fn block_drop_timer(mut timer: ResMut<BlockDropTimer>, time: Res<Time>) {
  timer.0.tick(time.delta());
}

fn move_block(mut query: Query<(&mut Transform, &mut Block)>, timer: Res<BlockDropTimer>) {
  if !timer.0.finished() {
    return;
  }

  for (mut transform, mut block) in &mut query {
    block.pos.y -= 1;
    transform.translation.y = block.pos.y as f32 * BLOCK_SIZE;
  }
}

fn startup(mut commands: Commands) {
  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        color: Color::hex("98c379").unwrap(),
        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
        ..default()
      },
      transform: Transform::from_xyz(100.0, 800.0, 0.0),
      ..default()
    },
    Block {
      pos: IVec2::new(5, 10),
    },
  ));
}

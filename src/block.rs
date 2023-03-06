use bevy::prelude::*;

use crate::field;

#[derive(Component)]
struct Block {
  pos: IVec2,
}

#[derive(Resource, Default)]
struct BlockDropTimer(Timer);

pub fn init(app: &mut App) {
  app.add_startup_system(startup);
  app.add_system(move_block);

  app.insert_resource(BlockDropTimer(Timer::new(
    std::time::Duration::from_millis(1000),
    TimerMode::Repeating,
  )));
}

fn move_block(
  mut query: Query<&mut Block>,
  mut field_query: Query<(&mut Sprite, &mut field::Field)>,
  mut timer: ResMut<BlockDropTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta());
  if !timer.0.finished() {
    return;
  }

  for mut block in &mut query {
    field::unset_block(&mut field_query, block.pos);
    block.pos.y += 1;
    if field::set_block(&mut field_query, block.pos, Color::hex("98c379").unwrap()) {
      block.pos.y = 4;
    }
  }
}

fn startup(mut commands: Commands) {
  commands.spawn(Block {
    pos: IVec2::new(5, 4),
  });
}

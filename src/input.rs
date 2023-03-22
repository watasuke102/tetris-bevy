use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource, Default)]
pub struct KeyRepeat {
  duration:       Duration,
  repeat_timer:   Timer,
  previous:       Option<KeyCode>,
  pub left_move:  bool,
  pub right_move: bool,
}

#[derive(Resource, Default)]
pub struct SoftDropTimer(pub Timer);

impl KeyRepeat {
  fn new(repeat_timer: Timer) -> Self {
    KeyRepeat {
      repeat_timer,
      ..default()
    }
  }
}

pub fn init(app: &mut App) {
  app.add_system(move_horizontal_dir);
  app.add_system(soft_drop);

  let mut drop_timer = Timer::new(Duration::from_millis(50), TimerMode::Repeating);
  drop_timer.pause();
  app.insert_resource(SoftDropTimer(drop_timer));

  let mut repeat_timer = Timer::new(Duration::from_millis(50), TimerMode::Repeating);
  repeat_timer.pause();
  app.insert_resource(KeyRepeat::new(repeat_timer));
}

fn move_horizontal_dir(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut keyrepeat: ResMut<KeyRepeat>,
) {
  let mut pressed = false;
  let mut movable_pos: Option<KeyCode> = None;
  for code in [KeyCode::Left, KeyCode::Right] {
    if !key_input.pressed(code) {
      continue;
    }
    pressed = true;
    match keyrepeat.previous {
      Some(prev_code) => {
        if code == prev_code {
          keyrepeat.duration += time.delta();
          if keyrepeat.duration > Duration::from_millis(150) {
            if keyrepeat.repeat_timer.paused() {
              keyrepeat.repeat_timer.reset();
              keyrepeat.repeat_timer.unpause();
            }
            keyrepeat.repeat_timer.tick(time.delta());
          }
        } else {
          keyrepeat.repeat_timer.pause();
          keyrepeat.duration = time.delta();
        }
      }
      None => keyrepeat.duration = time.delta(),
    }
    keyrepeat.previous = Some(code);
    if keyrepeat.duration == time.delta() ||
      (keyrepeat.duration > Duration::from_millis(200) && keyrepeat.repeat_timer.finished())
    {
      movable_pos = Some(code);
    }
    break;
  }

  keyrepeat.right_move = false;
  keyrepeat.left_move = false;
  if let Some(code) = movable_pos {
    match code {
      KeyCode::Right => keyrepeat.right_move = true,
      KeyCode::Left => keyrepeat.left_move = true,
      _ => (),
    }
  }
  if !pressed {
    keyrepeat.repeat_timer.pause();
    keyrepeat.previous = None;
  }
}

fn soft_drop(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut drop_timer: ResMut<SoftDropTimer>,
) {
  if !key_input.pressed(KeyCode::Down) {
    drop_timer.0.pause();
    return;
  }
  if drop_timer.0.paused() {
    drop_timer.0.reset();
    drop_timer.0.unpause();
  }
  drop_timer.0.tick(time.delta());
}

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

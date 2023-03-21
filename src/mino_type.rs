use bevy::prelude::IVec2;


pub type MinoBlocks = [IVec2; 4];
pub struct MinoType {
  // 1(-1, -1)  3(0, -1)  5(1, -1)
  // 2(-1,  0)  4(0,  0)  6(1,  0)
  pub blocks: MinoBlocks,
  pub color:  &'static str,
}

pub const MINO_TYPES: [MinoType; 7] = [
  // I
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
      IVec2 { x: 2, y: 0 },
    ],
    color:  "56b6c2",
  },
  // J
  MinoType {
    blocks: [
      IVec2 { x: -1, y: -1 },
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "61afef",
  },
  // L
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
      IVec2 { x: 1, y: -1 },
    ],
    color:  "d69363",
  },
  // S
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 0, y: 1 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "98c379",
  },
  // Z
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 0, y: 1 },
      IVec2 { x: 1, y: 1 },
    ],
    color:  "e06c75",
  },
  // O
  MinoType {
    blocks: [
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: -1 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "e5c07b",
  },
  // T
  MinoType {
    blocks: [
      IVec2 { x: -1, y: 0 },
      IVec2 { x: 0, y: -1 },
      IVec2 { x: 0, y: 0 },
      IVec2 { x: 1, y: 0 },
    ],
    color:  "c678dd",
  },
];

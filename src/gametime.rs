//! Methods and wrappers for game time.
use unity::prelude::*;

/// Returns the current game time. Scales for slowdowns/speedups in the game.
/// See [https://docs.unity3d.com/ScriptReference/Time-time.html] for details.
#[unity::from_offset("App", "GameTime", "get_Time")]
pub fn get_time() -> f32;

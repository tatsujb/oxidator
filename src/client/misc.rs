use super::client::*;
use crate::*;
use imgui::*;
use na::{Isometry3, Matrix4, Point3, Vector2, Vector3, Vector4};
use std::collections::{HashMap, HashSet};

use utils::time;

impl App {
    pub fn clear_from_play(&mut self) {
        self.game_state.players.clear();
        self.game_state.my_player_id = None;
        self.game_state.kbots.clear();
        self.game_state.selected.clear();
        self.game_state.kinematic_projectiles.clear();
        self.kbot_gpu.update_instance(&[], &self.gpu.device);
    }
}
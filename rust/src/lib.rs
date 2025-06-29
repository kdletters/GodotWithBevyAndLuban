use crate::data::*;
use bevy::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::classes::{Engine, FileAccess};
use godot::prelude::*;
use godot_bevy::prelude::*;

mod card;
mod data;

#[bevy_app]
fn build_app(app: &mut App) {
}

struct MyExtension;

unsafe impl ExtensionLibrary for MyExtension {}

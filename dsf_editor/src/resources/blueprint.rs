use crate::resources::{EditorData, LevelEdit};
use dsf_core::components::Pos;
use dsf_core::resources::Tile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains a tile map. Is a blueprint for a structure of tiles inside a level.
/// If you copy a selection in the level editor, that selection is stored as a Blueprint.
/// Blueprints can be pasted. Blueprints can potentially be imported and exported from the editor.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Blueprint {
    pub dimensions: Pos,
    pub tiles: HashMap<Pos, Tile>,
}

impl Blueprint {
    pub fn new(dimensions: Pos) -> Self {
        Blueprint {
            dimensions,
            tiles: HashMap::default(),
        }
    }

    pub fn from_placing_tiles(editor_data: &EditorData, level_edit: &LevelEdit) -> Self {
        let key = editor_data.brush.get_key().as_ref();
        let tile_def = key.map(|key| level_edit.tile_map.tile_defs.get(key));
        let brush_dimens = tile_def
            .map(|def| def.dimens)
            .unwrap_or_else(|| Pos::new(1, 1));
        let selection_dimens = (*editor_data).selection.dimens();
        let mut blueprint = Blueprint::new(selection_dimens);
        for x in (0..(selection_dimens.x)).step_by(brush_dimens.x as usize) {
            for y in (0..(selection_dimens.y)).step_by(brush_dimens.y as usize) {
                if let Some(key) = key {
                    blueprint.insert_tile(
                        Pos::new(x, y),
                        &brush_dimens,
                        Tile::TileDefKey(key.clone()),
                    );
                } else {
                    blueprint.tiles.insert(Pos::new(x, y), Tile::AirBlock);
                }
            }
        }
        blueprint
    }

    fn insert_tile(&mut self, pos: Pos, dimens: &Pos, tile: Tile) {
        self.tiles.insert(pos, tile);
        for x in pos.x..(pos.x + dimens.x) {
            for y in pos.y..(pos.y + dimens.y) {
                if x != pos.x || y != pos.y {
                    self.tiles.insert(Pos::new(x, y), Tile::Dummy(pos));
                }
            }
        }
    }

    /// Returns true iff the given rectangle overlaps with any of the tiles in the blueprint.
    /// The given position must be relative to the blueprint.
    pub fn overlaps(&self, pos: Pos, dimens: Pos) -> bool {
        (pos.x..(pos.x + dimens.x))
            .any(|x| (pos.y..(pos.y + dimens.y)).any(|y| self.tiles.get(&Pos::new(x, y)).is_some()))
    }
}

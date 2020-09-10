use crate::resources::{DeprecatedEditorData, DeprecatedLevelEdit};
use amethyst::config::ConfigError;
use amethyst::prelude::{Config, World, WorldExt};

use dsf_core::levels::LevelSave;
use dsf_core::utility::files::get_levels_dir;
use std::path::PathBuf;

/// Returns a PathBuf to the file that is used to store auto saves.
pub fn auto_save_file() -> PathBuf {
    get_levels_dir().join("auto_save.ron")
}

/// Load and return the auto save level.
/// If there is no auto save file to load from, the default implementation will be used.
pub fn load_auto_save() -> Result<DeprecatedLevelEdit, ConfigError> {
    let level_file = auto_save_file();
    if level_file.exists() {
        read_level_file(level_file)
    } else {
        Ok(DeprecatedLevelEdit::default())
    }
}

/// Load and return the level with the given name.
#[allow(dead_code)] //Not used yet, but will be used in the future.
pub fn load(name: String) -> Result<DeprecatedLevelEdit, ConfigError> {
    let level_file = get_levels_dir().join(name + ".ron");
    read_level_file(level_file)
}

fn read_level_file(level_file: PathBuf) -> Result<DeprecatedLevelEdit, ConfigError> {
    let level = LevelSave::load(level_file)?;
    Ok(level.into())
}

/// Write the current state of the LevelEdit to the auto save file, overwriting what is already
/// there.
pub fn auto_save(world: &mut World) -> Result<(), ConfigError> {
    write_level_file(auto_save_file(), world)
}

/// Store the current state of the LevelEdit to file. The given name will be used as a filename.
/// TODO: check if name is reserved (ie: auto_save)
/// TODO: check if level already exists, if so maybe ask to overwrite?
///     (or keep track of which one we loaded, so we know whether it's safe to overwrite)
#[allow(dead_code)] //Not used yet, but will be used in the future.
pub fn save(name: String, world: &mut World) -> Result<(), ConfigError> {
    let level_file = get_levels_dir().join(name + ".ron");
    write_level_file(level_file, world)
}

fn write_level_file(level_file: PathBuf, world: &mut World) -> Result<(), ConfigError> {
    let data = world.write_resource::<DeprecatedEditorData>();
    let level: LevelSave = (*data).level.clone().into();
    level.write(level_file)
}

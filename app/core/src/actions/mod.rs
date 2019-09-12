use crate::maps::World;
use crate::theme::ThemeMode;

pub enum Actions {
    SettingsSetThemeMode(ThemeMode),
    SettingsSetScale(i32),
    GameSetElapsedTime(f64),
    GameSetWorld(World),
}

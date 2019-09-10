use crate::maps::World;
use crate::theme::ThemeMode;

pub enum Actions {
    SettingsSetThemeMode(ThemeMode),
    GameSetWorld(World),
}

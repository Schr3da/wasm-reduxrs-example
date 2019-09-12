use crate::maps::World;
use crate::theme::ThemeMode;

pub enum Actions {
    SettingsSetThemeMode(ThemeMode),
    GameSetElapsedTime(f64),
    GameSetWorld(World),
    

}

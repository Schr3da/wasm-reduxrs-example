use crate::maps::{Map};
use crate::theme::{ThemeMode};

pub enum Actions { 
    SettingsSetThemeMode(ThemeMode),
    GameSetMap(Map),
}

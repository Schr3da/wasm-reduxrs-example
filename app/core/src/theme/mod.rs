#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ThemeMode {
    LIGHT,
    DARK,
}

pub trait Theme {
    fn mode(&self) -> ThemeMode;

    fn color_forground(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#2a2c33",
            ThemeMode::DARK => "#c5c8c6",
        }
    }

    fn color_background(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#f9f9f9",
            ThemeMode::DARK => "#161719",
        }
    }

    fn color_1(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#000000",
            ThemeMode::DARK => "#000000",
        }
    }

    fn color_2(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#de3e35",
            ThemeMode::DARK => "#fd5ff1",
        }
    }

    fn color_3(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#de3e35",
            ThemeMode::DARK => "#87c38a",
        }
    }

    fn color_4(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#3f953a",
            ThemeMode::DARK => "#ffd7b1",
        }
    }

    fn color_5(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#d2b67c",
            ThemeMode::DARK => "#85befd",
        }
    }

    fn color_6(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#2f5af3",
            ThemeMode::DARK => "#b9b6fc",
        }
    }

    fn color_7(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#950095",
            ThemeMode::DARK => "#85befd",
        }
    }

    fn color_8(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#3f953a",
            ThemeMode::DARK => "#e0e0e0",
        }
    }

    fn color_9(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#bbbbbb",
            ThemeMode::DARK => "#000000",
        }
    }

    fn color_10(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#000000",
            ThemeMode::DARK => "#fd5ff1",
        }
    }

    fn color_11(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#de3e35",
            ThemeMode::DARK => "#94fa36",
        }
    }

    fn color_12(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#3f953a",
            ThemeMode::DARK => "#f5ffa8",
        }
    }

    fn color_13(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#d2b67c",
            ThemeMode::DARK => "#96cbfe",
        }
    }

    fn color_14(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#2f5af3",
            ThemeMode::DARK => "#b9b6fc",
        }
    }

    fn color_15(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#3f953a",
            ThemeMode::DARK => "#85befd",
        }
    }

    fn color_16(&self) -> &'static str {
        match self.mode() {
            ThemeMode::LIGHT => "#ffffff",
            ThemeMode::DARK => "#e0e0e0",
        }
    }
}

pub struct LightTheme {}
impl Theme for LightTheme {
    fn mode(&self) -> ThemeMode {
        ThemeMode::LIGHT
    }
}

pub struct DarkTheme {}
impl Theme for DarkTheme {
    fn mode(&self) -> ThemeMode {
        ThemeMode::DARK
    }
}

pub fn new_theme<T: Theme>(mode: ThemeMode) -> Box<dyn Theme> {
    match mode {
        ThemeMode::LIGHT => Box::new(LightTheme{}),
        ThemeMode::DARK => Box::new(DarkTheme{}),
    }
}



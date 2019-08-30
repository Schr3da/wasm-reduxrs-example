pub trait Theme {
    fn color_forground() -> &'static str;
    fn color_background() -> &'static str;
    fn color_1() -> &'static str;
    fn color_2() -> &'static str;
    fn color_3() -> &'static str;
    fn color_4() -> &'static str;
    fn color_5() -> &'static str;
    fn color_6() -> &'static str;
    fn color_7() -> &'static str;
    fn color_8() -> &'static str;
    fn color_9() -> &'static str;
    fn color_10() -> &'static str;
    fn color_11() -> &'static str;
    fn color_12() -> &'static str;
    fn color_13() -> &'static str;
    fn color_14() -> &'static str;
    fn color_15() -> &'static str;
    fn color_16() -> &'static str;
}

pub struct LightTheme;

impl Theme for LightTheme {

    fn color_forground() -> &'static str {
        "#2a2c33"
    }

    fn color_background() -> &'static str {
        "#f9f9f9"
    }

    fn color_1() -> &'static str {
        "#000000"
    }

    fn color_2() -> &'static str {
        "#de3e35"
    }

    fn color_3() -> &'static str {
        "#de3e35"
    }

    fn color_4() -> &'static str {
        "#3f953a"
    }

    fn color_5() -> &'static str {
        "#d2b67c"
    }

    fn color_6() -> &'static str {
        "#2f5af3"
    }

    fn color_7() -> &'static str {
        "#950095"
    }

    fn color_8() -> &'static str {
        "#3f953a"
    }

    fn color_9() -> &'static str {
        "#bbbbbb"
    }

    fn color_10() -> &'static str {
        "#000000"
    }

    fn color_11() -> &'static str {
        "#de3e35"
    }

    fn color_12() -> &'static str {
        "#3f953a"
    }

    fn color_13() -> &'static str {
        "#d2b67c"
    }

    fn color_14() -> &'static str {
        "#2f5af3"
    }

    fn color_15() -> &'static str {
        "#3f953a"
    }

    fn color_16() -> &'static str {
        "#ffffff"
    }
}

pub struct DarkTheme;

impl Theme for DarkTheme {

    fn color_forground() -> &'static str {
        "#c5c8c6"
    }

    fn color_background() -> &'static str {
        "#161719"
    }

    fn color_1() -> &'static str {
        "#000000"
    }

    fn color_2() -> &'static str {
        "#fd5ff1"
    }

    fn color_3() -> &'static str {
        "#87c38a"
    }

    fn color_4() -> &'static str {
        "#ffd7b1"
    }

    fn color_5() -> &'static str {
        "#85befd"
    }

    fn color_6() -> &'static str {
        "#b9b6fc"
    }

    fn color_7() -> &'static str {
        "#85befd"
    }

    fn color_8() -> &'static str {
        "#e0e0e0"
    }

    fn color_9() -> &'static str {
        "#000000"
    }

    fn color_10() -> &'static str {
        "#fd5ff1"
    }

    fn color_11() -> &'static str {
        "#94fa36"
    }

    fn color_12() -> &'static str {
        "#f5ffa8"
    }

    fn color_13() -> &'static str {
        "#96cbfe"
    }

    fn color_14() -> &'static str {
        "#b9b6fc"
    }

    fn color_15() -> &'static str {
        "#85befd"
    }

    fn color_16() -> &'static str {
        "#e0e0e0"
    }

}

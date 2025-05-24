//! Key Event Types



#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyCode(pub u8);

impl From<u8> for KeyCode { fn from(value: u8) -> Self { Self(value) } }
impl Into<u8> for KeyCode { fn into(self) -> u8 { self.0 } }

macro_rules! define_keycodes {
    ($($name:ident $val:literal,)*) => {
        $(pub const $name: crate::key::KeyCode = crate::key::KeyCode($val);)*
    }
}

// Alphanumeric constants.
impl KeyCode {
    define_keycodes!(
        AN_0 0,
        AN_1 1,
        AN_2 2,
        AN_3 3,
        AN_4 4,
        AN_5 5,
        AN_6 6,
        AN_7 7,
        AN_8 8,
        AN_9 9,

        AN_A 10,
        AN_B 11,
        AN_C 12,
        AN_D 13,
        AN_E 14,
        AN_F 15,
        AN_G 16,
        AN_H 17,
        AN_I 18,
        AN_J 19,
        AN_K 20,
        AN_L 21,
        AN_M 22,
        AN_N 23,
        AN_O 24,
        AN_P 25,
        AN_Q 26,
        AN_R 27,
        AN_S 28,
        AN_T 29,
        AN_U 30,
        AN_V 31,
        AN_W 32,
        AN_X 33,
        AN_Y 34,
        AN_Z 35,
    );
}

// Punctuation constants.
impl KeyCode {
    define_keycodes!(
        AN_TILDE 36,
        AN_MINUS 37,
        AN_EQUAL 38,
        AN_LBRACKET 39,
        AN_RBRACKET 40,
        AN_BACKSLASH 41,
        AN_SEMICOLON 42,
        AN_APOSTROPHE 43,
        AN_COMMA 44,
        AN_DOT 45,
        AN_SLASH 46,
    );
}

// Control constants.
impl KeyCode {
    define_keycodes!(
        C_LCTRL 47,
        C_RCTRL 48,
        C_LSHIFT 49,
        C_RSHIFT 50,
        C_LALT 51,
        C_RALT 52,
        C_LMETA 53,
        C_RMETA 54,

        C_SPACE 55,
        C_BACKSPACE 56,
        C_TAB 57,
        C_ENTER 58,
        C_ESCAPE 59,
        C_MENU 60,

        C_INSERT 61,
        C_DELETE 62,
        C_HOME 63,
        C_END 64,
        C_PAGEUP 65,
        C_PAGEDOWN 66,
        C_ARROWUP 67,
        C_ARROWDOWN 68,
        C_ARROWLEFT 69,
        C_ARROWRIGHT 70,
    );
}

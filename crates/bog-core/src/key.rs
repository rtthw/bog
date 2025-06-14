//! Key event types



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyUpdate {
    Down {
        code: KeyCode,
        repeat: bool,
    },
    Up {
        code: KeyCode,
    },
}


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

// Utilities.
impl KeyCode {
    /// Convert this keycode into a `char`, if possible.
    pub fn to_char(&self, shifted: bool) -> Option<char> {
        Some(match (*self, shifted) {
            (KeyCode::AN_0, false) => '0',
            (KeyCode::AN_0, true) => ')',
            (KeyCode::AN_1, false) => '1',
            (KeyCode::AN_1, true) => '!',
            (KeyCode::AN_2, false) => '2',
            (KeyCode::AN_2, true) => '@',
            (KeyCode::AN_3, false) => '3',
            (KeyCode::AN_3, true) => '#',
            (KeyCode::AN_4, false) => '4',
            (KeyCode::AN_4, true) => '$',
            (KeyCode::AN_5, false) => '5',
            (KeyCode::AN_5, true) => '%',
            (KeyCode::AN_6, false) => '6',
            (KeyCode::AN_6, true) => '^',
            (KeyCode::AN_7, false) => '7',
            (KeyCode::AN_7, true) => '&',
            (KeyCode::AN_8, false) => '8',
            (KeyCode::AN_8, true) => '*',
            (KeyCode::AN_9, false) => '9',
            (KeyCode::AN_9, true) => '(',

            (KeyCode::AN_A, false) => 'a',
            (KeyCode::AN_A, true) => 'A',
            (KeyCode::AN_B, false) => 'b',
            (KeyCode::AN_B, true) => 'B',
            (KeyCode::AN_C, false) => 'c',
            (KeyCode::AN_C, true) => 'C',
            (KeyCode::AN_D, false) => 'd',
            (KeyCode::AN_D, true) => 'D',
            (KeyCode::AN_E, false) => 'e',
            (KeyCode::AN_E, true) => 'E',
            (KeyCode::AN_F, false) => 'f',
            (KeyCode::AN_F, true) => 'F',
            (KeyCode::AN_G, false) => 'g',
            (KeyCode::AN_G, true) => 'G',
            (KeyCode::AN_H, false) => 'h',
            (KeyCode::AN_H, true) => 'H',
            (KeyCode::AN_I, false) => 'i',
            (KeyCode::AN_I, true) => 'I',
            (KeyCode::AN_J, false) => 'i',
            (KeyCode::AN_J, true) => 'J',
            (KeyCode::AN_K, false) => 'k',
            (KeyCode::AN_K, true) => 'K',
            (KeyCode::AN_L, false) => 'l',
            (KeyCode::AN_L, true) => 'L',
            (KeyCode::AN_M, false) => 'm',
            (KeyCode::AN_M, true) => 'M',
            (KeyCode::AN_N, false) => 'n',
            (KeyCode::AN_N, true) => 'N',
            (KeyCode::AN_O, false) => 'o',
            (KeyCode::AN_O, true) => 'O',
            (KeyCode::AN_P, false) => 'p',
            (KeyCode::AN_P, true) => 'P',
            (KeyCode::AN_Q, false) => 'q',
            (KeyCode::AN_Q, true) => 'Q',
            (KeyCode::AN_R, false) => 'r',
            (KeyCode::AN_R, true) => 'R',
            (KeyCode::AN_S, false) => 's',
            (KeyCode::AN_S, true) => 'S',
            (KeyCode::AN_T, false) => 't',
            (KeyCode::AN_T, true) => 'T',
            (KeyCode::AN_U, false) => 'u',
            (KeyCode::AN_U, true) => 'U',
            (KeyCode::AN_V, false) => 'v',
            (KeyCode::AN_V, true) => 'V',
            (KeyCode::AN_W, false) => 'w',
            (KeyCode::AN_W, true) => 'W',
            (KeyCode::AN_X, false) => 'x',
            (KeyCode::AN_X, true) => 'X',
            (KeyCode::AN_Y, false) => 'y',
            (KeyCode::AN_Y, true) => 'Y',
            (KeyCode::AN_Z, false) => 'z',
            (KeyCode::AN_Z, true) => 'Z',

            (KeyCode::AN_TILDE, false) => '`',
            (KeyCode::AN_MINUS, false) => '-',
            (KeyCode::AN_EQUAL, false) => '=',
            (KeyCode::AN_LBRACKET, false) => '[',
            (KeyCode::AN_RBRACKET, false) => ']',
            (KeyCode::AN_BACKSLASH, false) => '\\',
            (KeyCode::AN_SEMICOLON, false) => ';',
            (KeyCode::AN_APOSTROPHE, false) => '\'',
            (KeyCode::AN_COMMA, false) => ',',
            (KeyCode::AN_DOT, false) => '.',
            (KeyCode::AN_SLASH, false) => '/',

            (KeyCode::AN_TILDE, true) => '~',
            (KeyCode::AN_MINUS, true) => '_',
            (KeyCode::AN_EQUAL, true) => '+',
            (KeyCode::AN_LBRACKET, true) => '{',
            (KeyCode::AN_RBRACKET, true) => '}',
            (KeyCode::AN_BACKSLASH, true) => '|',
            (KeyCode::AN_SEMICOLON, true) => ':',
            (KeyCode::AN_APOSTROPHE, true) => '"',
            (KeyCode::AN_COMMA, true) => '<',
            (KeyCode::AN_DOT, true) => '>',
            (KeyCode::AN_SLASH, true) => '/',

            (KeyCode::C_SPACE, _) => ' ',
            (KeyCode::C_TAB, _) => '\t',
            (KeyCode::C_ENTER, _) => '\n',

            _ => None?,
        })
    }

    #[inline]
    pub const fn is_control(&self) -> bool {
        matches!(self, &KeyCode::C_LCTRL | &KeyCode::C_RCTRL)
    }

    #[inline]
    pub const fn is_shift(&self) -> bool {
        matches!(self, &KeyCode::C_LSHIFT | &KeyCode::C_RSHIFT)
    }

    #[inline]
    pub const fn is_alt(&self) -> bool {
        matches!(self, &KeyCode::C_LALT | &KeyCode::C_RALT)
    }

    #[inline]
    pub const fn is_super(&self) -> bool {
        matches!(self, &KeyCode::C_LMETA | &KeyCode::C_RMETA)
    }
}



/// A virtual, more easily understandable version of a [`KeyCode`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Char(char),

    Control,
    Shift,
    Alt,
    Super,

    Unknown,
}

impl From<(KeyCode, bool)> for Key {
    fn from((code, shifted): (KeyCode, bool)) -> Self {
        if let Some(ch) = code.to_char(shifted) {
            Self::Char(ch)
        } else if code.is_control() {
            Self::Control
        } else if code.is_shift() {
            Self::Shift
        } else if code.is_alt() {
            Self::Alt
        } else if code.is_super() {
            Self::Super
        } else {
            Self::Unknown
        }
    }
}

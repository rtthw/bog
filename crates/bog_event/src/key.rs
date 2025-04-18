//! Key Event Types



#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyCode(pub u8);

impl From<u8> for KeyCode { fn from(value: u8) -> Self { Self(value) } }
impl Into<u8> for KeyCode { fn into(self) -> u8 { self.0 } }

macro_rules! keycode {
    ($name:ident $val:literal) => {
        pub const $name: crate::key::KeyCode = crate::key::KeyCode($val);
    };
}

// Alphanumeric constants.
impl KeyCode {
    keycode!(AN_0 0);
    keycode!(AN_1 1);
    keycode!(AN_2 2);
    keycode!(AN_3 3);
    keycode!(AN_4 4);
    keycode!(AN_5 5);
    keycode!(AN_6 6);
    keycode!(AN_7 7);
    keycode!(AN_8 8);
    keycode!(AN_9 9);

    keycode!(AN_A 10);
    keycode!(AN_B 11);
    keycode!(AN_C 12);
    keycode!(AN_D 13);
    keycode!(AN_E 14);
    keycode!(AN_F 15);
    keycode!(AN_G 16);
    keycode!(AN_H 17);
    keycode!(AN_I 18);
    keycode!(AN_J 19);
    keycode!(AN_K 20);
    keycode!(AN_L 21);
    keycode!(AN_M 22);
    keycode!(AN_N 23);
    keycode!(AN_O 24);
    keycode!(AN_P 25);
    keycode!(AN_Q 26);
    keycode!(AN_R 27);
    keycode!(AN_S 28);
    keycode!(AN_T 29);
    keycode!(AN_U 30);
    keycode!(AN_V 31);
    keycode!(AN_W 32);
    keycode!(AN_X 33);
    keycode!(AN_Y 34);
    keycode!(AN_Z 35);
}

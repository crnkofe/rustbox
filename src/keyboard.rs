#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

/**
 * Shift is incorrectly handled for the moment
 */
impl Modifiers {
    pub fn new() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
        }
    }

    pub fn new_all(ctrl: bool, shift:bool, alt:bool) -> Self {
        Self {
            ctrl: ctrl,
            shift: shift,
            alt: alt,
        }
    }

    pub fn shift_down(&self) -> bool {
        return self.shift;
    }

    pub fn ctrl_down(&self) -> bool {
        return self.ctrl;
    }

    pub fn alt_down(&self) -> bool {
        return self.alt;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Up,
    Down,
    Delete,
    Insert,

    Home,
    End,
    PageUp,
    PageDown,

    Char(char),
    F(u32),
    Unknown(u16),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExtendedKey {
    // ctrl, shift, alt modifiers
    pub modifiers: Modifiers,
    // actual pressed key
    pub key: Key,

    pub raw_emod: u8,
    pub raw_ch: u32,
    pub raw_key: u16,
}

impl ExtendedKey {

    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self {
            key: key,
            modifiers: modifiers,
            raw_emod: 0,
            raw_ch: 0,
            raw_key: 0,
        }
    }

    pub fn from_code(code: u16) -> Option<ExtendedKey> {
        let empty_modifiers = Modifiers::new();
        let ctrl = Modifiers{ctrl:true, ..empty_modifiers};
        match code {
            termbox::TB_KEY_CTRL_A => Some(ExtendedKey::new(Key::Char('a'), ctrl)),
            termbox::TB_KEY_CTRL_B => Some(ExtendedKey::new(Key::Char('b'), ctrl)),
            termbox::TB_KEY_CTRL_C => Some(ExtendedKey::new(Key::Char('c'), ctrl)),
            termbox::TB_KEY_CTRL_D => Some(ExtendedKey::new(Key::Char('d'), ctrl)),
            termbox::TB_KEY_CTRL_E => Some(ExtendedKey::new(Key::Char('e'), ctrl)),
            termbox::TB_KEY_CTRL_F => Some(ExtendedKey::new(Key::Char('f'), ctrl)),
            termbox::TB_KEY_BACKSPACE => Some(ExtendedKey::new(Key::Backspace, empty_modifiers)),
            termbox::TB_KEY_TAB => Some(ExtendedKey::new(Key::Tab, empty_modifiers)),
            termbox::TB_KEY_CTRL_J => Some(ExtendedKey::new(Key::Char('j'), ctrl)),
            termbox::TB_KEY_CTRL_K => Some(ExtendedKey::new(Key::Char('k'), ctrl)),
            termbox::TB_KEY_CTRL_L => Some(ExtendedKey::new(Key::Char('l'), ctrl)),
            termbox::TB_KEY_ENTER => Some(ExtendedKey::new(Key::Enter, empty_modifiers)),
            termbox::TB_KEY_CTRL_N => Some(ExtendedKey::new(Key::Char('n'), ctrl)),
            termbox::TB_KEY_CTRL_O => Some(ExtendedKey::new(Key::Char('o'), ctrl)),
            termbox::TB_KEY_CTRL_P => Some(ExtendedKey::new(Key::Char('p'), ctrl)),
            termbox::TB_KEY_CTRL_Q => Some(ExtendedKey::new(Key::Char('q'), ctrl)),
            termbox::TB_KEY_CTRL_R => Some(ExtendedKey::new(Key::Char('r'), ctrl)),
            termbox::TB_KEY_CTRL_S => Some(ExtendedKey::new(Key::Char('s'), ctrl)),
            termbox::TB_KEY_CTRL_T => Some(ExtendedKey::new(Key::Char('t'), ctrl)),
            termbox::TB_KEY_CTRL_U => Some(ExtendedKey::new(Key::Char('u'), ctrl)),
            termbox::TB_KEY_CTRL_V => Some(ExtendedKey::new(Key::Char('v'), ctrl)),
            termbox::TB_KEY_CTRL_W => Some(ExtendedKey::new(Key::Char('w'), ctrl)),
            termbox::TB_KEY_CTRL_X => Some(ExtendedKey::new(Key::Char('x'), ctrl)),
            termbox::TB_KEY_CTRL_Y => Some(ExtendedKey::new(Key::Char('y'), ctrl)),
            termbox::TB_KEY_CTRL_Z => Some(ExtendedKey::new(Key::Char('z'), ctrl)),
            termbox::TB_KEY_ESC => Some(ExtendedKey::new(Key::Esc, empty_modifiers)),
            termbox::TB_KEY_CTRL_BACKSLASH => Some(ExtendedKey::new(Key::Char('\\'), ctrl)),
            termbox::TB_KEY_CTRL_RSQ_BRACKET => Some(ExtendedKey::new(Key::Char(']'), ctrl)),
            termbox::TB_KEY_CTRL_6 => Some(ExtendedKey::new(Key::Char('6'), ctrl)),
            termbox::TB_KEY_CTRL_7 => Some(ExtendedKey::new(Key::Char('7'), ctrl)),
            termbox::TB_KEY_CTRL_SLASH => Some(ExtendedKey::new(Key::Char('/'), ctrl)),
            termbox::TB_KEY_CTRL_UNDERSCORE => Some(ExtendedKey::new(Key::Char('_'), ctrl)),
            termbox::TB_KEY_SPACE => Some(ExtendedKey::new(Key::Char(' '), empty_modifiers)),
            termbox::TB_KEY_BACKSPACE => Some(ExtendedKey::new(Key::Backspace, empty_modifiers)),
            termbox::TB_KEY_ARROW_RIGHT => Some(ExtendedKey::new(Key::Right, empty_modifiers)),
            termbox::TB_KEY_ARROW_LEFT => Some(ExtendedKey::new(Key::Left, empty_modifiers)),
            termbox::TB_KEY_ARROW_DOWN => Some(ExtendedKey::new(Key::Down, empty_modifiers)),
            termbox::TB_KEY_ARROW_UP => Some(ExtendedKey::new(Key::Up, empty_modifiers)),
            termbox::TB_KEY_F1 => Some(ExtendedKey::new(Key::F(1), empty_modifiers)),
            termbox::TB_KEY_F2 => Some(ExtendedKey::new(Key::F(2), empty_modifiers)),
            termbox::TB_KEY_F3 => Some(ExtendedKey::new(Key::F(3), empty_modifiers)),
            termbox::TB_KEY_F4 => Some(ExtendedKey::new(Key::F(4), empty_modifiers)),
            termbox::TB_KEY_F5 => Some(ExtendedKey::new(Key::F(5), empty_modifiers)),
            termbox::TB_KEY_F6 => Some(ExtendedKey::new(Key::F(6), empty_modifiers)),
            termbox::TB_KEY_F7 => Some(ExtendedKey::new(Key::F(7), empty_modifiers)),
            termbox::TB_KEY_F8 => Some(ExtendedKey::new(Key::F(8), empty_modifiers)),
            termbox::TB_KEY_F9 => Some(ExtendedKey::new(Key::F(9), empty_modifiers)),
            termbox::TB_KEY_F10 => Some(ExtendedKey::new(Key::F(10), empty_modifiers)),
            termbox::TB_KEY_F11 => Some(ExtendedKey::new(Key::F(11), empty_modifiers)),
            termbox::TB_KEY_F12 => Some(ExtendedKey::new(Key::F(12), empty_modifiers)),
            termbox::TB_KEY_INSERT => Some(ExtendedKey::new(Key::Insert, empty_modifiers)),
            termbox::TB_KEY_DELETE => Some(ExtendedKey::new(Key::Delete, empty_modifiers)),
            termbox::TB_KEY_HOME => Some(ExtendedKey::new(Key::Home, empty_modifiers)),
            termbox::TB_KEY_END => Some(ExtendedKey::new(Key::End, empty_modifiers)),
            termbox::TB_KEY_PGUP => Some(ExtendedKey::new(Key::PageUp, empty_modifiers)),
            termbox::TB_KEY_PGDN => Some(ExtendedKey::new(Key::PageDown, empty_modifiers)),
            _     => None,
        }
    }
}

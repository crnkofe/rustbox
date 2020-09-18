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
pub enum PressedKey {
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
pub struct Key {
    // ctrl, shift, alt modifiers
    pub modifiers: Modifiers,
    // actual pressed key
    pub key: PressedKey,

    pub raw_emod: u8,
    pub raw_ch: u32,
    pub raw_key: u16,
}

impl Key {

    pub fn new(key: PressedKey, modifiers: Modifiers) -> Self {
        Self {
            key: key,
            modifiers: modifiers,
            raw_emod: 0,
            raw_ch: 0,
            raw_key: 0,
        }
    }

    pub fn from_code(code: u16) -> Option<Key> {
        let empty_modifiers = Modifiers::new();
        let ctrl = Modifiers{ctrl:true, ..empty_modifiers};
        match code {
            termbox::TB_KEY_CTRL_A => Some(Key::new(PressedKey::Char('a'), ctrl)),
            termbox::TB_KEY_CTRL_B => Some(Key::new(PressedKey::Char('b'), ctrl)),
            termbox::TB_KEY_CTRL_C => Some(Key::new(PressedKey::Char('c'), ctrl)),
            termbox::TB_KEY_CTRL_D => Some(Key::new(PressedKey::Char('d'), ctrl)),
            termbox::TB_KEY_CTRL_E => Some(Key::new(PressedKey::Char('e'), ctrl)),
            termbox::TB_KEY_CTRL_F => Some(Key::new(PressedKey::Char('f'), ctrl)),
            termbox::TB_KEY_BACKSPACE => Some(Key::new(PressedKey::Backspace, empty_modifiers)),
            termbox::TB_KEY_TAB => Some(Key::new(PressedKey::Tab, empty_modifiers)),
            termbox::TB_KEY_CTRL_J => Some(Key::new(PressedKey::Char('j'), ctrl)),
            termbox::TB_KEY_CTRL_K => Some(Key::new(PressedKey::Char('k'), ctrl)),
            termbox::TB_KEY_CTRL_L => Some(Key::new(PressedKey::Char('l'), ctrl)),
            termbox::TB_KEY_ENTER => Some(Key::new(PressedKey::Enter, empty_modifiers)),
            termbox::TB_KEY_CTRL_N => Some(Key::new(PressedKey::Char('n'), ctrl)),
            termbox::TB_KEY_CTRL_O => Some(Key::new(PressedKey::Char('o'), ctrl)),
            termbox::TB_KEY_CTRL_P => Some(Key::new(PressedKey::Char('p'), ctrl)),
            termbox::TB_KEY_CTRL_Q => Some(Key::new(PressedKey::Char('q'), ctrl)),
            termbox::TB_KEY_CTRL_R => Some(Key::new(PressedKey::Char('r'), ctrl)),
            termbox::TB_KEY_CTRL_S => Some(Key::new(PressedKey::Char('s'), ctrl)),
            termbox::TB_KEY_CTRL_T => Some(Key::new(PressedKey::Char('t'), ctrl)),
            termbox::TB_KEY_CTRL_U => Some(Key::new(PressedKey::Char('u'), ctrl)),
            termbox::TB_KEY_CTRL_V => Some(Key::new(PressedKey::Char('v'), ctrl)),
            termbox::TB_KEY_CTRL_W => Some(Key::new(PressedKey::Char('w'), ctrl)),
            termbox::TB_KEY_CTRL_X => Some(Key::new(PressedKey::Char('x'), ctrl)),
            termbox::TB_KEY_CTRL_Y => Some(Key::new(PressedKey::Char('y'), ctrl)),
            termbox::TB_KEY_CTRL_Z => Some(Key::new(PressedKey::Char('z'), ctrl)),
            termbox::TB_KEY_ESC => Some(Key::new(PressedKey::Esc, empty_modifiers)),
            termbox::TB_KEY_CTRL_BACKSLASH => Some(Key::new(PressedKey::Char('\\'), ctrl)),
            termbox::TB_KEY_CTRL_RSQ_BRACKET => Some(Key::new(PressedKey::Char(']'), ctrl)),
            termbox::TB_KEY_CTRL_6 => Some(Key::new(PressedKey::Char('6'), ctrl)),
            termbox::TB_KEY_CTRL_7 => Some(Key::new(PressedKey::Char('7'), ctrl)),
            termbox::TB_KEY_CTRL_SLASH => Some(Key::new(PressedKey::Char('/'), ctrl)),
            termbox::TB_KEY_CTRL_UNDERSCORE => Some(Key::new(PressedKey::Char('_'), ctrl)),
            termbox::TB_KEY_SPACE => Some(Key::new(PressedKey::Char(' '), empty_modifiers)),
            termbox::TB_KEY_BACKSPACE => Some(Key::new(PressedKey::Backspace, empty_modifiers)),
            termbox::TB_KEY_ARROW_RIGHT => Some(Key::new(PressedKey::Right, empty_modifiers)),
            termbox::TB_KEY_ARROW_LEFT => Some(Key::new(PressedKey::Left, empty_modifiers)),
            termbox::TB_KEY_ARROW_DOWN => Some(Key::new(PressedKey::Down, empty_modifiers)),
            termbox::TB_KEY_ARROW_UP => Some(Key::new(PressedKey::Up, empty_modifiers)),
            termbox::TB_KEY_F1 => Some(Key::new(PressedKey::F(1), empty_modifiers)),
            termbox::TB_KEY_F2 => Some(Key::new(PressedKey::F(2), empty_modifiers)),
            termbox::TB_KEY_F3 => Some(Key::new(PressedKey::F(3), empty_modifiers)),
            termbox::TB_KEY_F4 => Some(Key::new(PressedKey::F(4), empty_modifiers)),
            termbox::TB_KEY_F5 => Some(Key::new(PressedKey::F(5), empty_modifiers)),
            termbox::TB_KEY_F6 => Some(Key::new(PressedKey::F(6), empty_modifiers)),
            termbox::TB_KEY_F7 => Some(Key::new(PressedKey::F(7), empty_modifiers)),
            termbox::TB_KEY_F8 => Some(Key::new(PressedKey::F(8), empty_modifiers)),
            termbox::TB_KEY_F9 => Some(Key::new(PressedKey::F(9), empty_modifiers)),
            termbox::TB_KEY_F10 => Some(Key::new(PressedKey::F(10), empty_modifiers)),
            termbox::TB_KEY_F11 => Some(Key::new(PressedKey::F(11), empty_modifiers)),
            termbox::TB_KEY_F12 => Some(Key::new(PressedKey::F(12), empty_modifiers)),
            termbox::TB_KEY_INSERT => Some(Key::new(PressedKey::Insert, empty_modifiers)),
            termbox::TB_KEY_DELETE => Some(Key::new(PressedKey::Delete, empty_modifiers)),
            termbox::TB_KEY_HOME => Some(Key::new(PressedKey::Home, empty_modifiers)),
            termbox::TB_KEY_END => Some(Key::new(PressedKey::End, empty_modifiers)),
            termbox::TB_KEY_PGUP => Some(Key::new(PressedKey::PageUp, empty_modifiers)),
            termbox::TB_KEY_PGDN => Some(Key::new(PressedKey::PageDown, empty_modifiers)),
            _     => None,
        }
    }
}

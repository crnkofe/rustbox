extern crate gag;
extern crate num_traits;
extern crate termbox_sys as termbox;
#[macro_use] extern crate bitflags;

pub use self::style::{Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use std::error::Error;
use std::fmt;
use std::io;
use std::char;
use std::default::Default;
use std::ops::FnOnce;
use std::sync::Mutex;

use num_traits::FromPrimitive;
use termbox::RawEvent;
use termbox::RawCell;
use std::os::raw::c_int;
use gag::Hold;
use std::time::Duration;

pub mod keyboard;
pub mod mouse;

pub use keyboard::Modifiers;
pub use keyboard::ExtendedKey;
pub use keyboard::Key;
pub use mouse::Mouse;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    KeyEventRaw(u8, u16, u32),
    KeyEvent(ExtendedKey),
    ResizeEvent(i16, i16),
    MouseEvent(Mouse, i16, i16),
    NoEvent
}

#[derive(Clone, Copy, Debug)]
pub enum InputMode {
    Current = 0x00,

    /// When ESC sequence is in the buffer and it doesn't match any known
    /// ESC sequence => ESC means TB_KEY_ESC
    Esc     = 0x01,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// sequence => ESC enables TB_MOD_ALT modifier for the next keyboard event.
    Alt     = 0x02,
    /// Same as `Esc` but enables mouse events
    EscMouse = 0x05,
    /// Same as `Alt` but enables mouse events
    AltMouse = 0x06
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputMode {
    Normal = 0,
    EightBit = 1,  // 256 Colors
    // TrueColor = 2 ; needs to be compiled in
    NoOutput = 10,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u16),
    Default,
}

impl Color {
    pub fn as_256color(&self) -> u16 {
        match *self {
            Color::Black => 0x10,
            Color::Red => 0x01,
            Color::Green => 0x02,
            Color::Yellow => 0x03,
            Color::Blue => 0x04,
            Color::Magenta => 0x05,
            Color::Cyan => 0x06,
            Color::White => 0x07,
            Color::Byte(b) => b,
            Color::Default => panic!("Attempted to cast default color to byte"),
        }
    }

    pub fn as_16color(&self) -> u16 {
        match *self {
            Color::Default => 0x00,
            Color::Black => 0x10,
            Color::Red => 0x01,
            Color::Green => 0x02,
            Color::Yellow => 0x03,
            Color::Blue => 0x04,
            Color::Magenta => 0x05,
            Color::Cyan => 0x06,
            Color::White => 0x0F,
            Color::Byte(b) => panic!("Attempted to cast color byte {} to 16 color mode", b),
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::Black
    }
}

mod style {
    bitflags! {
        #[repr(C)]
        flags Style: u16 {
            const TB_NORMAL_COLOR = 0x000F,
            const RB_BOLD = 0x0100,
            const RB_UNDERLINE = 0x0400,
            const RB_REVERSE = 0x0800,
            const RB_NORMAL = 0x0000,
            const TB_ATTRIB = RB_BOLD.bits | RB_UNDERLINE.bits | RB_REVERSE.bits,
        }
    }

    impl Style {
        pub fn from_color(color: super::Color) -> Style {
            Style { bits: color.as_16color() & TB_NORMAL_COLOR.bits }
        }

        pub fn from_256color(color: super::Color) -> Style {
            Style { bits: color.as_256color() }
        }
    }
}

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, key: 0, ch: 0, meta: 0, w: 0, h: 0, x: 0, y: 0 };

#[derive(Debug)]
pub enum EventError {
   TermboxError,
   Unknown(isize),
}

impl fmt::Display for EventError {
   fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      write!(fmt, "{}", self.to_string())
   }
}

impl Error for EventError {
   fn description(&self) -> &str {
      match *self {
         EventError::TermboxError => "Error in Termbox",
         // I don't know how to format this without lifetime error.
         // EventError::Unknown(n) => &format!("There was an unknown error. Error code: {}", n),
         EventError::Unknown(_) => "Unknown error in Termbox",
      }
   }
}

impl FromPrimitive for EventError {
   fn from_i64(n: i64) -> Option<EventError> {
      match n {
         -1 => Some(EventError::TermboxError),
         n => Some(EventError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<EventError> {
      Some(EventError::Unknown(n as isize))
   }
}

pub type EventResult = Result<Event, EventError>;

fn unpack_key(event_key: u16, event_char: u32) -> Option<ExtendedKey> {
    let mut modifiers = Modifiers::new();
    let parsed_key = char::from_u32(event_char);
    match parsed_key {
        Some(c) => {
            if c.is_ascii_uppercase() {
                modifiers = Modifiers::new_all(false, true, false);
                return Some(ExtendedKey::new(Key::Char(c), modifiers));
            } else {
                return Some(ExtendedKey::new(Key::Char(c), modifiers));
            }
        }
        _ => {}
    }
    return Some(ExtendedKey::new(Key::Unknown(event_key), Modifiers::new()));
}

/// Unpack a RawEvent to an Event
///
/// if the `raw` parameter is true, then the Event variant will be the raw
/// representation of the event.
///     for instance KeyEventRaw instead of KeyEvent
///
/// This is useful if you want to interpret the raw event data yourself, rather
/// than having rustbox translate it to its own representation.
fn unpack_event(ev_type: c_int, ev: &RawEvent, raw: bool) -> EventResult {
    match ev_type {
        0 => Ok(Event::NoEvent),
        1 => Ok(
            if raw {
                Event::KeyEventRaw(ev.meta, ev.key, ev.ch)
            } else {
                // TODO: emod has alt modifier
                // there is no shift modifier to check for so we simplify - check for uppercase
                let k = match ev.key {
                    0 => unpack_key(ev.key, ev.ch),
                    // TODO: figure out how to get ctrl+shift shortcuts
                    a => ExtendedKey::from_code(a),
                };


                if let Some(key) = k {
                    let mut actual_key = key;
                    actual_key.raw_emod = ev.meta;
                    actual_key.raw_key = ev.key;
                    actual_key.raw_ch = ev.ch;
                    return Ok(Event::KeyEvent(actual_key))
                } else {
                    Event::KeyEvent(ExtendedKey::new(Key::Unknown(ev.key), Modifiers::new()))
                }
            }),
        2 => Ok(Event::ResizeEvent(ev.w, ev.h)),
        3 => {
            let mouse = Mouse::from_code(ev.key).unwrap_or(Mouse::Left);
            Ok(Event::MouseEvent(mouse, ev.x, ev.y))
        },
        // `unwrap` is safe here because FromPrimitive for EventError only returns `Some`.
        n => Err(FromPrimitive::from_isize(n as isize).unwrap()),
    }
}

#[derive(Debug)]
pub enum InitError {
    BufferStderrFailed(io::Error),
    AlreadyOpen,
    UnsupportedTerminal,
    FailedToOpenTTy,
    PipeTrapError,
    Unknown(isize),
}

impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::BufferStderrFailed(_) => "Could not redirect stderr",
            InitError::AlreadyOpen => "RustBox is already open",
            InitError::UnsupportedTerminal => "Unsupported terminal",
            InitError::FailedToOpenTTy => "Failed to open TTY",
            InitError::PipeTrapError => "Pipe trap error",
            InitError::Unknown(_) => "Unknown error from Termbox",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            InitError::BufferStderrFailed(ref e) => Some(e),
            _ => None
        }
    }
}

impl FromPrimitive for InitError {
   fn from_i64(n: i64) -> Option<InitError> {
      match n {
         -1 => Some(InitError::UnsupportedTerminal),
         -2 => Some(InitError::FailedToOpenTTy),
         -3 => Some(InitError::PipeTrapError),
         n => Some(InitError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<InitError> {
      Some(InitError::Unknown(n as isize))
   }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    // We only bother to redirect stderr for the moment, since it's used for panic!
    _stderr: Option<Hold>,

    // Store this so we know which colours to use
    output_mode: OutputMode,

    // Used/obtained by methods that read from the terminal (or termbox's
    // internal state). Termbox is only thread safe to the extent that one
    // thread can read while another writes; this lock prevents overlapping
    // reads.
    // See https://github.com/nsf/termbox/commit/493dc1395c91174e97658ff15fa2380227faf28f
    input_lock: Mutex<()>,

    // Used/obtained by methods that write to the terminal (or termbox's
    // internal state). Termbox is only thread safe to the extent that one
    // thread can read while another writes; this lock prevents overlapping
    // writes.
    // See https://github.com/nsf/termbox/commit/493dc1395c91174e97658ff15fa2380227faf28f
    output_lock: Mutex<()>,
}

#[derive(Clone, Copy,Debug)]
pub struct InitOptions {
    /// Use this option to initialize with a specific input mode
    ///
    /// See InputMode enum for details on the variants.
    pub input_mode: InputMode,

    /// Use this option to initialize with a specific output mode
    ///
    /// See OutputMode enum for details on the variants.
    pub output_mode: OutputMode,

    /// Use this option to automatically buffer stderr while RustBox is running.  It will be
    /// written when RustBox exits.
    ///
    /// This option uses a nonblocking OS pipe to buffer stderr output.  This means that if the
    /// pipe fills up, subsequent writes will fail until RustBox exits.  If this is a concern for
    /// your program, don't use RustBox's default pipe-based redirection; instead, redirect stderr
    /// to a log file or another process that is capable of handling it better.
    pub buffer_stderr: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        InitOptions {
            input_mode: InputMode::Current,
            output_mode: OutputMode::Normal,
            buffer_stderr: false,
        }
    }
}

impl RustBox {
    /// Initialize rustbox.
    ///
    /// For the default options, you can use:
    ///
    /// ```
    /// use rustbox::RustBox;
    /// use std::default::Default;
    /// let rb = RustBox::init(Default::default());
    /// ```
    ///
    /// Otherwise, you can specify:
    ///
    /// ```
    /// use rustbox::{RustBox, InitOptions};
    /// use std::default::Default;
    /// let rb = RustBox::init(InitOptions { input_mode: rustbox::InputMode::Esc, ..Default::default() });
    /// ```
    pub fn init(opts: InitOptions) -> Result<RustBox, InitError> {
        let stderr = if opts.buffer_stderr {
            Some(try!(Hold::stderr().map_err(|e| InitError::BufferStderrFailed(e))))
        } else {
            None
        };

        // Create the RustBox.
        let mut rb = unsafe { 
            if opts.output_mode != OutputMode::NoOutput {
                match termbox::tb_init() {
                    0 => RustBox {
                        _stderr: stderr,
                        output_mode: opts.output_mode,
                        input_lock: Mutex::new(()),
                        output_lock: Mutex::new(()),
                    },
                    res => {
                        return Err(FromPrimitive::from_isize(res as isize).unwrap())
                    }
                }
            } else {
                RustBox {
                    _stderr: stderr,
                    output_mode: opts.output_mode,
                    input_lock: Mutex::new(()),
                    output_lock: Mutex::new(()),
                }
            }
        };

        /*
        // TODO: set input mode
        match opts.input_mode {
            InputMode::Current=> (),
            _ => rb.set_input_mode(opts.input_mode),
        }
        */

        match opts.output_mode {
            OutputMode::Normal => (),
            OutputMode::NoOutput => (),
            _ => rb.set_output_mode(opts.output_mode),
        }

        Ok(rb)
    }

    pub fn width(&self) -> usize {
        if self.output_mode == OutputMode::NoOutput {
            return 0;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_width() as usize }
    }

    pub fn height(&self) -> usize {
        if self.output_mode == OutputMode::NoOutput {
            return 0;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_height() as usize }
    }

    pub fn clear(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_clear_buffer() }
    }

    pub fn clear_screen(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_clear_screen() }
    }

    pub fn present(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_render() }
    }

    pub fn set_cursor(&self, x: isize, y: isize) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        unsafe { termbox::tb_set_cursor(x as c_int, y as c_int) }
    }

    pub fn print(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, s: &str) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        let fg_int;
        let bg_int;

        match self.output_mode {
            // 256 color mode
            OutputMode::EightBit => {
                fg_int = Style::from_256color(fg) | (sty & style::TB_ATTRIB);
                bg_int = Style::from_256color(bg);
            },

            // 16 color mode
            _ => {
                fg_int = Style::from_color(fg) | (sty & style::TB_ATTRIB);
                bg_int = Style::from_color(bg);
            }
        }

        for (i, ch) in s.chars().enumerate() {
            unsafe {
                self.change_cell(x+i, y, ch as u32, fg_int.bits(), bg_int.bits());
            }
        }
    }

    pub fn print_char(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, ch: char) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let _lock = self.output_lock.lock();

        let fg_int;
        let bg_int;

        match self.output_mode {
            // 256 color mode
            OutputMode::EightBit => {
                fg_int = Style::from_256color(fg) | (sty & style::TB_ATTRIB);
                bg_int = Style::from_256color(bg);
            },

            // 16 color mode
            _ => {
                fg_int = Style::from_color(fg) | (sty & style::TB_ATTRIB);
                bg_int = Style::from_color(bg);
            }
        }
        unsafe {
            self.change_cell(x, y, ch as u32, fg_int.bits(), bg_int.bits());
        }
    }

    pub fn poll_event(&self, raw: bool) -> EventResult {
        if self.output_mode == OutputMode::NoOutput {
            return Ok(Event::NoEvent);
        }
        let _lock = self.input_lock.lock();
        let mut ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_poll_event(&mut ev)
        };
        unpack_event(rc, &ev, raw)
    }

    pub fn peek_event(&self, timeout: Duration, raw: bool) -> EventResult {
        if self.output_mode == OutputMode::NoOutput {
            return Ok(Event::NoEvent);
        }
        let _lock = self.input_lock.lock();
        let mut ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_peek_event(&mut ev, (timeout.as_secs() * 1000 + timeout.subsec_nanos() as u64 / 1000000) as c_int)
        };
        unpack_event(rc, &ev, raw)
    }

    pub unsafe fn change_cell(&self, x: usize, y: usize, ch: u32, fg: u16, bg: u16) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        let mut cell = RawCell{
            ch: ch,
            fg: fg,
            bg: bg,
        };
        termbox::tb_cell(x as i32, y as i32, &mut cell);
    }

    pub fn hide_cursor(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        unsafe {
            termbox::tb_hide_cursor();
        }
    }

    pub fn show_cursor(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        unsafe {
            termbox::tb_show_cursor();
        }
    }

    pub fn enable_mouse(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        unsafe {
            termbox::tb_enable_mouse();
        }
    }

    pub fn disable_mouse(&self) {
        if self.output_mode == OutputMode::NoOutput {
            return;
        }
        unsafe {
            termbox::tb_disable_mouse();
        }
    }

    /*
    pub fn set_input_mode(&self, mode: InputMode) {
        let _lock = self.output_lock.lock();

        unsafe {
            termbox::tb_select_input_mode(mode as c_int);
        }
    }
    */

    pub fn set_output_mode(&mut self, mode: OutputMode) {
        let _lock = self.output_lock.lock();

        self.output_mode = mode;
        if mode != OutputMode::NoOutput {
            unsafe {
                termbox::tb_select_output_mode(mode as c_int);
            }
        }
    }

    /// Convenience method to lock all (both input/output) access to
    /// Rustbox, shutdown termbox itself, and then defer to the caller (via F,
    /// while access is still locked). Once F completes, termbox is started and
    /// the locks are released.
    pub fn suspend<F>(&self, func: F)
        where F: FnOnce() -> ()
    {
        // Lock I/O until we've resumed.
        let _input_lock = self.input_lock.lock();
        let _output_lock = self.output_lock.lock();

        if self.output_mode != OutputMode::NoOutput {
            unsafe {
                termbox::tb_shutdown();
            }
        }

        func();

        if self.output_mode != OutputMode::NoOutput {
            unsafe {
                termbox::tb_init();
            }
        }
    }
}

impl Drop for RustBox {
    fn drop(&mut self) {
        if self.output_mode != OutputMode::NoOutput {
            // Since only one instance of the RustBox is ever accessible, we should not
            // need to do this atomically.
            // Note: we should definitely have RUSTBOX_RUNNING = true here.
            unsafe {
                termbox::tb_shutdown();
            }
        }
    }
}

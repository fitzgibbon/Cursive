//! User-input events and their effects.
//!
//! * Every user input the application receives is converted to an
//!   [`Event`](./enum.Event.html).
//! * Each event is then given to the root, and descends the view tree down to
//!   the view currently in focus, through the
//!   [`on_event`](../view/trait.View.html#method.on_event) method.
//!     * If the view consumes the event, it may return a callback to be
//!       executed.
//!     * Otherwise, it ignores the event, and the view parent can in turn
//!       choose to consume it or not.
//! * If no view consumes the event, the
//!   [global callback](../struct.Cursive.html#method.add_global_callback)
//!   table is checked.


use Cursive;
use std::ops::Deref;
use std::rc::Rc;

use vec::Vec2;

/// Callback is a function that can be triggered by an event.
/// It has a mutable access to the cursive root.
#[derive(Clone)]
pub struct Callback(Rc<Box<Fn(&mut Cursive)>>);
// TODO: remove the Box when Box<T: Sized> -> Rc<T> is possible

impl Callback {
    /// Wraps the given function into a `Callback` object.
    pub fn from_fn<F: Fn(&mut Cursive) + 'static>(f: F) -> Self {
        Callback(Rc::new(Box::new(f)))
    }
}

impl Deref for Callback {
    type Target = Box<Fn(&mut Cursive)>;
    fn deref<'a>(&'a self) -> &'a Box<Fn(&mut Cursive)> {
        &self.0
    }
}

impl From<Rc<Box<Fn(&mut Cursive)>>> for Callback {
    fn from(f: Rc<Box<Fn(&mut Cursive)>>) -> Self {
        Callback(f)
    }
}

impl From<Box<Fn(&mut Cursive) + Send>> for Callback {
    fn from(f: Box<Fn(&mut Cursive) + Send>) -> Self {
        Callback(Rc::new(f))
    }
}

impl From<Box<Fn(&mut Cursive)>> for Callback {
    fn from(f: Box<Fn(&mut Cursive)>) -> Self {
        Callback(Rc::new(f))
    }
}

/// Answer to an event notification.
/// The event can be consumed or ignored.
pub enum EventResult {
    /// The event was ignored. The parent can keep handling it.
    Ignored,
    /// The event was consumed. An optionnal callback to run is attached.
    Consumed(Option<Callback>), // TODO: make this a FnOnce?
}

impl EventResult {
    /// Convenient method to create `Consumed(Some(f))`
    pub fn with_cb<F: 'static + Fn(&mut Cursive)>(f: F) -> Self {
        EventResult::Consumed(Some(Callback::from_fn(f)))
    }

    /// Returns `true` if `self` is `EventResult::Consumed`.
    pub fn is_consumed(&self) -> bool {
        match *self {
            EventResult::Consumed(_) => true,
            EventResult::Ignored => false,
        }
    }

    /// Process this result if it is a callback.
    ///
    /// Does nothing otherwise.
    pub fn process(self, s: &mut Cursive) {
        if let EventResult::Consumed(Some(cb)) = self {
            cb(s);
        }
    }
}

/// A non-character key on the keyboard
#[derive(PartialEq,Eq,Clone,Copy,Hash,Debug)]
pub enum Key {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    /// Tabulation key
    Tab,
    /// Backspace key
    Backspace,
    /// Escape key
    Esc,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Insert key
    Ins,
    /// Delete key
    Del,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,

    /// Pause Break key
    PauseBreak,

    /// The 5 in the center of the keypad, when numlock is disabled.
    NumpadCenter,

    /// F0 key
    F0,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
}

impl Key {
    /// Returns the function key corresponding to the given number
    ///
    /// 1 -> F1, etc...
    ///
    /// # Panics
    ///
    /// If `n == 0 || n > 12`
    pub fn from_f(n: u8) -> Key {
        match n {
            0 => Key::F0,
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => panic!("unknown function key: F{}", n),
        }
    }
}

#[derive(PartialEq,Eq,Clone,Copy,Hash,Debug)]
/// A type of mouse event
pub enum MouseEvent {
    /// A button was pressed.
    Press(MouseButton),
    /// A button was released.
    Release(MouseButton),

    /// The mousewheel was moved up.
    WheelUp,
    /// The mousewheel was moved down.
    WheelDown,
}

#[derive(PartialEq,Eq,Clone,Copy,Hash,Debug)]
/// A button on the mouse
pub enum MouseButton {
    /// The left button
    Left,
    /// The right button
    Right,
    /// The middle button
    Middle,
}

/// Represents an event as seen by the application.
#[derive(PartialEq,Eq,Clone,Hash,Debug)]
pub enum Event {
    /// Event fired when the window is resized.
    WindowResize,

    /// Event fired regularly when a auto-refresh is set.
    Refresh,

    /// A character was entered (includes numbers, punctuation, ...).
    Char(char),
    /// A character was entered with the Ctrl key pressed.
    CtrlChar(char),
    /// A character was entered with the Alt key pressed.
    AltChar(char),

    /// A mouse event.
    Mouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },

    /// A mouse event with the Ctrl key pressed.
    CtrlMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },
    /// A mouse event with the Alt key pressed.
    AltMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },
    /// A mouse event with the Shift key pressed.
    ShiftMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },
    /// A mouse event with the Ctrl and Alt keys pressed.
    CtrlAltMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },
    /// A mouse event with the Shift key pressed.
    CtrlShiftMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },
    /// A mouse event with the Shift key pressed.
    AltShiftMouse {
        /// Position of the cursor
        pos: Vec2,
        /// Type of mouse event fired.
        event: MouseEvent,
    },

    /// A non-character key was pressed.
    Key(Key),
    /// A non-character key was pressed with the Shift key pressed.
    Shift(Key),
    /// A non-character key was pressed with the Alt key pressed.
    Alt(Key),
    /// A non-character key was pressed with the Shift and Alt keys pressed.
    AltShift(Key),
    /// A non-character key was pressed with the Ctrl key pressed.
    Ctrl(Key),
    /// A non-character key was pressed with the Ctrl and Shift keys pressed.
    CtrlShift(Key),
    /// A non-character key was pressed with the Ctrl and Alt keys pressed.
    CtrlAlt(Key),

    /// An unknown event was received.
    Unknown(Vec<u8>),

    #[doc(hidden)]
    /// The application is about to exit.
    Exit,
}

impl Event {
    /// Returns the position of the mouse.
    ///
    /// If this event is not a mouse event, returns `None`.
    pub fn mouse_position(&self) -> Option<Vec2> {
        match *self {
            Event::Mouse { pos, .. } |
            Event::CtrlMouse { pos, .. } |
            Event::ShiftMouse { pos, .. } |
            Event::AltMouse { pos, .. } |
            Event::CtrlShiftMouse { pos, .. } |
            Event::CtrlAltMouse { pos, .. } => Some(pos),
            _ => None,
        }
    }

    /// Updates self to be relative to the given context.
    ///
    /// If `self` is a mouse event with coordinates that fit in the given
    /// window, substract `top_left` from the position.
    ///
    /// Otherwise, do nothing.
    pub fn relativize(&mut self, top_left: Vec2) {
        match *self {
            Event::Mouse { ref mut pos, .. } |
            Event::CtrlMouse { ref mut pos, .. } |
            Event::ShiftMouse { ref mut pos, .. } |
            Event::AltMouse { ref mut pos, .. } |
            Event::CtrlShiftMouse { ref mut pos, .. } |
            Event::CtrlAltMouse { ref mut pos, .. } => {
                *pos = *pos - top_left;
            }
            _ => (),
        }
    }

    /// Attempt to create a new event relative to the given viewport
    ///
    /// If `self` is a mouse event with coordinates that fit in the given
    /// window, returns an event with the position relative to the window.
    ///
    /// Otherwise, returns `None`.
    pub fn make_relative(&self, top_left: Vec2, size: Option<Vec2>)
                         -> Option<Self> {
        // First, this only applies to mouse events
        match self.mouse_position() {
            None => Some(self.clone()),
            Some(pos) => {
                if !top_left.fits_in(pos) {
                    // println_stderr!("{:?} fits in {:?}", top_left, pos);
                    // We are too high or too far left
                    None
                } else if size.map(|size| !(pos < (top_left + size)))
                              .unwrap_or(false) {
                    // println_stderr!("{:?} < {:?} + {:?}", pos, top_left, size);
                    // We are too low or too far right
                    None
                } else {
                    let mut clone = self.clone();
                    clone.relativize(top_left);
                    Some(clone)
                }
            }
        }
    }
}

impl From<char> for Event {
    fn from(c: char) -> Event {
        Event::Char(c)
    }
}

impl From<Key> for Event {
    fn from(k: Key) -> Event {
        Event::Key(k)
    }
}

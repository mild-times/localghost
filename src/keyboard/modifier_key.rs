use std::fmt::Display;

/// Special keys which are used to generate special characters or cause special
/// actions when used in combination with other keys.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values#Modifier_keys)
#[derive(Debug)]
pub enum ModifierKey {
    /// The Alt (Alternative) key.
    Alt,
    /// The AltGr or AltGraph (Alternate Graphics) key. Enables the ISO Level 3
    /// shift modifier (where Shift is the level 2 modifier).
    AltGraph,
    /// The Caps Lock key. Toggles the capital character lock on and off for
    /// subsequent input.
    CapsLock,
    /// The Control, Ctrl, or Ctl key. Allows typing control characters.
    Control,
    /// The Fn (Function modifier) key. Used to allow generating function key
    /// (F1–F15, for instance) characters on keyboards without a dedicated function
    /// key area. Often handled in hardware so that events aren't generated for this
    /// key.
    Fn,
    /// The FnLock or F-Lock (Function Lock) key.Toggles the function key mode
    /// described by "Fn" on and off. Often handled in hardware so that events aren't
    /// generated for this key.
    FnLock,
    /// The Hyper key.
    Hyper,
    /// The Meta key. Allows issuing special command inputs. This is the Windows logo
    /// key, or the Command or ⌘ key on Mac keyboards.
    Meta,
    /// The NumLock (Number Lock) key. Toggles the numeric keypad between number
    /// entry some other mode (often directional arrows).
    NumLock,
    /// The Scroll Lock key. Toggles between scrolling and cursor movement modes.
    ScollLock,
    /// The Shift key. Modifies keystrokes to allow typing upper (or other) case
    /// letters, and to support typing punctuation and other special characters.
    Shift,
    /// The Super key.
    Super,
    /// The Symbol modifier key (found on certain virtual keyboards).
    Symbol,
    /// The Symbol Lock key.
    SymbolLock,
}

impl ModifierKey {
    /// Convert the enum to a `&'static str`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alt => "Alt",
            Self::AltGraph => "AltGraph",
            Self::CapsLock => "CapsLock",
            Self::Control => "Control",
            Self::Fn => "Fn",
            Self::FnLock => "FnLock",
            Self::Hyper => "Hyper",
            Self::Meta => "Meta",
            Self::NumLock => "NumLock",
            Self::ScollLock => "ScrollLock",
            Self::Shift => "Shift",
            Self::Super => "Super",
            Self::Symbol => "Symbol",
            Self::SymbolLock => "SymbolLock",
        }
    }
}

impl Display for ModifierKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

use crate::keyboard::{KeyKind, ModifierKey};

/// An event that describes a user interaction with the keyboard.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
#[derive(Debug)]
pub struct KeyboardEvent {
    pub(crate) inner: web_sys::KeyboardEvent,
}

impl KeyboardEvent {
    /// Returns the value of the key pressed by the user, taking into
    /// consideration the state of modifier keys such as Shift as well as the
    /// keyboard locale and layout.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    pub fn key(&self) -> KeyKind {
        let key = self.inner.key();
        match key.as_str() {
            "Unidentified" => KeyKind::Unidentified,
            "Dead" => KeyKind::Dead,
            _ => KeyKind::Key(key),
        }
    }

    /// Indicates if the `alt` key (`Option` or `⌥` on macOS) was pressed when
    /// the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.inner.alt_key()
    }

    /// Indicates if the `control` key was pressed when the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)
    pub fn ctrl_key(&self) -> bool {
        self.inner.ctrl_key()
    }

    /// Indicates if the `shift` key was pressed when the event occured.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)
    pub fn shift_key(&self) -> bool {
        self.inner.shift_key()
    }

    /// Indicates whether the specified key was pressed or locked when the event
    /// occurred.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)
    pub fn modifier_key(&self, modifier: ModifierKey) -> bool {
        self.inner.get_modifier_state(modifier.as_str())
    }

    /// Indicates if the event is fired within a composition session.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)
    pub fn is_composing(&self) -> bool {
        self.inner.is_composing()
    }

    /// Indicates if the given key is being held down such that it is automatically repeating.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
    pub fn is_repeating(&self) -> bool {
        self.inner.repeat()
    }

    // TODO: location, init an enum -- https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    // TODO: key_code, init an enum -- https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
}

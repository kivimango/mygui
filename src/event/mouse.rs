use orbclient::ButtonEvent;

/// The clicked button during a pressed MouseClickEvent.
#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left, Middle, Right, None
}

/// The state of the mouse buttons during a MouseClickEvent.
#[derive(Debug, PartialEq)]
pub enum MouseButtonState {
    Pressed, Released
}

/// A mouse click event converted from orbclient::ButtonEvent.
/// Orbclient sends two ButtonEvent, one for click and one for release.
/// MouseClickEvents processed by the mouse_system().
pub struct MouseClickEvent {
    pub button: MouseButton,
    pub state: MouseButtonState
}

impl From<ButtonEvent> for MouseClickEvent {
    fn from(button_event: ButtonEvent) -> Self {
        let mut button = MouseButton::None;
        let mut state = MouseButtonState::Pressed;

        if button_event.left || button_event.middle || button_event.right {
            state = MouseButtonState::Pressed;
        } else if !button_event.left && !button_event.middle && !button_event.right {
            state = MouseButtonState::Released;
        }

        if state == MouseButtonState::Pressed {
            if button_event.left {
                button = MouseButton::Left;
            } else if button_event.middle {
                button = MouseButton::Middle;
            } else if button_event.right {
                button = MouseButton::Right;
            }
        }

        MouseClickEvent {
            button,
            state
        }       
    }
}

mod test {
    use orbclient::ButtonEvent;
    use super::{MouseClickEvent, MouseButton, MouseButtonState};

    #[test]
    fn test_from_left_click() {
        let orb_mouse_event = ButtonEvent {
            left: true,
            middle: false,
            right: false,
        };

        let mygui_mouse_event = MouseClickEvent::from(orb_mouse_event);

        assert_eq!(mygui_mouse_event.button, MouseButton::Left);
        assert_eq!(mygui_mouse_event.state, MouseButtonState::Pressed);
    }

    #[test]
    fn test_from_release() {
        let orb_mouse_event = ButtonEvent {
            left: false,
            middle: false,
            right: false,
        };

        let mygui_mouse_event = MouseClickEvent::from(orb_mouse_event);

        assert_eq!(mygui_mouse_event.button, MouseButton::None);
        assert_eq!(mygui_mouse_event.state, MouseButtonState::Released);
    }
}
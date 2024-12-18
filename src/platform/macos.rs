use core_graphics::event::{CGEvent, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;

pub struct MacOSMouseController;

impl super::MouseController for MacOSMouseController {
    type Error = ();

    fn new() -> Self {
        Self
    }

    fn get_position(&self) -> (i32, i32) {
        let event = CGEvent::new(
            CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
                .expect("Failed to create CGEventSource"),
        );

        match event {
            Ok(event) => {
                let point = event.location();
                (point.x as i32, point.y as i32)
            }
            Err(_) => (0, 0),
        }
    }

    fn set_position(&mut self, x: i32, y: i32) -> Result<(), Self::Error> {
        let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)?;
        let event = CGEvent::new_mouse_event(
            source,
            core_graphics::event::CGEventType::MouseMoved,
            CGPoint::new(x as f64, y as f64),
            core_graphics::event::CGMouseButton::Left,
        )?;
        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn hook_mouse(&mut self) -> Result<(), Self::Error> {
        // Not implemented.
        Ok(())
    }

    fn unhook_mouse(&mut self) -> Result<(), Self::Error> {
        // Not implemented.
        Ok(())
    }
}

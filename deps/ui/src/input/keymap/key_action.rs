use refs::Weak;
use vents::Event;

use crate::KeymapKey;

pub struct KeyAction {
    pub key:    KeymapKey,
    action:     Event,
    subscriber: Weak,
}

impl KeyAction {
    pub fn new<T: ?Sized>(
        subscriber: Weak<T>,
        key: impl Into<KeymapKey>,
        action: impl FnMut() + Send + 'static,
    ) -> Self {
        let event = Event::default();
        event.sub(action);
        Self {
            subscriber: subscriber.erase(),
            key:        key.into(),
            action:     event,
        }
    }
}

impl KeyAction {
    pub fn check(&self, key: KeymapKey) -> bool {
        if self.subscriber.is_null() {
            return false;
        }
        if self.key == key {
            self.action.trigger(());
        }
        true
    }
}

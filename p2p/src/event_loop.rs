use futures::{empty, Empty};
use tokio_core::reactor::Core;

/// Creates a new event loop
pub fn event_loop() -> Core {
    Core::new().unwrap()
}

/// Returns a future that runs forever.
pub fn forever() -> Empty<(), ()> {
    empty()
}

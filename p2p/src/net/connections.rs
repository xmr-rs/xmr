use parking_lot::RwLock;
use std::collections::HashMap;

use net::SharedTcpStream;
use types::PeerId;

pub struct Connections {
    channels: RwLock<HashMap<PeerId, SharedTcpStream>>,
}

impl Connections {
    pub fn new() -> Connections {
        Connections {
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn channel(&self, id: &PeerId) -> Option<SharedTcpStream> {
        self.channels.read().get(id).cloned()
    }

    pub fn store(&self, id: PeerId, stream: SharedTcpStream) -> SharedTcpStream {
		self.channels.write().insert(id, stream.clone());
        stream
    }
}

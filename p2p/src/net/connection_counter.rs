use std::cmp;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::SocketAddr;
use std::collections::HashMap;

use parking_lot::RwLock;

/// Connection type.
#[derive(Debug, Clone)]
pub enum ConnectionType {
    Inbound,
    Outbound,
}

/// Counts number of open inbound and outbound connections.
pub struct ConnectionCounter {
    /// Current number of inbound connections.
    current_inbound_connections: AtomicUsize,
    /// Current number of outbound connections.
    current_outbound_connections: AtomicUsize,
    /// Maximum number of inbound connections.
    max_inbound_connections: u32,
    /// Maximum number of outbound connections.
    max_outbound_connections: u32,
    /// Connection type.
    connection_type: RwLock<HashMap<SocketAddr, ConnectionType>>,
}

impl ConnectionCounter {
    pub fn new(max_inbound_connections: u32, max_outbound_connections: u32) -> Self {
        let total_max_connections = max_inbound_connections + max_outbound_connections;
        ConnectionCounter {
            current_inbound_connections: AtomicUsize::new(0),
            current_outbound_connections: AtomicUsize::new(0),
            max_inbound_connections: max_inbound_connections,
            max_outbound_connections: max_outbound_connections,
            connection_type: RwLock::new(HashMap::with_capacity(total_max_connections as _)),
        }
    }

    /// Increases inbound connections counter by 1.
    pub fn note_new_inbound_connection(&self, addr: SocketAddr) {
        self.current_inbound_connections
            .fetch_add(1, Ordering::AcqRel);
        self.connection_type.write().insert(addr, ConnectionType::Inbound);
    }

    /// Increases outbound connections counter by 1.
    pub fn note_new_outbound_connection(&self, addr: SocketAddr) {
        self.current_outbound_connections
            .fetch_add(1, Ordering::AcqRel);
        self.connection_type.write().insert(addr, ConnectionType::Outbound);
    }

    /// Closes an inbound or outbound connection depending on the
    /// direction of `addr` and decreases their counter by 1 respectively.
    pub fn note_close_connection(&self, addr: &SocketAddr) {
        if let Some(connection) = self.connection_type.write().remove(addr) {
            match connection {
                ConnectionType::Outbound => {
                    self.current_outbound_connections
                        .fetch_sub(1, Ordering::AcqRel);
                }
                ConnectionType::Inbound => {
                    self.current_inbound_connections
                        .fetch_sub(1, Ordering::AcqRel);
                }
            }
        }
    }

    /// Returns number of inbound connections needed to reach the maximum
    pub fn inbound_connections_needed(&self) -> u32 {
        let ic = self.inbound_connections();
        ic.1 - cmp::min(ic.0, ic.1)
    }

    /// Returns number of inbound connections needed to reach the maximum
    pub fn outbound_connections_needed(&self) -> u32 {
        let oc = self.outbound_connections();
        oc.1 - cmp::min(oc.0, oc.1)
    }

    /// Returns a pair of unsigned integers where first element is current number
    /// of connections and the second is max.
    pub fn inbound_connections(&self) -> (u32, u32) {
        let current = self.current_inbound_connections.load(Ordering::Acquire) as u32;
        (current, self.max_inbound_connections)
    }

    /// Returns a pair of unsigned integers where first element is current number
    /// of connections and the second is max.
    pub fn outbound_connections(&self) -> (u32, u32) {
        let current = self.current_outbound_connections.load(Ordering::Acquire) as u32;
        (current, self.max_outbound_connections)
    }

    /// The type of connection
    pub fn connection_type(&self, addr: &SocketAddr) -> Option<ConnectionType> {
        self.connection_type.read().get(addr).cloned()
    }
}

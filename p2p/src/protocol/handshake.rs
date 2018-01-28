use std::marker::PhantomData;

use portable_storage::ser::Serializable;

use protocol::{P2P_COMMAND_BASE_ID, BasicNodeData};
use protocol::peerlist::PeerlistEntry;
use cryptonote::CoreSyncData;
use levin::Command;

use ser::DefaultSerializableLinkedList;

/// Handshake with `cryptonote::CoreSyncData` as the payload data.
pub type CryptoNoteHandshake = Handshake<CoreSyncData>;

/// The handshake command.
#[derive(Debug)]
pub struct Handshake<P: Serializable>(PhantomData<P>);

impl<P: Serializable> Command for Handshake<P> {
    type Request = HandshakeRequest<P>;
    type Response = HandshakeResponse<P>;

    const ID: u32 = P2P_COMMAND_BASE_ID + 1;
}

/// The handshake command request.
#[derive(Debug, Default, Clone)]
pub struct HandshakeRequest<P: Serializable> {
    pub node_data: BasicNodeData,
    pub payload_data: P,
}

serializable! {
    HandshakeRequest<P> where (P: Serializable) {
        node_data,
        payload_data,
    }
}

/// The handshake command response.
#[derive(Debug, Default, Clone)]
pub struct HandshakeResponse<P: Serializable> {
    pub node_data: BasicNodeData,
    pub payload_data: P,
    pub local_peerlist_new: DefaultSerializableLinkedList<PeerlistEntry>,
}

serializable! {
    HandshakeResponse<P> where (P: Serializable) {
        node_data,
        payload_data,
        local_peerlist_new,
    }
}

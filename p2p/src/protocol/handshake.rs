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

#[cfg(test)]
pub mod tests {
    use super::*;

    use std::io::Read;
    use std::fs::File;

    use uuid::Uuid;
    use cryptonote::CoreSyncData;
    use protocol::BasicNodeData;
    use levin::DefaultEndian;
    use hash::H256;
    use portable_storage::{self, Serialize};
    use bytes::BytesMut;

    #[test]
    fn check_test_vector() {
        // XXX: keep in sync with compat/test-vectors/src/main.cpp

        let expected_buf = include_bytes!("../../../compat/test-vectors/data/COMMAND_HANDSHAKE_T_TEST_VECTOR").to_vec();

        let network_id = &[0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61,
                           0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x11];
        let network_id = Uuid::from_bytes(network_id)
            .expect("couldn't create uuid from constant");

        let req = HandshakeRequest {
            node_data: BasicNodeData {
                network_id: network_id.into(),
                local_time: 0,
                my_port: 0,
                peer_id: 0.into(),
            },
            payload_data: CoreSyncData {
                current_height: 0,
                cumulative_difficulty: 0,
                top_id: H256::default(),
                top_version: 0,
            },
        };

        let section = req.serialize();

        let mut buf = BytesMut::new();
        portable_storage::write::<DefaultEndian>(&mut buf, &section);

        if buf.len() != expected_buf.len() {
            panic!("slice lengths aren't equal");
        }

        let mut i1 = buf.iter();
        let mut i2 = buf.iter();
        let mut offset = 0;
        loop {
            if let (Some(&b1), Some(&b2)) = (i1.next(), i2.next()) {
                if b1 != b2 {
                    panic!("slice's aren't equal: offset: {}, buf: {:?}, expected_buf: {:?}", offset, buf, expected_buf);
                }
                offset += 1;
            } else {
                break;
            }
        }
    }
}

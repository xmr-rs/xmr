use portable_storage_utils::stl::StlLinkedList;

use types::{P2P_COMMAND_BASE_ID, BasicNodeData, PeerlistEntry};
use types::cn::CoreSyncData;
use levin::Command;

/// The handshake command.
#[derive(Debug, Clone, Copy)]
pub struct Handshake;

impl Command for Handshake {
    type Request = HandshakeRequest;
    type Response = HandshakeResponse;

    const ID: u32 = P2P_COMMAND_BASE_ID + 1;
}

/// The handshake command request.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HandshakeRequest {
    pub node_data: BasicNodeData,
    pub payload_data: CoreSyncData,
}

/// The handshake command response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HandshakeResponse {
    pub node_data: BasicNodeData,
    pub payload_data: CoreSyncData,
    pub local_peerlist: StlLinkedList<PeerlistEntry>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use types::BasicNodeData;
    use types::cn::CoreSyncData;
    use levin::{Command, Storage};
    use network::Network;
    use primitives::H256;
    use bytes::BytesMut;
    use portable_storage;

    #[test]
    fn test_vector() {
        type Request = <Handshake as Command>::Request;
        let test_vector =
            include_bytes!("../../../../compat/test-vectors/data/COMMAND_HANDSHAKE_T_TEST_VECTOR").to_vec();
        let network = Network::Testnet;

        let req = Request {
            node_data: BasicNodeData {
                local_time: 0,
                my_port: 0,
                network_id: network.id().into(),
                peer_id: 0.into(),
            },
            payload_data: CoreSyncData {
                current_height: 0,
                cumulative_difficulty: 0,
                top_id: H256::default(),
                top_version: 0,
            },
        };

        let section = req.to_section().unwrap();
        let mut buf = BytesMut::new();
        portable_storage::write(&mut buf, &section);

        assert_eq!(buf.as_ref(), test_vector.as_slice());
    }
}

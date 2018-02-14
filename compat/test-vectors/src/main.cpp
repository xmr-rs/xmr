#include <string>
#include <fstream>

#include <p2p/p2p_protocol_defs.h>
#include <cryptonote_protocol/cryptonote_protocol_defs.h>
#include <storages/portable_storage.h>
#include <crypto/hash.h>
#include <cryptonote_config.h>

using epee::serialization::portable_storage;
using nodetool::COMMAND_HANDSHAKE_T;
using cryptonote::CORE_SYNC_DATA;
using crypto::hash;
using config::testnet::NETWORK_ID;

void command_handshake_t_request();

int main() {
    command_handshake_t_request();
    return 0;
}


void command_handshake_t_request() {
    auto stg = portable_storage();
    
    COMMAND_HANDSHAKE_T<CORE_SYNC_DATA>::request req = {
        .node_data = {
            .network_id = NETWORK_ID,
            .local_time = 0,
            .my_port = 0,
            .peer_id = 0
        },
        .payload_data = {
            .current_height = 0,
            .cumulative_difficulty = 0,
            .top_id = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
            .top_version = 0
        },
    };
    req.store(stg);

    auto buf = std::string();
    stg.store_to_binary(buf);

    auto output_file = std::ofstream("COMMAND_HANDSHAKE_T_TEST_VECTOR");
    output_file.write(buf.c_str(), buf.size());
}

#include <string>
#include <fstream>

#include <p2p/p2p_protocol_defs.h>
#include <cryptonote_protocol/cryptonote_protocol_defs.h>
#include <storages/portable_storage.h>
#include <crypto/hash.h>
#include <cryptonote_config.h>
#include <cryptonote_basic/cryptonote_basic.h>
#include <cryptonote_basic/blobdatatype.h>
#include <serialization/binary_archive.h>


template<class t_object>
bool to_blob(const t_object& to, cryptonote::blobdata& b_blob)
{
    std::stringstream ss;
    binary_archive<true> ba(ss);
    bool r = ::serialization::serialize(ba, const_cast<t_object&>(to));
    b_blob = ss.str();
    return r;
}


void command_handshake_t_request();
void block_header();

int main() {
    command_handshake_t_request();
    block_header();
    return 0;
}

void command_handshake_t_request() {
    using epee::serialization::portable_storage;
    using nodetool::COMMAND_HANDSHAKE_T;
    using cryptonote::CORE_SYNC_DATA;
    using crypto::hash;
    using config::testnet::NETWORK_ID;

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

void block_header() {
    using std::exit;
    using cryptonote::block_header;
    using serialization::serialize;


    block_header hdr = {
        .major_version = 1,
        .minor_version = 0,
        .timestamp = 0,
        .prev_id = {0},
        .nonce = 0
    };

    std::string buf = std::string();
    if (!to_blob(hdr, buf)) {
        exit(-1);
    }

    auto output_file = std::ofstream("BLOCK_HEADER_TEST_VECTOR");
    output_file.write(buf.c_str(), buf.size());
}

use std::net::{SocketAddr, Shutdown};
use std::sync::Arc;
use std::error::Error as StdError;
use std::time::{SystemTime, UNIX_EPOCH};

use failure::Error;
use futures_cpupool::CpuPool;
use futures::{Future, finished};
use tokio_core::reactor::{Handle, Remote};
use rand::OsRng;

use parking_lot::RwLock;

use network::Network;
use db::SharedStore;

use config::Config;
use types::PeerId;
use net::{connect, ConnectionCounter, Connections, PeerContext};
use protocol::{OutboundSync, LocalSyncNodeRef};
use levin::Command;
use types::{BasicNodeData, PeerlistEntry};
use types::cmd::Handshake;
use types::cn::CoreSyncData;
use utils::Peerlist;

pub type BoxedEmptyFuture = Box<Future<Item=(), Error=()> + Send>;

pub struct Context {
    connection_counter: ConnectionCounter,
    local_sync_node: LocalSyncNodeRef,
    pub(crate) remote: Remote,
    pub(crate) pool: CpuPool,
    pub(crate) connections: Connections,
    pub(crate) peerlist: RwLock<Peerlist>,
    pub(crate) config: Config,
    pub(crate) peer_id: PeerId,
}

impl Context {
    pub fn new(pool_handle: CpuPool,
               remote: Remote,
               config: Config,
               local_sync_node: LocalSyncNodeRef) -> Context {
        let mut rng = OsRng::new().expect("Cannot open OS random.");
        let peer_id = PeerId::random(&mut rng);
        Context {
            connection_counter: ConnectionCounter::new(config.in_peers, config.out_peers),
            local_sync_node,
            remote: remote,
            pool: pool_handle,
            connections: Connections::new(),
            peerlist: RwLock::new(Peerlist::new()),
            config,
            peer_id,
        }
    }

    pub fn connect(context: Arc<Context>,
                   address: SocketAddr,
                   req: <Handshake as Command>::Request) {
        trace!("connect request: {:?}" , req);
        trace!("connect address: {:?}", address);

        context.connection_counter.note_new_outbound_connection();
        context.remote.clone().spawn(move |handle| {
            context.pool.clone().spawn(Self::connect_future(context.clone(), handle, address, req))
        })
    }

    pub fn connect_future(context: Arc<Context>,
                          handle: &Handle,
                          address: SocketAddr,
                          req: <Handshake as Command>::Request) -> BoxedEmptyFuture {
        let connection = connect(&address, handle, context.clone(), req);
        Box::new(connection.then(move |result| {
            match result {
                Ok((stream, response)) => {
                    match response {
                        Ok(response) => {
                            trace!("connect response - {:?}", response);
                            let addr = match address {
                                SocketAddr::V4(ref v4) => v4.clone(),
                                SocketAddr::V6(_) => {
                                    warn!("IPv6 addresses aren't supported yet.");
                                    stream.shutdown(Shutdown::Both).ok();
                                    context.connection_counter.note_close_outbound_connection();
                                    return finished(());
                                },
                            };
                            let info = PeerlistEntry {
                                adr: addr.into(),
                                id: response.node_data.peer_id,
                                // TODO: last seen time
                                last_seen: 0,
                            };
                            context.peerlist.write().insert(address, info.clone());
                            let peer_context = PeerContext::new(context.clone(), info);
                            let outbound_sync_connection = Arc::new(
                                OutboundSync::new(peer_context)
                            );
                            context.local_sync_node.new_sync_connection(
                                response.node_data.peer_id,
                                &response.payload_data,
                                outbound_sync_connection,
                            );
                            context.connections.store(
                                response.node_data.peer_id,
                                stream.into(),
                            );
                        },
                        Err(e) => {
                            stream.shutdown(Shutdown::Both).ok();
                            context.connection_counter.note_close_outbound_connection();
                            warn!("node returned invalid data: {:?}", e);
                        }
                    }
                },
                Err(e) => {
                    context.connection_counter.note_close_outbound_connection();
                    warn!("couldn't establish connection to node: {}", e.description());
                }
            }

            finished(())
        }))
    }

    fn basic_node_data(&self) -> BasicNodeData {
        let my_port = if self.config.hide_my_port {
            0
        } else {
            self.config.listen_port.unwrap_or(self.config.network.listen_port())
        };

        let local_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("the system time is behind unix epoch").as_secs();

        BasicNodeData {
            network_id: self.config.network.id().into(),
            local_time,
            my_port,
            peer_id: self.peer_id,
        }
    }
}

pub struct P2P {
    _event_loop_handle: Handle,
    context: Arc<Context>,
    _pool: CpuPool,
}

impl P2P {
    pub fn new(config: Config, local_sync_node: LocalSyncNodeRef, handle: Handle) -> P2P {
        trace!("p2p config: {:?}", config);

        let pool = CpuPool::new(config.threads);
        let remote = handle.remote().clone();
        P2P {
            _event_loop_handle: handle,
            context: Arc::new(Context::new(pool.clone(), remote, config.clone(), local_sync_node)),
            _pool: pool,
        }
    }

    pub fn run(&self, store: SharedStore) -> Result<(), Error> {
        type Request = <Handshake as Command>::Request;

        trace!("running p2p");

        for addr in self.context.config.peers.iter() {
            let req = Request {
                node_data: self.context.basic_node_data(),
                payload_data: core_sync_data(store.clone(), &self.context.config.network),
            };

            Context::connect(self.context.clone(), addr.clone(), req)
        }

        Ok(())
    }
}

fn core_sync_data(store: SharedStore, network: &Network) -> CoreSyncData {
    let best_block = store.best_block();
    CoreSyncData {
        current_height: best_block.height,
        cumulative_difficulty: 0,
        top_id: best_block.id,
        top_version: network.hard_forks().ideal_version(),
    }
}

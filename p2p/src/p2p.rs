use std::net::SocketAddr;
use std::sync::Arc;
use std::error::Error as StdError;

use failure::Error;
use futures_cpupool::CpuPool;
use futures::{Future, finished};
use tokio_core::reactor::{Handle, Remote};
use rand::OsRng;

use network::Network;
use db::Store;

use config::Config;
use protocol::PeerId;
use net::{connect, ConnectionCounter};
use levin::Command;
use cryptonote::CoreSyncData;
use protocol::BasicNodeData;
use protocol::handshake::Handshake;

pub type BoxedEmptyFuture = Box<Future<Item=(), Error=()> + Send>;

pub struct Context {
    connection_counter: ConnectionCounter,
    remote: Remote,
    pool: CpuPool,
    pub(crate) config: Config,
    pub(crate) peer_id: PeerId,
}

impl Context {
    pub fn new(pool_handle: CpuPool,
               remote: Remote,
               config: Config) -> Context {
        let mut rng = OsRng::new().expect("Cannot open OS random.");
        let peer_id = PeerId::random(&mut rng);
        Context {
            // TODO: Add a cfg for max inbound/outbound connections
            connection_counter: ConnectionCounter::new(5, 5),
            remote: remote,
            pool: pool_handle,
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
                Ok((_, response)) => {
                    match response {
                        Ok(_response) => (),
                        Err(e) => {
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

        BasicNodeData {
            network_id: self.config.network.id().into(),
            local_time: 0,
            my_port,
            peer_id: self.peer_id,
        }
    }
}

pub struct P2P {
    event_loop_handle: Handle,
    context: Arc<Context>,
    _pool: CpuPool,
}

impl P2P {
    pub fn new(config: Config, handle: Handle) -> P2P {
        trace!("p2p config: {:?}", config);

        let pool = CpuPool::new(config.threads);
        let remote = handle.remote().clone();
        P2P {
            event_loop_handle: handle,
            context: Arc::new(Context::new(pool.clone(), remote, config.clone())),
            _pool: pool,
        }
    }

    pub fn run<S>(&self, store: &S) -> Result<(), Error> where S: Store {
        type Request = <Handshake as Command>::Request;

        trace!("running p2p");

        for addr in self.context.config.peers.iter() {
            let req = Request {
                node_data: self.context.basic_node_data(),
                payload_data: core_sync_data(store, &self.context.config.network),
            };

            Context::connect(self.context.clone(), addr.clone(), req)
        }

        Ok(())
    }
}

fn core_sync_data<S>(store: &S, network: &Network) -> CoreSyncData where S: Store {
    let best_block = store.best_block();
    CoreSyncData {
        current_height: best_block.height,
        cumulative_difficulty: 0,
        top_id: best_block.id,
        top_version: network.hard_forks().ideal_version(),
    }
}

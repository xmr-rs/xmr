use std::net::SocketAddr;
use std::sync::Arc;

use failure::Error;
use futures_cpupool::CpuPool;
use futures::{Future, empty};
use tokio_core::reactor::{Handle, Remote};
use rand::OsRng;

use db::SharedBlockChain;

use config::Config;
use protocol::PeerId;
use net::{connect, ConnectionCounter};

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

    pub fn connect(context: Arc<Context>, address: SocketAddr) {
        context.connection_counter.note_new_outbound_connection();
        context.remote.clone().spawn(move |handle| {
            context.pool.clone().spawn(Self::connect_future(context.clone(), address))
        })
    }

    pub fn connect_future(_context: Arc<Context>, _address: SocketAddr) -> BoxedEmptyFuture {
        Box::new(empty())
    }
}

pub struct P2P {
    _event_loop_handle: Handle,
    _context: Context,
    _pool: CpuPool,
}

impl P2P {
    pub fn new(config: Config, handle: Handle, _db: SharedBlockChain) -> P2P {
        let pool = CpuPool::new(config.threads);
        P2P {
            _context: Context::new(pool.clone(), handle.remote().clone(), config.clone()),
            _event_loop_handle: handle,
            _pool: pool,
        }
    }

    pub fn run(&self) -> Result<(), Error> { Ok(()) }
}

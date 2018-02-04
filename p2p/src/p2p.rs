use std::net::SocketAddr;
use std::sync::Arc;

use failure::Error;
use futures_cpupool::CpuPool;
use futures;
use tokio_core::reactor::{Handle, Remote};

use db::SharedBlockChain;

use config::Config;
use net::ConnectionCounter;

pub struct Context {
    connection_counter: ConnectionCounter,
    remote: Remote,
    pool: CpuPool,
    config: Config,
    blockchain: SharedBlockChain,
}

impl Context {
    pub fn new(pool_handle: CpuPool,
               remote: Remote,
               config: Config,
               db: SharedBlockChain) -> Context {
        Context {
            // TODO: Add a cfg for max inbound/outbound connections
            connection_counter: ConnectionCounter::new(5, 5),
            remote: remote,
            pool: pool_handle,
            config,
        }
    }

    pub fn connect(context: Arc<Context>, address: SocketAddr) {
        context.connection_counter.note_new_outbound_connection();
        context.remote.clone().spawn(move |handle| {
            context.pool.clone().spawn(futures::empty())
        })
    }

    pub fn connect_future(_context: Arc<Context>, _address: SocketAddr) {}
}

pub struct P2P {
    _event_loop_handle: Handle,
    _context: Context,
    _pool: CpuPool,
}

impl P2P {
    pub fn new(config: Config, handle: Handle, db: SharedBlockChain) -> P2P {
        let pool = CpuPool::new(config.threads);
        P2P {
            _context: Context::new(pool.clone(), handle.remote().clone(), config.clone(), db.clone()),
            _event_loop_handle: handle,
            _pool: pool,
        }
    }

    pub fn run(&self) -> Result<(), Error> { Ok(()) }
}

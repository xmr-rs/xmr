/// The Monero network we are in
#[derive(Debug, Clone, Copy)]
pub enum Network {
    /// The real world, the place where Monero coins have real world economic
    /// value.
    Mainnet,
    /// The main Monero testnet, the place where you can dream you are rich.
    Testnet,
}

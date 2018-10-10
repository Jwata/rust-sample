extern crate bitcoin_rpc;

use bitcoin_rpc:: {
    client,
    network,
    network::server
};

#[test]
fn connect_all() {
    client::connect();
    network::connect();
    server::connect();
}

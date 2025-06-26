use language_server::server::{LSPConnection, LSPServerManager};

use crate::{completion::std::StdCompletion, server::vrl::VRLServer};

mod ast;
mod completion;
mod diagnostics;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = LSPServerManager {
        server: VRLServer {
            connection: LSPConnection::new_network(4874),
            std_completion: StdCompletion::new(),
            ..Default::default()
        },
    };
    server.run().unwrap();
}

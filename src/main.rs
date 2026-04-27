

use dotenv::dotenv;
use env_logger;
use log::info;
mod full_indexing;

fn main() {
    full_indexing::start_indexing("/workspaces/mcp-semantic-search/src").unwrap();
}

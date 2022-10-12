use lpmng_mq::server::{AgentResponse, RouterRequest, Server};
use tokio;

#[tokio::main]
async fn main() {
    fn handler(_: RouterRequest) -> AgentResponse {
        AgentResponse {
            success: true,
            body: "".into(),
        }
    }

    let _ = Server::new("[::1]:8080", handler).serve().await;
}

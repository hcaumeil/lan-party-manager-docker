use agent::{router_client::RouterClient, PingRequest, RouterRequest};
use tonic::{transport::Channel, Request};

use self::agent::AgentResponse;

pub mod agent {
    tonic::include_proto!("agent");
}

pub struct Client {
    instance: RouterClient<Channel>,
}

impl Client {
    pub async fn connect(address: &str) -> Option<Self> {
        let instance = RouterClient::connect(String::from(address)).await;

        match instance {
            Ok(i) => Some(Client { instance: i }),
            Err(_) => None,
        }
    }

    pub async fn ping(&mut self) -> bool {
        let request = Request::new(PingRequest {
            body: "ping".into(),
        });

        let response = self.instance.ping(request).await;

        response
            .expect("rpc response has crashed")
            .into_inner()
            .success
    }

    pub async fn send(&mut self, request: RouterRequest) -> AgentResponse {
        self.instance
            .send(Request::new(request))
            .await
            .expect("rpc response has crashed")
            .into_inner()
    }
}

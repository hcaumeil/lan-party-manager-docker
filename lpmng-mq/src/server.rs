pub use agent::{
    router_server::{Router, RouterServer},
    AgentResponse, PingRequest, RouterRequest,
};
use tonic::{transport, Request, Response, Status};

pub mod agent {
    tonic::include_proto!("agent");
}

#[derive(Debug)]
pub struct RouterService {
    pub handler: fn(RouterRequest) -> AgentResponse,
}

impl Default for RouterService {
    fn default() -> Self {
        fn handler(_: RouterRequest) -> AgentResponse {
            AgentResponse {
                success: false,
                body: "Unimplemented Server Handler".into(),
            }
        }

        RouterService { handler }
    }
}

#[tonic::async_trait]
impl Router for RouterService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<AgentResponse>, Status> {
        if request.into_inner().body == "ping" {
            return Ok(Response::new(AgentResponse {
                success: true,
                body: "PONG !".into(),
            }));
        }

        Ok(Response::new(AgentResponse {
            success: false,
            body: "PONG ?".into(),
        }))
    }

    async fn send(
        &self,
        request: Request<RouterRequest>,
    ) -> Result<Response<AgentResponse>, Status> {
        Ok(Response::new((self.handler)(request.into_inner())))
    }
}

pub struct Server {
    address: String,
    handler: fn(RouterRequest) -> AgentResponse,
}

impl Server {
    pub fn new(address: &str, handler: fn(RouterRequest) -> AgentResponse) -> Self {
        Server {
            address: address.into(),
            handler,
        }
    }

    async fn _serve(&self, service: RouterService) -> Result<(), Box<dyn std::error::Error>> {
        let _ = transport::Server::builder()
            .add_service(RouterServer::new(service))
            .serve(self.address.parse().unwrap())
            .await?;
        Ok(())
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._serve(RouterService {
            handler: self.handler,
        })
        .await
    }

    pub async fn serve_default(&self) -> Result<(), Box<dyn std::error::Error>> {
        self._serve(RouterService::default()).await
    }
}

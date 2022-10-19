use std::net::IpAddr;
use std::{fs::File, net::Ipv4Addr};

use lpmng_mq::server::{AgentResponse, RouterRequest, Server};
use pftables_rs::{PfTable, PfrAddr};
use tokio;

fn ip_to_u32(str: String) -> Option<u32> {
    let mut res: u32 = 0;

    let split: Vec<&str> = str.split(".").collect();

    if split.len() != 4 {
        return None;
    }

    for i in 0..4 {
        res += (u8::from_str_radix(split[i], 10).ok()? as u32) << ((3 - i) * 4);
    }

    Some(res)
}

fn server_handler(req: RouterRequest) -> AgentResponse {
    match req.action.as_str() {
        "add" => {
            let ip = ip_to_u32(req.body.clone());

            if ip.is_none() {
                return AgentResponse::fail("unable to parse ip");
            }

            let mut ip_vec = Vec::new();
            ip_vec.push(PfrAddr::new(IpAddr::V4(Ipv4Addr::from(ip.unwrap())), 0));

            println!("[INFO] adding ip : {}", req.body);
            let _ = PfTable::new("authorized_users")
                .add_addrs(&File::open("/etc/authorized_users").unwrap(), ip_vec);
            AgentResponse::success()
        }
        _ => AgentResponse {
            success: false,
            body: "unknown method".into(),
        },
    }
}

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8080", server_handler);

    println!("[INFO] server has started");

    let _ = server.serve().await;
}

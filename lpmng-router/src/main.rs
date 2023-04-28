use std::net::IpAddr;
use std::{fs, net::Ipv4Addr};

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

            println!("[INFO] adding ip : {}", req.body);
            _ = std::process::Command::new("pfctl")
                .args(["-t", "authorized_users", "-T", "add", &req.body])
                .output();
            AgentResponse::success()
        }
        "remove" => {
            let ip = ip_to_u32(req.body.clone());

            if ip.is_none() {
                return AgentResponse::fail("unable to parse ip");
            }

            let mut ip_vec = Vec::new();
            ip_vec.push(PfrAddr::new(IpAddr::V4(Ipv4Addr::from(ip.unwrap())), 0));

            println!("[INFO] removing ip : {}", req.body);
            let _ = PfTable::new("authorized_users").del_addrs(
                &fs::OpenOptions::new().write(true).open("/dev/pf").unwrap(),
                ip_vec,
            );
            AgentResponse::success()
        }
        "get" => {
            println!("[INFO] getting ips");
            let ips = PfTable::new("authorized_users")
                .get_addrs(&fs::OpenOptions::new().write(true).open("/dev/pf").unwrap());

            let body = ips
                .unwrap()
                .into_iter()
                .map(|ip| ip.addr.to_string())
                .fold(String::from(""), |acc, e| acc + &e + "\n");

            AgentResponse {
                success: true,
                body,
            }
        }
        "clear" => {
            println!("[INFO] clearing ips");
            let _ = PfTable::new("authorized_users")
                .clr_addrs(&fs::OpenOptions::new().write(true).open("/dev/pf").unwrap());
            AgentResponse::success()
        }
        _ => AgentResponse {
            success: false,
            body: "unknown method".into(),
        },
    }
}

fn server_handler_test(req: RouterRequest) -> AgentResponse {
    match req.action.as_str() {
        "add" => {
            let ip = ip_to_u32(req.body.clone());

            if ip.is_none() {
                return AgentResponse::fail("unable to parse ip");
            }

            println!("[INFO] adding ip : {}", req.body);
            AgentResponse::success()
        }
        "remove" => {
            let ip = ip_to_u32(req.body.clone());

            if ip.is_none() {
                return AgentResponse::fail("unable to parse ip");
            }

            println!("[INFO] removing ip : {}", req.body);
            AgentResponse::success()
        }
        "get" => {
            println!("[INFO] getting ips");
            AgentResponse::success()
        }
        "clear" => {
            println!("[INFO] clearing ips");
            AgentResponse::success()
        }
        _ => AgentResponse {
            success: false,
            body: "unknown method".into(),
        },
    }
}

fn env_abort(env: &'static str) -> impl Fn(std::env::VarError) -> String {
    move |e| {
        eprintln!("[ERROR] ${env} is not set ({})", e);
        std::process::exit(1);
    }
}

fn env_get(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(env_abort(env))
}

#[tokio::main]
async fn main() {
    let router_address = env_get("ROUTER_ADDRESS");

    let server;
    if std::env::var("TEST").is_ok() {
        server = Server::new(&router_address, server_handler_test);
    } else {
        server = Server::new(&router_address, server_handler);
    }

    println!("[INFO] server has started");

    let _ = server.serve().await;
}

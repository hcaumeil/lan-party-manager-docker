use std::process::Stdio;

use lpmng_mq::server::{AgentResponse, RouterRequest, Server};
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
            _ = std::process::Command::new("iptables")
                .args(["-A", "LPMNG", "-s", &req.body, "-i", "eth1", "-j", "ACCEPT"])
                .output();
            AgentResponse::success()
        }
        "remove" => {
            let ip = ip_to_u32(req.body.clone());

            if ip.is_none() {
                return AgentResponse::fail("unable to parse ip");
            }

            println!("[INFO] removing ip : {}", req.body);
            _ = std::process::Command::new("iptables")
                .args(["-D", "LPMNG", "-s", &req.body, "-i", "eth1", "-j", "ACCEPT"])
                .output();

            AgentResponse::success()
        }
        "get" => {
            println!("[INFO] getting ips");

            let ips = std::process::Command::new("iptables")
                .args(["-L", "LPMNG", "-vn"])
                .stdout(Stdio::piped())
                .spawn();

            if ips.is_err() {
                return AgentResponse::fail("no output");
            }

            let ips = ips.unwrap();
            let res = std::process::Command::new("grep")
                .args(["-oh", "10.82.\\w*.\\w*"])
                .stdin(ips.stdout.unwrap())
                .output();

            match res {
                Ok(r) => {
                    let body = String::from_utf8_lossy(&r.stdout).to_string();

                    AgentResponse {
                        success: true,
                        body,
                    }
                }
                Err(_) => AgentResponse::fail("no output"),
            }
        }
        "clear" => {
            println!("[INFO] clearing ips");
            _ = std::process::Command::new("pfctl")
                .args(["-t", "authorized_users", "-T", "flush"])
                .output();
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

use super::db::DbHandler;
use lpmng_mq::client::agent::{AgentResponse, RouterRequest};
use lpmng_mq::client::Client;

use std::io::Write;

pub struct ConsoleHandler {
    pub db_handler: Option<DbHandler>,
    pub router_address: String,
    pub router: Option<Client>,
}

fn help() {
    println!("help : show this help");
    println!("exit : exit the console");
    println!("rc / router-connect : connect to the router ");
    println!("rp / router-ping : ping the router");
    println!("dbc / db-connect : connect to the database");
    println!("");
}

async fn router_connect(handler: &mut ConsoleHandler) {
    handler.router = Client::connect(&handler.router_address).await;

    if handler.router.is_some() {
        println!("Router successfully connected !");
    }
}

async fn router_ping(handler: &mut ConsoleHandler) {
    if handler.router.is_some() {
        if handler.router.as_mut().unwrap().ping().await {
            println!("Successfull PONG!");
        }
    } else {
        eprintln!("There is no connection to the router, try command 'rc'");
    }
}

async fn router_add(handler: &mut ConsoleHandler, args: &[&str]) {
    if handler.router.is_some() {
        if args.len() > 0 {
            let res = handler
                .router
                .as_mut()
                .unwrap()
                .send(RouterRequest {
                    action: "add".to_owned(),
                    body: args[0].to_owned(),
                })
                .await;
            if res.success {
                println!("ip has been added !");
            } else {
                eprintln!("router error: {}", res.body);
            }
        }
    } else {
        eprintln!("There is no connection to the router, try command 'rc'");
    }
}

async fn db_connect(handler: &mut ConsoleHandler) {
    handler.db_handler = DbHandler::connect().await;

    if handler.db_handler.is_some() {
        println!("Database successfully connected !");
    }
}

pub async fn console(mut handler: ConsoleHandler) {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    if handler.router.is_some() {
        println!("Router successfully connected !");
    }

    if handler.db_handler.is_some() {
        println!("Database successfully connected !");
    }

    println!("[GREET] Time for C-sides !");
    loop {
        let mut buff = String::new();
        print!("lpmng > ");
        _ = stdout.flush();
        _ = stdin.read_line(&mut buff);

        buff.retain(|c| c != '\n');
        let args: Vec<&str> = buff.split_whitespace().collect();

        match *args.first().unwrap() {
            "help" => help(),
            "exit" => return,
            "rc" | "router-connect" => router_connect(&mut handler).await,
            "rp" | "router-ping" => router_ping(&mut handler).await,
            "radd" | "router-add" => router_add(&mut handler, &args[1..]).await,
            "dbc" | "db-connect" => db_connect(&mut handler).await,
            _ => eprintln!("error: this command does not exist"),
        }
    }
}

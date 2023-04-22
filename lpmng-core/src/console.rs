use super::db::DbHandler;
use lpmng_mq::client::agent::RouterRequest;
use lpmng_mq::client::Client;

use std::io::Write;

pub struct ConsoleHandler {
    pub db_handler: Option<DbHandler>,
    pub router_address: String,
    pub router: Option<Client>,
}

pub static BANNER: &str = "
,_____,
| === |    [::::::],     ,-#,    #-----[_]
| === |    [::::::] \\   /  | \\   |
| === |     |    |   '-'   |  \\  |  [_]-#
| === |     |    |         |   \\ |      |
|_____|-----#    @        [_]   '#------#
";

fn help() {
    println!("help : show this help");
    println!("exit : exit the console");
    println!("rc / router-connect : connect to the router ");
    println!("rp / router-ping : ping the router");
    println!("radd / router-add [ipv4] : allow an ip address");
    println!("rrm / router-remove [ipv4] : remove an ip address");
    println!("dbc / db-connect : connect to the database");
    println!("dbu / db-users : get users from the database");
    println!("banner : print banner");
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

async fn router_ip_action(
    handler: &mut ConsoleHandler,
    args: &[&str],
    action: &str,
    success_msg: &str,
) {
    if handler.router.is_some() {
        if args.len() > 0 {
            let res = handler
                .router
                .as_mut()
                .unwrap()
                .send(RouterRequest {
                    action: action.to_owned(),
                    body: args[0].to_owned(),
                })
                .await;
            if res.success {
                println!("{}", success_msg);
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

async fn db_get_users(handler: &mut ConsoleHandler) {
    handler.db_handler = DbHandler::connect().await;

    if handler.db_handler.is_some() {
        let users = handler.db_handler.as_mut().unwrap().get_users().await;

        if users.is_some() {
            println!("username firstname lastname role is_allowed");
            println!("-----");

            for u in users.unwrap() {
                println!(
                    "{} {} {} {} {}",
                    u.username, u.firstname, u.lastname, u.role, u.is_allowed
                );
            }
        }
    } else {
        eprintln!("There is no connection to the database, try command 'rdb'");
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
            "radd" | "router-add" => {
                router_ip_action(&mut handler, &args[1..], "add", "ip successfully added !").await
            }
            "rrm" | "router-remove" => {
                router_ip_action(
                    &mut handler,
                    &args[1..],
                    "remove",
                    "ip successfully removed !",
                )
                .await
            }
            "dbc" | "db-connect" => db_connect(&mut handler).await,
            "dbu" | "db-users" => db_get_users(&mut handler).await,
            "banner" => println!("{}", BANNER),
            _ => eprintln!("error: this command does not exist"),
        }
    }
}

use axum::{routing::get, Router};
use std::{
    env,
    str,
    net::SocketAddr,
};

mod config;

#[tokio::main]
async fn main() {
    //Get config file
    let env_args: Vec<String> = env::args().collect();
    let config_path = match env_args.len() {
        1 => String::from("config.toml"),
        2 => env_args[1].clone(),
        _ => panic!("Too many arguments"),
    };
    let cfg = config::config::Config::new(config_path);

    //Set up app and listener
    let app: Router;
    let addr: SocketAddr;

    {
        let mut split = cfg.listen_on.split(":");

        //Parse out IP (default to 127.0.0.1)
        let ip: [u8; 4] = match split.next().unwrap() {
            "localhost" => [127, 0, 0, 1],
            s => match s.split(".") {
                parts => {
                    let mut ip: [u8; 4] = [0, 0, 0, 0];
                    for (i, part) in parts.enumerate() {
                        ip[i] = part.parse::<u8>().unwrap();
                    }
                    ip
                }
            }
        };

        //Set port (default to 3000)
        let port = match split.next() {
            Some(p) => match p.parse::<u16>() {
                Ok(p) => p,
                Err(_) => 3000,
            },
            None => 3000,
        };

        //Set up app
        app = Router::new().route("/", get(handler));
        addr = SocketAddr::from((ip, port));
    }

    //Set up server
    let addr = SocketAddr::from(addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
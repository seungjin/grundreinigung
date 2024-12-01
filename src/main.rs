use anyhow::{Error, Result};
use clap::Parser;
use command::uptime::Uptime;
use openssh::{KnownHosts, Session};
use std::sync::Arc;

use server::Server;

mod command;
mod server;

const fn version_string() -> &'static str {
    concat!(
        env!("GIT_HASH"),
        "(mini hash), ",
        env!("GIT_BRANCH"),
        "(branch)\nCommit at ",
        env!("GIT_COMMIT_DATE"),
        "\nBuild at ",
        env!("BUILD_TIMESTAMP")
    )
}

#[derive(Debug, Parser)]
#[command(version = version_string())]
struct Cli {
    #[arg(short, long)]
    port: Option<usize>,

    #[arg(short, long)]
    login: Option<String>,

    #[arg(required = true)]
    host_addr: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let uri = get_uri(cli);
    println!("Connecting {}", uri);

    let s = Server::check(0, uri.leak()).await;

    let a = Uptime::exec(s).await;
    println!("{:?}", a);

    //Apt::update(s.clone(), true)
    //.await
    //.map_err(anyhow::Error::from);

    //println!("{:?}", cli);
}

fn get_uri(cli: Cli) -> String {
    let port = match cli.port {
        None => 22,
        Some(p) => p,
    };

    let login = match cli.login {
        None => "root".to_string(),
        Some(l) => l,
    };

    format!("ssh://{}@{}:{}", login, cli.host_addr, port)
}

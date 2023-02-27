use clap::Parser;
use client::{GameClient, Host, SocketLogger};
use std::net::ToSocketAddrs;

mod args;
mod ascii;
mod client;
mod game;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let mut host = Host::new();

    if !args.host && args.connect.eq("") {
        panic!("parameter error");
    }

    match args.host {
        true => {
            host.log_listen_socket();
            host.wait_hand_shake();
        }
        false => {
            let addr = args
                .connect
                .as_str()
                .to_socket_addrs()?
                .next()
                .expect("is not invalid ip");
            host.set_addr(addr);
            host.send_hand_shake();
        }
    };

    host.log_on_hand_shake();
    host.choice();
    host.judge();

    Ok(())
}

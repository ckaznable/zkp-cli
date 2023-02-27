use std::{net::{SocketAddr, UdpSocket, Ipv4Addr, IpAddr}, sync::{Mutex, Arc}, thread::{JoinHandle, self}};

use crate::{game::{Game, GameResult}, ascii::{print_ascii, str_to_enum, enum_to_str}};

pub trait GameClient {
    fn game(&self) -> &Game;

    fn judge(&self) {
        let game = self.game();
        print_ascii(&game.rival, true);
        println!("");
        print_ascii(&game.hand, false);
        match game.judge() {
            GameResult::Draw => println!("Draw"),
            GameResult::Win => println!("You Win"),
            GameResult::Lose => println!("You Lose"),
        };
    }
}

pub trait SocketLogger {
    fn socket(&self) -> &Arc<Mutex<UdpSocket>>;
    fn addr(&self) -> &SocketAddr;

    fn log_listen_socket(&self) {
        let socket = self.socket();
        let socket = socket.lock().unwrap();
        let local_addr = socket.local_addr();
        let addr = local_addr.unwrap();
        println!("listening {}:{}", addr.ip(), addr.port());
    }

    fn log_on_hand_shake(&self) {
        let addr = self.addr();
        println!("{}:{} connected", addr.ip(), addr.port());
    }
}

pub struct Host {
    game: Game,
    socket: Arc<Mutex<UdpSocket>>,
    addr: SocketAddr,
}

impl SocketLogger for Host {
    fn socket(&self) -> &Arc<Mutex<UdpSocket>> {
        &self.socket
    }

    fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}

impl GameClient for Host {
    fn game(&self) -> &Game {
        &self.game
    }
}

impl Host {
    pub fn new() -> Host {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");

        Host {
            game: Game::default(),
            socket: Arc::new(Mutex::new(socket)),
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }

    pub fn set_addr(&mut self, addr: SocketAddr) {
        self.addr = addr;
    }

    pub fn wait_hand_shake(&mut self) {
        let socket = Arc::clone(&self.socket);
        let socket = socket.lock().unwrap();

        let mut buffer = [0u8; 1024];
        let (_, addr) = socket.recv_from(&mut buffer).expect("Could not receive data");
        self.addr = addr;
    }

    pub fn send_hand_shake(&self) {
        let socket = Arc::clone(&self.socket);
        let socket = socket.lock().unwrap();

        let data = "hand shake".as_bytes();
        socket.send_to(data, self.addr).expect("Could not send data");
    }

    pub fn choice(&mut self) {
        let handle = self.wait_target_choice();
        self.game.choice().unwrap();
        self.send_choice();
        self.game.rival_hand(str_to_enum(handle.join().unwrap().as_str()));
    }

    fn wait_target_choice<'a>(&self) -> JoinHandle<String> {
        let socket = Arc::clone(&self.socket);
        thread::spawn(move || {
            let socket = socket.lock().unwrap();
            let mut buffer = [0u8; 1024];
            let (len, _) = socket.recv_from(&mut buffer).expect("Could not receive data");
            std::str::from_utf8(&buffer[..len]).expect("Invalid UTF-8 string").to_string()
        })
    }

    fn send_choice(&self) {
        let choice = enum_to_str(self.game.hand);
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");

        let data = choice.as_bytes();
        socket.send_to(data, self.addr).expect("Could not send data");
    }
}
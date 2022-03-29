use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener};

pub struct RobotClient {
    endpoint: Endpoint
}

impl RobotClient {
    pub fn connect() -> Self {
        let (handler, listener) = node::split::<()>();

        let (endpoint, _) = handler.network().connect(Transport::FramedTcp, "127.0.0.1:3042").unwrap();

        println!("Connecting...");

        listener.for_each(move |event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(endpoint, result) => {
                    if result {
                        println!("Connected to {:?}", endpoint);
                        handler.stop();
                    } else {
                        println!("Connecting...");
                        std::thread::sleep(Duration::from_millis(200));
                        handler.network().connect(Transport::FramedTcp, "127.0.0.1:3042").unwrap();
                    }
                }
                _ => {}
            }
            _ => {}
        });

        let (handler, listener) = node::split::<()>();
        let status = handler.network().send(endpoint, b"Hello?");
        println!("{:?}", status);

        Self {
            endpoint
        }
    }

    pub fn gen_trajectory(&self) {
        let (handler, listener) = node::split::<()>();
        println!("{:?}", self.endpoint);
        let status = handler.network().send(self.endpoint, b"Hello World");
        println!("{:?}", status);
    }
}
use message_io::network::{NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeListener};

enum Signal {
    Test
}

pub fn connect() {
    let (handler, listener) = node::split::<Signal>();
    let (server, _) = handler.network().connect(Transport::FramedTcp, "127.0.0.1:3042").unwrap();

    println!("Connecting...");

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(endpoint, result) => {
                if result {
                    println!("Connected to {:?}!", endpoint);
                    handler.stop();
                } else {
                    println!("Connection to {:?} failed.", endpoint);
                }
            }
            NetEvent::Accepted(_, _) => {}
            NetEvent::Message(_, _) => {}
            NetEvent::Disconnected(_) => {}
        }
        NodeEvent::Signal(signal) => match signal {
            Signal::Test => {}
        }
    });
}
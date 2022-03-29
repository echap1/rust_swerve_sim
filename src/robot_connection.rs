use std::io::{Read, Write};
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener};
use uom::ConstZero;
use uom::si::angle::Angle;
use uom::si::f32::Length;
use crate::auto_pathing::trajectory::Trajectory;
use crate::field::{FieldPose, FieldPosition};

pub struct RobotClient {
    stream: TcpStream
}

impl RobotClient {
    pub fn connect() -> Self {
        let stream: TcpStream;

        loop {
            match TcpStream::connect("127.0.0.1:65426") {
                Ok(s) => {
                    stream = s;
                    break;
                }
                Err(_) => {
                    println!("Connecting...");
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
        }

        println!("Connected!");

        Self {
            stream
        }
    }

    pub fn gen_trajectory(&mut self, trajectory: &Trajectory) -> Vec<FieldPosition> {
        self.stream.write(&serde_json::to_vec(&trajectory).unwrap()).unwrap();
        let mut buf = [0; 65536];
        let length = self.stream.read(&mut buf).unwrap();
        let res = String::from_utf8_lossy(&buf[0..length]);
        // println!("{}", res);
        serde_json::from_str(&res).unwrap()
    }
}
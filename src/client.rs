use std::collections::HashMap;
use std::time::SystemTime;

use crossbeam::channel::{self, Sender};
use tokio::net::TcpStream;

use crate::record::Record;
use crate::worker::{Message, Worker};

pub struct Client {
    sender: Sender<Message>,
}

impl Client {
    pub async fn new() -> tokio::io::Result<Client> {
        let socket = TcpStream::connect("127.0.0.1:24224").await?;
        let (sender, receiver) = channel::unbounded();

        let _ = tokio::spawn(async move {
            let mut worker = Worker::new(socket, receiver);
            worker.run().await
        });

        Ok(Client { sender })
    }

    pub fn send(
        &self,
        tag: &'static str,
        entry: HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let record = Record {
            tag,
            entry,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
        };
        self.sender.send(Message::Record(record))?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), channel::SendError<Message>> {
        self.sender.send(Message::Terminate)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.sender.send(Message::Terminate);
    }
}
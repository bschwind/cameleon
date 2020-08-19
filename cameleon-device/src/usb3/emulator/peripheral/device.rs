use std::{convert::TryInto, sync::Arc, time};

use async_std::{
    sync::{channel, Mutex, Receiver, Sender},
    task,
};
use futures::channel::oneshot;

use super::{fake_protocol::*, memory::Memory};

const REQ_PACKET_CHANNEL_CAPACITY: usize = 1;
const ACK_PACKET_CHANNEL_CAPACITY: usize = 1;

pub(super) struct Device {
    timestamp: Timestamp,
    memory: Arc<Mutex<Memory>>,
    tx_for_host: Option<Sender<FakeReqPacket>>,
    rx_for_host: Option<Receiver<FakeAckPacket>>,
}

impl Device {
    pub(super) fn new(memory: Memory) -> Self {
        Self {
            timestamp: Timestamp::new(),
            memory: Arc::new(Mutex::new(memory)),
            tx_for_host: None,
            rx_for_host: None,
        }
    }

    pub(super) fn run(&mut self) {
        // Create channels for communication between device and host.
        let (req_tx_for_host, req_rx_for_device) = channel(REQ_PACKET_CHANNEL_CAPACITY);
        let (ack_tx_for_device, ack_rx_for_host) = channel(ACK_PACKET_CHANNEL_CAPACITY);
        self.tx_for_host = Some(req_tx_for_host);
        self.rx_for_host = Some(ack_rx_for_host);

        todo!();
    }
}

#[derive(Debug, Clone)]
pub(super) struct Timestamp(Arc<Mutex<time::Instant>>);

impl Timestamp {
    pub(super) fn new() -> Self {
        Self(Arc::new(Mutex::new(time::Instant::now())))
    }

    pub(super) async fn as_nanos(&self) -> u64 {
        let mut inner = self.0.lock().await;
        let ns: u64 = match inner.elapsed().as_nanos().try_into() {
            Ok(time) => time,
            Err(_) => {
                *inner = time::Instant::now();
                inner.elapsed().as_nanos() as u64
            }
        };
        ns
    }
}

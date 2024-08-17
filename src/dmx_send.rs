use dmx::{self, DmxTransmitter};
use std::sync::mpsc::Receiver;
use std::{thread, time};

pub fn spawn_receiver(rx: Receiver<Vec<u8>>) {
    thread::spawn(move || {
        let mut dmx_port = dmx::open_serial("/dev/serial0").unwrap();

        let mut previous_data: Vec<u8> = vec![0x00, 0x00];

        // we want this thread to be sending packets for the lifetime of the app
        let mut j = 0;
        loop {
            j += 1;
            println!("hi number {} from the spawned thread", j);

            match rx.try_recv() {
                Ok(data) => {
                    previous_data = data.clone();
                    dmx_port.send_dmx_packet(&data).unwrap();
                }
                Err(_e) => {
                    dmx_port.send_dmx_packet(&previous_data).unwrap();
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        }
    });
}

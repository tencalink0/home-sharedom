use tokio::runtime;
use bluez_async::BluetoothSession;
use std::time::Duration;
use tokio::time;

//bluez and bluez-async are linux only so alternatives will have to be found

// from: https://github.com/bluez-rs/bluez-async/blob/main/bluez-async/examples/devices.rs
pub struct BluetoothServer {
    name: &'static str,
    scan_time: Duration
}

impl BluetoothServer {
    pub fn new(name: &'static str) -> BluetoothServer {
        BluetoothServer { 
            name,
            scan_time: Duration::from_secs(5)
        }
    }

    pub async fn begin(&self) -> Result<(), eyre::Report> {
        let (_, session) = BluetoothSession::new().await?;

        session.start_discovery().await?;
        time::sleep(self.scan_time).await;
        session.stop_discovery().await?;
        
        let devices = session.get_devices().await?;
        println!("Devices {:?}", devices);

        Ok(())
    }
}
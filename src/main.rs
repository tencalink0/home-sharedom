mod web_server;
mod bluetooth;
use web_server::WebServer;
use bluetooth::BluetoothServer;

#[tokio::main]
async fn main() {
    let web_server = WebServer::new([127, 0, 0, 1], 3000);
    let bluetooth_server = BluetoothServer::new("Sharedom-Server");
    
    web_server.begin().await;
}
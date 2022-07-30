// secrets.rs - stores WiFi secrets like SSID, passphrase, etc
//

use heapless::String;

const SSID: &str = "borys2g";
const PASSPHRASE: &str = "75884345";

// Non-production Ambi server IP address/port pair for local network development
//192.168.0.101
const NON_PROD_AMBI_IP: [u8; 4] = [192, 168, 0, 101];
const NON_PROD_AMBI_PORT: u16 = 8000;
// const NON_PROD_AMBI_PORT: u16 = 4000;

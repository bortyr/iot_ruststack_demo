# IoT Ruststack Demo

Showcase of a demo fullstack project intended for IoT/Embedded domain with using Rust programming language and it's current popular frameworks.

Short description: Raspberry Pi Pico sends measurement data as an HTTP POST request through Pimoroni Wireless Module (based on ESP32) that goes to backend REST API server 
that saves the data to PostgreSQL database and then serves this data to frontend server.

![image](https://user-images.githubusercontent.com/62113056/181935783-d6a10aab-c281-4679-a069-ca2087804cb9.png)

<img width="1440" alt="iot_ruststack_demo_showcase" src="https://user-images.githubusercontent.com/62113056/181935691-b2aee251-56da-46ca-91ad-9165c2a94fd4.png">

## Frontend 

Written in [Yew](https://yew.rs/). Used [Bulma](https://bulma.io/) CSS framework for styling.

## Backend

Written in [Rocket](https://rocket.rs/). Used [PostgreSQL](https://www.google.com/search?client=firefox-b-d&q=postgrsql) as database.

## Embedded

Used [Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/) with [Pimoroni Pico Wireless Pack](https://shop.pimoroni.com/products/pico-wireless-pack?variant=32369508581459) 
which uses ESP32 to provide Pico Wi-Fi connectivity. BME280 sensor is used for temperature measurment if possible - if not, value 23.0 is sent.

Drivers and whole software have been written in Rust thanks to guys at [Rust Never Sleeps](https://github.com/Jim-Hodapp-Coaching) group - check them out!

## TODO

- Finish Tauri deployment
  - Provide correct CORS in Rocket. This will allow tauri app to fetch data from Rocket server without issues

# Setup

In frontend run:

```sh
trunk serve
```

In backend run:

```sh
ROCKET_ENV=stage cargo run
```

In embedded:

- Provide your Wi-Fi SSID and Passphrase in secrets.rs
- Flash the Raspberry Pi Pico with 

```sh
cargo run
```

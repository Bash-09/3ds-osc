use std::net::UdpSocket;

use ctru::prelude::*;
use rosc::{OscMessage, OscPacket, OscType};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());
    let _soc = Soc::new().unwrap();

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {info}");

        unsafe {
            loop {
                ctru_sys::hidScanInput();
                let keys = ctru_sys::hidKeysDown();
                if KeyPad::from_bits_truncate(keys).contains(KeyPad::START) {
                    break;
                }
            }
        }
    }));

    println!("Hello, World!");
    println!("\x1b[29;16HPress Start to exit");

    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    socket.connect("255.255.255.255:8081").unwrap();

    let (mut last_touch_x, mut last_touch_y) = (0, 0);
    let (mut last_circle_x, mut last_circle_y) = (0, 0);

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        let mut packets = vec![];

        // Touch
        let (touch_x, touch_y) = hid.touch_position();
        if touch_x != 0 && touch_y != 0 && (touch_x != last_touch_x || touch_y != last_touch_y) {
            last_touch_x = touch_x;
            last_touch_y = touch_y;
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/touch"),
                args: vec![OscType::Int(touch_x as i32), OscType::Int(touch_y as i32)],
            }));
        }

        // Circle
        let (circle_x, circle_y) = hid.circlepad_position();
        if circle_x != last_circle_x || circle_y != last_circle_y {
            last_circle_x = circle_x;
            last_circle_y = circle_y;
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/circle"),
                args: vec![OscType::Int(circle_x as i32), OscType::Int(circle_y as i32)],
            }));
        }

        // Buttons
        if hid.keys_down().contains(KeyPad::A) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/a"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::B) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/b"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::X) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/x"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::Y) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/y"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::DPAD_LEFT) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/left"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::DPAD_RIGHT) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/right"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::DPAD_UP) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/up"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::DPAD_DOWN) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/down"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::L) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/l"),
                args: vec![],
            }));
        }

        if hid.keys_down().contains(KeyPad::R) {
            packets.push(OscPacket::Message(OscMessage {
                addr: String::from("/3ds/r"),
                args: vec![],
            }));
        }

        for p in packets {
            let bytes = rosc::encoder::encode(&p).unwrap();
            socket.send(&bytes).unwrap();
        }
    }
}

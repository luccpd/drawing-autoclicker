use image::*;
use std::{path::Path, thread, time};
use windows::{Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    loop {
        unsafe {
            println!("{:?}", GetKeyState(0x41));
            if GetKeyState(0x41) == 1 {
                cursor_image_traversal();
                press_key(VIRTUAL_KEY(0x41));
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

fn press_key(key: VIRTUAL_KEY) {
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        let mut input: [INPUT; 1] = std::mem::zeroed();
        input[0].r#type = INPUT_KEYBOARD;
        input[0].Anonymous.ki.wVk = key;
        input[0].Anonymous.ki.wScan = 0;
        input[0].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);
        input[0].Anonymous.ki.time = 0;
        input[0].Anonymous.ki.dwExtraInfo = 0;
        SendInput(&mut input, ipsize);
    }
}

fn mouse_click(x: i32, y: i32) {
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        let mut input: [INPUT; 2] = std::mem::zeroed();
        input[0].r#type = INPUT_MOUSE;
        input[0].Anonymous.mi.dx = x;
        input[0].Anonymous.mi.dy = y;
        input[0].Anonymous.mi.dwExtraInfo = 0;
        input[0].Anonymous.mi.time = 0;
        input[0].Anonymous.mi.mouseData = 0;
        input[0].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0002);

        input[1].r#type = INPUT_MOUSE;
        input[1].Anonymous.mi.dx = x;
        input[1].Anonymous.mi.dy = y;
        input[1].Anonymous.mi.dwExtraInfo = 0;
        input[1].Anonymous.mi.time = 0;
        input[1].Anonymous.mi.mouseData = 0;
        input[1].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0004);

        SendInput(&mut input, ipsize);
    }
}

fn cursor_image_traversal() {
    let windows_path = Path::new("img/test.jpg");
    let itt = image::open(windows_path).unwrap();

    for i in itt.pixels() {
        //println!("{:?}",i);
        unsafe {
            if GetKeyState(0x41) == 0 {
                break;
            }

            if i.2 == Rgba([255, 255, 255, 255]) {
                let offset_x = i.0 + 100;
                let offset_y = i.1 + 300;
                SetCursorPos(offset_x.try_into().unwrap(), offset_y.try_into().unwrap());
                mouse_click(offset_x.try_into().unwrap(), offset_y.try_into().unwrap());
                thread::sleep(time::Duration::from_nanos(1));
            }
        }
    }
}

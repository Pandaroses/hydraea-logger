#![feature(iter_map_windows)]
use input::event::keyboard::KeyboardEventTrait;
use input::event::tablet_pad::KeyState;
use input::{Event, Libinput, LibinputInterface};
use libc::{epoll_wait, fseek, FILE, O_RDONLY, O_RDWR, O_WRONLY};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, AsRawFd, FromRawFd};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

struct Interface;

const FILEPATH: &str = "/home/gsh/data.flop";

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct LogFile {
    bigrams: HashMap<(u32, u32), u32>,
    trigrams: HashMap<(u32, u32, u32), u32>,
    quadrams: HashMap<(u32, u32, u32, u32), u32>,
}

fn calculate(buf: [u32; 1024]) {
    println!("{}", buf[0]);
    let bigrams: Vec<(&u32, &u32)> = buf.iter().map_windows(|&[x, y]| (x, y)).collect();
    let trigrams: Vec<(&u32, &u32, &u32)> =
        buf.iter().map_windows(|&[x, y, z]| (x, y, z)).collect();
    let quadrams: Vec<(&u32, &u32, &u32, &u32)> = buf
        .iter()
        .map_windows(|&[x, y, z, i]| (x, y, z, i))
        .collect();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(FILEPATH)
        .unwrap();
    let mut output = vec![];
    file.read_to_end(&mut output).unwrap();
    let mut load: LogFile = bincode::deserialize(&output).unwrap_or_default();

    bigrams
        .iter()
        .for_each(|i| *load.bigrams.entry((*i.0, *i.1)).or_insert(1) += 1);
    trigrams
        .iter()
        .for_each(|i| *load.trigrams.entry((*i.0, *i.1, *i.2)).or_insert(1) += 1);
    quadrams
        .iter()
        .for_each(|i| *load.quadrams.entry((*i.0, *i.1, *i.2, *i.3)).or_insert(1) += 1);

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    let writer = bincode::serialize(&load).unwrap();
    file.write_all(&writer).unwrap();
}

fn main() {
    let mut shifted = false;
    let mut capped = false;
    let mut buf: [u32; 1024] = [0; 1024];
    let mut count = 0;
    let mut input = Libinput::new_with_udev(Interface);
    let input_fd = input.as_raw_fd();
    let input_file = unsafe { File::from_raw_fd(input_fd) };
    let mut fds = [nix::poll::PollFd::new(
        &input_file,
        nix::poll::PollFlags::POLLIN,
    )];
    input.udev_assign_seat("seat0").unwrap();
    loop {
        nix::poll::poll(&mut fds, 1000).unwrap();
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                Event::Keyboard(e) => {
                    match e.key_state() {
                        KeyState::Released => match e.key() {
                            58 => capped = !capped,
                            42 => shifted = false,
                            54 => shifted = false,
                            _ => {
                                if capped ^ shifted {
                                    //HANDLES UPPERCASE
                                    match e.key() {
                                        //MACRO KEY 1 = ~
                                        41 => buf[count] = 656,
                                        //MACRO KEY 2 = !
                                        2 => buf[count] = 657,
                                        //MACRO KEY 3 = @
                                        3 => buf[count] = 658,
                                        //MACRO KEY 4 = $
                                        4 => buf[count] = 659,
                                        //MACRO KEY 5 = %
                                        5 => buf[count] = 660,
                                        //MACRO KEY 6 = ^
                                        6 => buf[count] = 661,
                                        //MACRO KEY 7 = &
                                        7 => buf[count] = 662,
                                        //MACRO KEY 8 = *
                                        8 => buf[count] = 663,
                                        //MACRO KEY 9 = (
                                        9 => buf[count] = 664,
                                        //MACRO KEY 10 = )
                                        10 => buf[count] = 665,
                                        //MACRO KEY 11 = _
                                        12 => buf[count] = 666,
                                        //NUMPAD +
                                        13 => buf[count] = 78,
                                        //MACRO KEY 12 = {
                                        26 => buf[count] = 667,
                                        //MACRO KEY 13 = }
                                        27 => buf[count] = 668,
                                        //MACRO KEY 1024 = |
                                        43 => buf[count] = 669,
                                        //MACRO KEY 15 = :
                                        39 => buf[count] = 670,
                                        //MACRO KEY 16 = "
                                        40 => buf[count] = 671,
                                        //MACRO KEY 17 = <
                                        51 => buf[count] = 672,
                                        //MACRO KEY 18 = >
                                        52 => buf[count] = 673,
                                        //MACRO KEY 19 = ?
                                        53 => buf[count] = 674,
                                        _ => buf[count] = e.key(),
                                    }
                                } else {
                                    //HANDLES LOWERCASE
                                    buf[count] = e.key();
                                };
                                count += 1;
                            }
                        },
                        KeyState::Pressed => match e.key() {
                            42 | 54 => {
                                shifted = true;
                                count += 1
                            }
                            _ => {}
                        },
                    };
                    if count >= 1024 {
                        count = 0;
                        calculate(buf);
                        buf = [0; 1024];
                    }
                }
                _ => {}
            }
        }
    }
}

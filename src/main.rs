use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::io::Read;

use crate::instructions::read_instruction;

mod instructions;

#[derive(Debug)]
struct Chip8 {
    memory: Vec<u8>,
    display: Vec<Vec<u32>>,
    program_counter: u16,
    index_register: usize,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_register: Vec<u8>,
    increment_pc: bool,
    key_pressed: u8,
}

impl Chip8 {
    fn load_rom(&mut self, file_path: &str) {
        let mut f = fs::File::open(file_path).expect("no file found");
        let metadata = fs::metadata(file_path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");

        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
    }

    fn init() -> Chip8 {
        let mut initalized = Chip8 {
            memory: vec![0; 4096],
            display: vec![vec![0x00382b26; 64]; 32],
            program_counter: 512,
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_register: vec![0; 16],
            increment_pc: true,
            key_pressed: 0,
        };
        initalized.init_font_in_memory();

        initalized
    }

    fn init_font_in_memory(&mut self) {
        let font_set = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, &byte) in font_set.iter().enumerate() {
            self.memory[i + 80] = byte;
        }
    }

    fn get_display_row(&self, y: u16) -> Vec<u32> {
        self.display[y as usize].clone()
    }
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut emu = Chip8::init();
    emu.load_rom("/home/akira/Documents/coding/SIIRUP/roms/ibm.ch8");

    let mut window = Window::new(
        "SIIRUP",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..12 {
            let opcode = (emu.memory[emu.program_counter as usize] as u16) << 8
                | emu.memory[(emu.program_counter + 1) as usize] as u16;

            read_instruction(&mut emu, opcode);

            if emu.increment_pc {
                emu.program_counter += 2;
            }

            emu.increment_pc = true;
        }

        if emu.delay_timer > 0 {
            emu.delay_timer -= 1;
        }
        if emu.sound_timer > 0 {
            emu.sound_timer -= 1;
        }

        if window.is_key_down(Key::Key1) {
            emu.key_pressed = 0x1
        } else if window.is_key_down(Key::Key2) {
            emu.key_pressed = 0x2
        } else if window.is_key_down(Key::Key3) {
            emu.key_pressed = 0x3
        } else if window.is_key_down(Key::Key4) {
            emu.key_pressed = 0xC
        } else if window.is_key_down(Key::Q) {
            emu.key_pressed = 0x4
        } else if window.is_key_down(Key::W) {
            emu.key_pressed = 0x5
        } else if window.is_key_down(Key::E) {
            emu.key_pressed = 0x6;
        } else if window.is_key_down(Key::R) {
            emu.key_pressed = 0xD;
        } else if window.is_key_down(Key::A) {
            emu.key_pressed = 0x7;
        } else if window.is_key_down(Key::S) {
            emu.key_pressed = 0x8;
        } else if window.is_key_down(Key::D) {
            emu.key_pressed = 0x9;
        } else if window.is_key_down(Key::F) {
            emu.key_pressed = 0xE
        } else if window.is_key_down(Key::Z) {
            emu.key_pressed = 0xA
        } else if window.is_key_down(Key::X) {
            emu.key_pressed = 0x0
        } else if window.is_key_down(Key::C) {
            emu.key_pressed = 0xB
        } else if window.is_key_down(Key::V) {
            emu.key_pressed = 0xF
        } else {
            emu.key_pressed = 17;
        }

        let mut display_buffer = vec![0; WIDTH * HEIGHT];
        for y in 0..HEIGHT {
            let row = emu.get_display_row(y as u16);
            for x in 0..WIDTH {
                display_buffer[y * WIDTH + x] = row[x];
            }
        }

        window
            .update_with_buffer(&display_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

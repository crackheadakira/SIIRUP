use crate::Chip8;
use rand::Rng;

pub fn read_instruction(emulator: &mut Chip8, opcode: u16) {
    let x_nibble = ((opcode & 0x0F00) >> 8) as usize;
    let y_nibble = ((opcode & 0x00F0) >> 4) as usize;

    let vx = emulator.variable_register[x_nibble];
    let vy = emulator.variable_register[y_nibble];

    let n_nibble = opcode & 0x000F;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;

    match opcode {
        0x0000 => (),
        0x00E0 => {
            emulator.display = vec![vec![0x00382b26; 64]; 32];
            // println!("Clear screen");
        }
        0x00EE => {
            let popped = emulator
                .stack
                .pop()
                .expect("Something went wrong unwrapping popped element from stack");

            emulator.program_counter = popped;
        }
        0x1000..=0x1FFF => {
            emulator.program_counter = nnn;
            emulator.increment_pc = false;
            // println!("{:04X}: Jump to address {:03X?}", opcode, nnn)
        }
        0x2000..=0x2FFF => {
            emulator.stack.push(emulator.program_counter);
            emulator.program_counter = nnn;
            emulator.increment_pc = false;
        }
        0x3000..=0x3FFF => {
            if vx == nn {
                emulator.program_counter += 2;
            }
        }
        0x4000..=0x4FFF => {
            if vx != nn {
                emulator.program_counter += 2;
            }
        }
        0x5000..=0x5FFF => {
            if vx == vy {
                emulator.program_counter += 2;
            }
        }
        0x6000..=0x6FFF => {
            emulator.variable_register[x_nibble] = nn;
            // println!("{:04X}: Set register V{:0X} to {:02X}", opcode, x_nibble, nn)
        }
        0x7000..=0x7FFF => {
            emulator.variable_register[x_nibble] = vx.wrapping_add(nn);
        }
        0x8000..=0x8FFF => match opcode & 0x000F {
            0x0000 => {
                emulator.variable_register[x_nibble] = vy;
            }
            0x0001 => {
                emulator.variable_register[x_nibble] = vx | vy;
            }
            0x0002 => {
                emulator.variable_register[x_nibble] = vx & vy;
            }
            0x0003 => {
                emulator.variable_register[x_nibble] = vx ^ vy;
            }
            0x0004 => {
                let addition = vx.overflowing_add(vy);
                emulator.variable_register[x_nibble] = addition.0;
                emulator.variable_register[15] = 1;
                if !addition.1 {
                    emulator.variable_register[15] = 0;
                }
            }
            0x0005 => {
                let subtraction = vx.overflowing_sub(vy);
                emulator.variable_register[x_nibble] = subtraction.0;
                emulator.variable_register[15] = 1;
                if subtraction.1 {
                    emulator.variable_register[15] = 0;
                }
            }
            0x0006 => {
                emulator.variable_register[x_nibble] = vx >> 1;
                emulator.variable_register[15] = vx & 1;
            }
            0x0007 => {
                let subtraction = vy.overflowing_sub(vx);
                emulator.variable_register[x_nibble] = subtraction.0;
                emulator.variable_register[15] = 1;
                if subtraction.1 {
                    emulator.variable_register[15] = 0;
                }
            }
            0x000E => {
                emulator.variable_register[x_nibble] = vx << 1;
                emulator.variable_register[15] = vx >> 7;
            }
            _ => println!("Unknown opcode: {:04X}", opcode),
        },
        0x9000..=0x9FFF => {
            if vx != vy {
                emulator.program_counter += 2;
            }
        }
        0xA000..=0xAFFF => {
            emulator.index_register = nnn as usize;
            // println!("{:04X}: Set index register to {:?}", opcode, nnn)
        }
        0xB000..=0xBFFF => {
            emulator.program_counter = nnn + emulator.variable_register[0] as u16;
        }
        0xC000..=0xCFFF => {
            let rand: u8 = rand::thread_rng().gen();
            emulator.variable_register[x_nibble] = rand & nn;
        }
        0xD000..=0xDFFF => {
            let x = (vx & 63) as usize;
            let y = (vy & 31) as usize;
            emulator.variable_register[15] = 0; // set VF to 0

            let inactive_color = 0x00382b26;
            let active_color = 0x00b8c2b9;

            let idx_reg = emulator.index_register;
            for bytes in 0..n_nibble {
                let sprite = emulator.memory[idx_reg + bytes as usize];
                for bit in 0..8 {
                    let x = (x + bit) % 64;
                    let y = (y + bytes as usize) % 32;
                    let color = (sprite >> (7 - bit)) & 1;
                    let current_color = emulator.display[y][x];
                    if color == 1 && current_color == active_color {
                        emulator.display[y][x] = inactive_color;
                        emulator.variable_register[15] = 1;
                    } else if color == 1 && current_color == inactive_color {
                        emulator.display[y][x] = active_color;
                    }
                }
            }
        }
        0xE09E..=0xEFFF => match opcode & 0x00FF {
            0x009E => {
                if emulator.key_pressed == vx {
                    emulator.program_counter += 2;
                }
            }
            0x00A1 => {
                if emulator.key_pressed != vx {
                    emulator.program_counter += 2;
                }
            }
            _ => println!("Unknown opcode: {:04X}", opcode),
        },
        0xF000..=0xFFFF => match opcode & 0x00FF {
            0x0007 => {
                emulator.variable_register[x_nibble] = emulator.delay_timer;
            }
            0x000A => {
                if emulator.key_pressed == 17 {
                    emulator.program_counter -= 2;
                } else {
                    emulator.variable_register[x_nibble] = emulator.key_pressed;
                }
            }
            0x0015 => {
                emulator.delay_timer = emulator.variable_register[x_nibble];
            }
            0x0018 => {
                emulator.sound_timer = emulator.variable_register[x_nibble];
            }
            0x001E => {
                let mut new_idx =
                    emulator.index_register + emulator.variable_register[x_nibble] as usize;
                if new_idx > 0x1000 {
                    emulator.variable_register[15] = 1;
                    new_idx = 0;
                }
                emulator.index_register = new_idx;
            }
            0x0029 => {
                emulator.index_register = emulator.variable_register[x_nibble] as usize * 5;
            }
            0x0033 => {
                let vx_string = vx.to_string();
                let mut vx_iter = vx_string.chars();
                for i in 0..3 {
                    let digit = vx_iter.next().unwrap_or('0');
                    emulator.memory[emulator.index_register + i] =
                        digit.to_digit(10).unwrap() as u8;
                }
            }
            0x0055 => {
                for i in 0..=x_nibble {
                    emulator.memory[emulator.index_register + i] = emulator.variable_register[i];
                }
            }
            0x0065 => {
                for i in 0..=x_nibble {
                    emulator.variable_register[i] = emulator.memory[emulator.index_register + i];
                }
            }
            _ => println!("Unknown opcode: {:04X}", opcode),
        },
        _ => println!("Unknown opcode: {:04X}", opcode),
    }
}

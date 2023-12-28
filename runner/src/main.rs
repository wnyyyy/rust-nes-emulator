use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;
use rand::Rng;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use emulator::cpu::CPU;
use emulator::memory::memory::Memory;
use emulator::common::logger::trace;

fn main() {
    let mut cpu = CPU::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Emulator", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();
    let mut screen_state = [0u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();
    let game = get_rom("../test roms/nestest.nes").expect("TODO: panic message");
    cpu.load(&game).expect("TODO: panic message");
    cpu.reset();
    cpu.program_counter = 0xc000;
    cpu.status.interrupt_disable = true;
    let log_lines = Rc::new(RefCell::new(Vec::new()));
    let log_lines_clone = log_lines.clone();
    let run = cpu.run(move |cpu| Ok({
        log_lines_clone.borrow_mut().push(trace(cpu)?);
        handle_user_input(cpu, &mut event_pump);
        cpu.write(0xfe, rng.gen_range(1..16)).expect("TODO: panic message");

        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 5_000));
    }));
    if let Err(e) = run {
        println!("\n\nError: {:?}", e);
    }
    write_log(log_lines.borrow().clone());
}

fn write_log(log_lines: Vec<String>) {
    let mut file = File::create("../log.txt").expect("TODO: panic message");
    for line in log_lines {
        file.write_all(line.as_bytes());
        file.write_all(b"\n");
    }
}

fn get_rom(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}


fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                cpu.write(0xff, 0x77).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cpu.write(0xff, 0x73).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.write(0xff, 0x61).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.write(0xff, 0x64).unwrap();
            }
            _ => {/* do nothing */}
        }
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.read(i as u16).unwrap();
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}
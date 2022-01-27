
use crate::decode;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Point;

use std::time::Duration;
use std::fs::File;
use std::io::Read;

pub fn render_BHI(fpath: String, window: &druid::WindowHandle)  {
    window.close();
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut file = match File::open(&fpath) {
        Err(err) => {
            println!("Error: BHI file at '{}' not found. {}", fpath, err);
            return;
        }
        Ok(decoded) => {
            println!("File successfully read.");
            decoded
        }
    };

    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data);


    let mut buf = Vec::new();
    let cpy: &[u8] = &Vec::from(data);

    snap::read::FrameDecoder::new(cpy).read_to_end(&mut buf).unwrap();

    println!("Read and de-compressed {} bytes of data.", &cpy.len());

    let decomp_buffer:String = buf.iter().map(move |b| *b as char).collect();
    
    println!("Encoded {} bytes of vectorized data.", &decomp_buffer.len());

    let dimensions:(u32, u32) = decode::get_dimensions(&decomp_buffer);
    let xmax: u32 = dimensions.0;
    let ymax: u32 = dimensions.1;

    let rgb: Vec<(u8, u8, u8)> = decode::pack_rgb(&decomp_buffer);







    let window = video_subsystem.window("garbage", xmax, ymax)
        .position_centered()
        .build().unwrap();

    let mut canvas:sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    canvas.clear();

    for i in 0..xmax {
        for j in 0..ymax {

            let index: u32;            
            if j == 0 {
                index = 0;
            } else {
                index = j*xmax - (xmax-i);
            }

            let cp = rgb[ index as usize ];

            canvas.set_draw_color(Color::RGB(cp.0, cp.1, cp.2));

            let p = Point::new(i as i32, j as i32);
            canvas.draw_point(p);
        }
    }

    
    println!("Finished rendering {}x{}", xmax, ymax);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}

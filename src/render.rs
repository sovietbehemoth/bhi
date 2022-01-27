
use crate::decode;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::time::Duration;
use std::fs::File;
use std::io::Read;



//The purpose of this file is to render BHI image files using low level graphics rendering.


/**
 * Top-level interfacing function for rendering BHI image files to the screen.
 * @fpath Path to BHI file.
 * @druid Context of main window to be closed.
 */
pub fn render_BHI(fpath: String, window: &druid::WindowHandle)  {

    //Close main window to open SDL2 window.
    window.close();
    
    //Initialize library and subsystem.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    //Open BHI.
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

    //Decode BHI format. Follows similar format to decode.rs


    let decomp_buffer:String = decode::to_raw_bhi(&mut file);
    
    println!("Encoded {} bytes of vectorized data.", &decomp_buffer.len());

    let dimensions:(u32, u32) = decode::get_dimensions(&decomp_buffer);
    let xmax: u32 = dimensions.0;
    let ymax: u32 = dimensions.1;

    let rgb: Vec<(u8, u8, u8)> = decode::pack_rgb(&decomp_buffer);






    //Open window.
    let window = video_subsystem.window(&fpath, xmax, ymax)
        .position_centered()
        .build().unwrap();


    //Canvas for writing.
    let mut canvas:sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    //Clear in preparation.
    canvas.clear();




    //Write pixels to the screen. Works in a similar manner to decode.rs.

    for i in 0..xmax {
        for j in 0..ymax {

            let index: u32;            
            if j == 0 {
                index = 0;
            } else {
                index = j*xmax - (xmax-i);
            }

            let cp = rgb[ index as usize ];

            //Set color.
            canvas.set_draw_color(Color::RGB(cp.0, cp.1, cp.2));

            //Draw point.
            let p = Point::new(i as i32, j as i32);
            canvas.draw_point(p);
        }
    }

    
    println!("Finished rendering {}x{}", xmax, ymax);
    canvas.present();





    //Event loop; essentially just waits.

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

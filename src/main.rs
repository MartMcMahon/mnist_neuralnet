#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 28;
const HEIGHT: u32 = 28;
const WINDOW_W: u32 = 320;
const WINDOW_H: u32 = 320;

use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    // read first image file
    let mut f = File::open("train-images-idx3-ubyte").unwrap();

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    assert_eq!(i32::from_be_bytes(buffer[0..=3].try_into().unwrap()), 2051);
    assert_eq!(i32::from_be_bytes(buffer[4..=7].try_into().unwrap()), 60000);

    for (i, x) in buffer.iter().enumerate() {
        if x.eq(&(255 as u8)) {
            println!("{}", i);
            break;
        }
    }
    let mut world = World {
        // image: buffer[16..800].try_into().unwrap(),
        images: vec![
            buffer[16..800].try_into().unwrap(),
            buffer[800..1584].try_into().unwrap(),
        ],
        image_index: 0,
    };

    // draw window
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WINDOW_W as f64, WINDOW_H as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            println!("index: {:#?}", world.image_index)
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // on click (0 == left mouse button)
            if input.mouse_released(0) {
                world.image_index += 1;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    });
}

struct World {
    images: Vec<Vec<u8>>,
    image_index: usize,
}
impl World {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let img = &self.images[self.image_index];
            let p = [img[i], img[i], img[i], 0xff];
            pixel.copy_from_slice(&p);
        }
    }
}

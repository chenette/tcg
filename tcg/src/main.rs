use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rusttype::{Font, Scale};
use image::{ImageBuffer, Rgba};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();
    // Create a window
    let window = WindowBuilder::new()
        .with_title("Counter App")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    // Create a pixel buffer
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut counter = 0;

    // Load a font
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
                        if input.state == winit::event::ElementState::Pressed {
                            counter += 1;
                        }
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();
                frame.fill(0); // Clear the frame

                // Draw the counter value as text
                let text = format!("Counter: {}", counter);
                draw_text(frame, &text, WIDTH, HEIGHT, &font);

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }

        window.request_redraw();
    });
}

fn draw_text(frame: &mut [u8], text: &str, width: u32, height: u32, font: &rusttype::Font) {
    let scale = Scale::uniform(24.0);
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(10.0, 10.0 + v_metrics.ascent);

    let mut image = ImageBuffer::from_fn(width, height, |_, _| Rgba([0, 0, 0, 0]));

    for glyph in font.layout(text, scale, offset) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x;
                let y = y as i32 + bounding_box.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    let alpha = (v * 255.0) as u8;
                    *pixel = Rgba([255, 255, 255, alpha]);
                }
            });
        }
    }

    for (i, pixel) in image.pixels().enumerate() {
        let rgba = pixel.0;
        let index = i * 4;
        frame[index] = rgba[0];
        frame[index + 1] = rgba[1];
        frame[index + 2] = rgba[2];
        frame[index + 3] = rgba[3];
    }
}
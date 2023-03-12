pub mod gman {
    extern crate sfml;

    use std::thread::JoinHandle;

    #[allow(unused_imports)]
    use sfml::{graphics::*, window::*, system::*};

    use crate::{mandelbrot::Mandelbrot, gui::Gui};

    pub struct Gm {
        pub window: RenderWindow,
        pub fps: f32,
    }

    impl Gm {
        pub fn new() -> Self {
            let mut window: RenderWindow = RenderWindow::new((1920, 1080), "window", Style::NONE, &Default::default());
            window.set_position((1000, 0).into());
            window.set_framerate_limit(60);
            window.set_vertical_sync_enabled(true);
            let gm: Gm = Gm {
                window,
                fps: 0.0,
            };
            return gm;
        }

        pub fn run(&mut self) {
            let mut mandelbrot: Mandelbrot = Mandelbrot::new(self.window.size().x as i32, self.window.size().y as i32);
            let mut gui: Gui = Gui::new("fonts/Roboto-Regular.ttf", 24);
            gui.add_slider(10.0, 40.0, 200.0, 0.0, 100.0);
            gui.add_slider(10.0, 70.0, 200.0, 0.0, 100.0);
            gui.add_slider(10.0, 100.0, 200.0, 0.0, 100.0);
            gui.add_label(10.0, 130.0, "press R to regenerate colors".to_string());
            let clock: sfml::SfBox<Clock> = Clock::start();
            let mut prev_time: Time = clock.elapsed_time();
            let mut current_time: Time;
            let mut thread: Option<JoinHandle<Vec<u8>>> = None;
            let mut is_thread_done: bool = true;
            let mut regen_colors: bool = false;

            while self.window.is_open() {
                while let Some(event) = self.window.poll_event() {
                    match event {
                        Event::Closed => self.window.close(),
                        Event::KeyPressed { code: Key::Escape, alt: false, ctrl: false, shift: false, system: false } => self.window.close(),
                        Event::KeyReleased { code, ..} => {
                            if code == Key::R {
                                regen_colors = true;
                            }
                        }
                        _ => {}
                    }
                }
            
                let mouse_pos: Vector2i = mouse::desktop_position();
                let mouse_pos: Vector2f = Vector2f::new((mouse_pos.x - self.window.position().x) as f32, (mouse_pos.y - self.window.position().y) as f32);
                gui.update(mouse_pos.x as i32, mouse_pos.y as i32, (mouse::Button::Left.is_pressed(), mouse::Button::Middle.is_pressed(), mouse::Button::Right.is_pressed()));

                if gui.slider_components[0].get_value_changed() || gui.slider_components[1].get_value_changed() || gui.slider_components[2].get_value_changed() || regen_colors {
                    thread = Some(mandelbrot.set_color(gui.slider_components[0].value, gui.slider_components[1].value, gui.slider_components[2].value.into()));
                    is_thread_done = false;
                    regen_colors = false;
                }

                if is_thread_done == false {
                    let join_handle = thread.take().expect("must exist at this point (why doesnt it)");
                    if join_handle.is_finished() {
                        is_thread_done = true;
                        let pixels = join_handle.join().unwrap();
                        mandelbrot.set_pixels(pixels);
                    } else {
                        thread = Some(join_handle);
                    }
                }

                mandelbrot.prepare_for_render();
                self.window.clear(Color::WHITE);
                self.window.draw(&mandelbrot);
                self.window.draw(&gui);
                self.window.display();

                current_time = clock.elapsed_time();
                self.fps = 1.0 / (current_time.as_seconds() - prev_time.as_seconds());
                let title: &str = &("mandelbrot    fps: ".to_owned() + &self.fps.to_string());
                self.window.set_title(title);
                prev_time = current_time;
            }
        }
    }
}

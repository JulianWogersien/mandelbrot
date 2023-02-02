pub mod gman {
    extern crate sfml;

    #[allow(unused_imports)]
    use sfml::{graphics::*, window::*, system::*};

    use crate::mandelbrot::Mandelbrot;

    pub struct Gm {
        pub window: RenderWindow,
        pub fps: f32,
    }

    impl Gm {
        pub fn new() -> Self {
            let mut window: RenderWindow = RenderWindow::new((1920, 1080), "window", Style::FULLSCREEN, &Default::default());
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
            
            let clock: sfml::SfBox<Clock> = Clock::start();
            let mut prev_time: Time = clock.elapsed_time();
            let mut current_time: Time;

            while self.window.is_open() {
                while let Some(event) = self.window.poll_event() {
                    match event {
                        Event::Closed => self.window.close(),
                        Event::KeyPressed { code: Key::Escape, alt: false, ctrl: false, shift: false, system: false } => self.window.close(),
                        Event::KeyReleased { code, ..} => {
                            if code == Key::R {
                                
                            }
                        }
                        _ => {}
                    }
                }
            

                mandelbrot.prepare_for_render();
                self.window.clear(Color::WHITE);
                self.window.draw(&mandelbrot);
                self.window.display();

                current_time = clock.elapsed_time();
                self.fps = 1.0 / (current_time.as_seconds() - prev_time.as_seconds());
                let title: &str = &("g    fps: ".to_owned() + &self.fps.to_string());
                self.window.set_title(title);
                prev_time = current_time;
            }

            // terminate
        }
    }
}

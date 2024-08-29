use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::loader::{self, load};
use crate::renderer::Screen;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 10.;

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes
        let frame_duration = Duration::from_millis(10);
        let mut start = Instant::now();

        let mesh = load(loader::MAP);

        loop {
            // get list off all vertices

            if Instant::now() - start >= frame_duration {
                self.renderer.render(&self.camera, &mesh);
                start = Instant::now();
            }
            // render vertices
            // handle input

            if poll(Duration::from_millis(5)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => match event {
                        KeyEvent {
                            code,
                            modifiers: _,
                            kind: _,
                            state: _,
                        } => match code {
                            KeyCode::Char('e') => {
                                let _ = crossterm::terminal::disable_raw_mode().unwrap();
                                panic!("Exited app")
                            }
                            KeyCode::Right => {
                                self.camera.pos = self.camera.pos
                                    + Vec3 {
                                        x: SPEED,
                                        y: 0.,
                                        z: 0.,
                                    }
                                    .rotate_y(self.camera.rotation.x);
                            }
                            KeyCode::Left => {
                                self.camera.pos = self.camera.pos
                                    - Vec3 {
                                        x: SPEED,
                                        y: 0.,
                                        z: 0.,
                                    }
                                    .rotate_y(self.camera.rotation.x);
                            }
                            KeyCode::Up => {
                                self.camera.pos = self.camera.pos
                                    + Vec3 {
                                        x: 0.,
                                        y: 0.,
                                        z: SPEED,
                                    }
                                    .rotate_y(self.camera.rotation.x);
                            }
                            KeyCode::Down => {
                                self.camera.pos = self.camera.pos
                                    - Vec3 {
                                        x: 0.,
                                        y: 0.,
                                        z: SPEED,
                                    }
                                    .rotate_y(self.camera.rotation.x);
                            }
                            KeyCode::Char('w') => {
                                self.camera.rotation.y += 0.05;
                            }
                            KeyCode::Char('a') => {
                                self.camera.rotation.x -= 0.05;
                            }
                            KeyCode::Char('s') => {
                                self.camera.rotation.y -= 0.05;
                            }
                            KeyCode::Char('d') => {
                                self.camera.rotation.x += 0.05;
                            }
                            _ => (),
                        },
                    },
                    _ => (),
                }
            }

            // perform game logic
        }
    }
}

use std::sync::mpsc;
use glutin;
#[cfg(feature = "glfw")]
use glfw;
use sys;

pub type ReceiverHub = (
    mpsc::Receiver<sys::control::Event>,
    mpsc::Receiver<sys::bullet::Event>
);

pub struct SenderHub {
    control: mpsc::Sender<sys::control::Event>,
    bullet: mpsc::Sender<sys::bullet::Event>,
}

impl SenderHub {
    pub fn new() -> (SenderHub, ReceiverHub) {
        let (sc, rc) = mpsc::channel();
        let (sb, rb) = mpsc::channel();
        (SenderHub {
            control: sc,
            bullet: sb,
        }, (rc, rb))
    }

    pub fn process_glutin(&self, event: glutin::Event) {
        use sys::control::Event::*;
        use sys::bullet::Event::*;
        use glutin::Event::KeyboardInput;
        use glutin::{ElementState, VirtualKeyCode};
        match event {
            KeyboardInput(state, _, Some(VirtualKeyCode::A)) =>
                self.control.send(EvThrust(match state {
                    ElementState::Pressed => 1.0,
                    ElementState::Released => 0.0,
                })).unwrap(),
            KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) =>
                self.control.send(EvTurn(-1.0)).unwrap(),
            KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) =>
                self.control.send(EvTurn(1.0)).unwrap(),
            KeyboardInput(ElementState::Released, _, Some(k))
                if k == VirtualKeyCode::Left || k == VirtualKeyCode::Right =>
                self.control.send(EvTurn(0.0)).unwrap(),
            KeyboardInput(state, _, Some(VirtualKeyCode::S)) =>
                self.bullet.send(EvShoot(match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                })).unwrap(),
            _ => (),
        }
    }

    #[cfg(feature = "glfw")]
    pub fn process_glfw(&self, event: glfw::WindowEvent) {
        use sys::control::Event::*;
        use sys::bullet::Event::*;
        use glfw::{Action, Key, WindowEvent};
        match event {
            WindowEvent::Key(Key::A, _, state, _) =>
                self.control.send(EvThrust(match state {
                    Action::Press | Action::Repeat => 1.0,
                    Action::Release => 0.0,
                })).unwrap(),
            WindowEvent::Key(Key::Left, _, Action::Press, _) =>
                self.control.send(EvTurn(-1.0)).unwrap(),
            WindowEvent::Key(Key::Right, _, Action::Press, _) =>
                self.control.send(EvTurn(1.0)).unwrap(),
            WindowEvent::Key(k, _, Action::Release, _)
                if k == Key::Left || k == Key::Right =>
                self.control.send(EvTurn(0.0)).unwrap(),
            WindowEvent::Key(Key::S, _, state, _) =>
                self.bullet.send(EvShoot(match state {
                    Action::Press | Action::Repeat => true,
                    Action::Release => false,
                })).unwrap(),
            _ => (),
        }
    }
}

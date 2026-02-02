mod screen;
mod os;
mod keys;

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use tao::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new();
    let mut hot_key_manager = match GlobalHotKeyManager::new(){
        Ok(h) => h,
        _ => {
            panic!("Failed to initialize global hot key manager");
        }
    };

    let (on_id, off_id) = keys::register_keycodes(&mut hot_key_manager);
    screen::show_text();

    {
        std::thread::spawn(move || {
            let reciever = GlobalHotKeyEvent::receiver();

            while let Ok(event) = reciever.recv() {
                if event.state != HotKeyState::Pressed {
                    continue;
                }

                let res = if event.id == off_id {
                    screen::disable_screen()
                } else if event.id == on_id {
                    screen::active_screen()
                } else {
                    continue;
                };

                if let Err(e) = res {
                    eprintln!("[screenoff] {e}");
                }
            }
        });


        event_loop.run(move |_event, _target, control_flow| {
            *control_flow = ControlFlow::Wait;
        });
    }
}

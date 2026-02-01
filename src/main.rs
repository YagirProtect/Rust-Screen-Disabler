mod screen;
mod os;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use global_hotkey::hotkey::{Code, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::HotKey;
use tao::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new();
    let hot_key_manager = match GlobalHotKeyManager::new(){
        Ok(h) => h,
        _ => {
            panic!("Failed to initialize global hot key manager");
        }
    };

    match hot_key_manager.register(HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::F11)){
        Ok(_) => {},
        Err(_) => {
            panic!("Failed to register hot key manager");
        }
    }

    screen::show_text();


    let screen_active = Arc::new(AtomicBool::new(false));


    {
        let screen_active = Arc::clone(&screen_active);
        std::thread::spawn(move || {
            let reciever = GlobalHotKeyEvent::receiver();

            while let Ok(event) = reciever.recv() {
                if event.state != HotKeyState::Pressed {
                    continue;
                }



                let target_active = !screen_active.load(Ordering::Relaxed);
                let res = if target_active { screen::disable_screen() } else { screen::active_screen() };

                match res {
                    Ok(()) => {
                        screen_active.store(target_active, Ordering::Relaxed)
                    }
                    Err(e) => {
                        eprintln!("[screenoff] {e}")
                    }
                }
            }
        });


        event_loop.run(move |_event, _target, control_flow| {
            *control_flow = ControlFlow::Wait;
        });
    }
}

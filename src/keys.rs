use global_hotkey::GlobalHotKeyManager;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

pub fn register_keycodes(hotkey_manager: &mut GlobalHotKeyManager) -> (u32, u32){

    let on_key = get_on_key();
    let off_key = get_off_key();
    
    match hotkey_manager.register(on_key) {
        Ok(_) => {},
        Err(_) => {
            panic!("Failed to register hot key manager");
        }
    }

    match hotkey_manager.register(off_key) {
        Ok(_) => {},
        Err(_) => {
            panic!("Failed to register hot key manager");
        }
    }


    (on_key.id, off_key.id)
}


fn get_on_key() -> HotKey{
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    return HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyL);

    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::F12)
}

fn get_off_key() -> HotKey{
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    return HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyK);

    HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::F11)
}
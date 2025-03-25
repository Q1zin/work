use std::process::Command;
use std::thread;
use std::time::Duration;

// API for get keyboard layout
fn get_keyboard_layout() -> String {
    #[cfg(target_os = "windows")] {
        "TODO".to_string()
    }

    #[cfg(target_os = "linux")] {
        "TODO".to_string()
    }

    #[cfg(target_os = "macos")] {
        get_keyboard_layout_mac()
    }
}

// Event for change layout
fn spaw_lang(new_layout: &str) {
    println!("Изменение раскладки: {}", new_layout);
}

fn get_keyboard_layout_mac() -> String {
    let output = Command::new("defaults")
        .args(&["read", 
                "com.apple.HIToolbox", 
                "AppleCurrentKeyboardLayoutInputSourceID"])
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = result.trim().split('.').collect();

        parts
            .last()
            .map(|&layout| map_layout_to_lang(layout).to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn map_layout_to_lang(layout: &str) -> &str {
    match layout {
        "Russian" | "RussianWin" | "Russian-Phonetic" => "RU",
        "US" | "Australian" | "British" | "British-PC" | "USInternational-PC" | "ABC" => "EN",
        _ => layout,
    }
}

// Пока не нашёл нормального способа получить событие изменения раскладки клавиатуры
// Поэтому пока что буду опрашивать каждые 250 мс
fn track_keyboard_layout_changes() {
    let mut last_layout = get_keyboard_layout();
    loop {
        let current_layout = get_keyboard_layout();
        thread::sleep(Duration::from_millis(250));
        if current_layout != last_layout {
            last_layout = current_layout;
            spaw_lang(&last_layout)
        }
    }
}

fn main() {
    let handle = thread::spawn(|| {
        track_keyboard_layout_changes();
    });

    handle.join().unwrap();
}
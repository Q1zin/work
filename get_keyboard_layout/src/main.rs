use std::process::Command;

fn get_keyboard_layout() -> String {
    #[cfg(target_os = "windows")] {
        "Windows layout".to_string()
    }

    #[cfg(target_os = "linux")] {
        "Linux layout".to_string()
    }

    #[cfg(target_os = "macos")] {
        get_keyboard_layout_mac()
    }
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
        let collection: Vec<&str> = result.trim().split('.').collect();

        match collection.last().copied() {
            Some(layout) => String::from(map_layout_to_lang(layout)),
            None => String::from("Unknown")
        }
    } else {
        String::from("Unknown")
    }
}

fn map_layout_to_lang(layout: &str) -> &str {
    match layout {
        "Russian" | "RussianWin" | "Russian-Phonetic" => "RU",
        "US" | "Australian" | "British" | "British-PC" | "USInternational-PC" | "ABC" => "EN",
        _ => layout,
    }
}

fn main() {
    println!("{}", get_keyboard_layout());    
}

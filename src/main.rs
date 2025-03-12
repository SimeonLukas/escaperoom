use directories::UserDirs;
use std::{thread, time};


fn main() {
    let bytes = include_bytes!("ende.mp4");
    let mut line = String::new();
    println!("Ich bin Groot!");
    println!("Bitte Passwort eingeben:");
    std::io::stdin().read_line(&mut line).unwrap();
    let b1 = line.trim();
    if b1 == "713" {
        println!("Passwort {} ist korrekt!", b1);
        // write file to Desktop
        println!("Datei wird extrahiert...");
        let user_dirs = UserDirs::new().unwrap();
        let desktop_dir = user_dirs.desktop_dir().unwrap();
        let file_path = desktop_dir.join("finale.mp4");
        std::fs::write(file_path, bytes).unwrap();
        println!("Video auf Desktop gespeichert!");
        println!("Taste zum Beenden drücken...");
        std::io::stdin().read_line(&mut line).unwrap();
    } else {
        println!("Passwort falsch!");
        println!("Programm wird in 5 Sekunden neu gestartet...");
        thread::sleep(time::Duration::from_secs(1));
        println!("5");
        thread::sleep(time::Duration::from_secs(1));
        println!("\r4");
        thread::sleep(time::Duration::from_secs(1));
        println!("3");
        thread::sleep(time::Duration::from_secs(1));
        println!("2");
        thread::sleep(time::Duration::from_secs(1));
        println!("1");
        thread::sleep(time::Duration::from_secs(1));
        println!("Programm wird neu gestartet...");
        main()
    }
}

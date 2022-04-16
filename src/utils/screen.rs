// Clears the screen. To be used at the start of the program.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// Prints a banner for the program.
pub fn print_banner() {
    let banner = r#"    
        _       __     ____  _
       / \   ___| |__ |  _ \| |
      / _ \ / __| '_ \| | | | |
     / ___ \\__ \ | | | |_| | |___
    /_/   \_\___/_| |_|____/|_____|

    > Download spotify playlists."#;
    println!("{}", banner);
}

use anyhow::Result;
use colored::*;

pub fn run() -> Result<()> {
    println!("|030| {}    |040| {}", "Black".black(), "Black".on_black());
    println!("|031| {}      |041| {}", "Red".red(), "Red".on_red());
    println!("|032| {}    |042| {}", "Green".green(), "Green".on_green());
    println!("|033| {}   |043| {}", "Yellow".yellow(), "Yellow".on_yellow());
    println!("|034| {}     |044| {}", "Blue".blue(), "Blue".on_blue());
    println!("|035| {}  |045| {}", "Magenta".magenta(), "Magenta".on_magenta());
    println!("|036| {}     |046| {}", "Cyan".cyan(), "Cyan".on_cyan());
    println!("|037| {}    |047| {}", "White".white(), "White".on_white());

    Ok(())
}

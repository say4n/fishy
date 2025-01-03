use std::{collections::HashMap, io::Read, ops::Div};

use crossterm::style::Stylize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hist_file_path = shellexpand::tilde("~/.local/share/fish/fish_history").to_string();
    let mut file = std::fs::File::open(hist_file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let entries: Vec<Option<&str>> = contents
        .split("- cmd: ")
        .map(|e| e.split("\n").next())
        .collect();

    let mut command_statistics = HashMap::<String, u32>::new();

    for entry in entries.to_owned() {
        if entry.is_some() {
            let key = entry.unwrap().split(" ").next().unwrap();
            let count = command_statistics.get(key).unwrap_or(&0);
            command_statistics.insert(key.to_string(), count + 1);
        }
    }

    let mut sorted_command_statistics: Vec<(&String, &u32)> = command_statistics.iter().collect();
    sorted_command_statistics.sort_by(|a, b| b.1.cmp(a.1));

    let top_k = 10;

    let heading = format!("Top {} commands", top_k);
    println!("{}\n", heading.white().on_black().bold());

    for i in 0..top_k {
        let (command, count) = sorted_command_statistics[i];
        let max_count = sorted_command_statistics[0].1;
        let bars_count = ((10 * count).div(max_count)).try_into().unwrap();

        println!(
            "│{}│ {:>5} {}",
            ["█".repeat(bars_count), " ".repeat(10 - bars_count)].join(""),
            count,
            command.to_owned().bold()
        );
    }

    println!();
    println!("Total commands: {}", entries.to_owned().len());
    println!("Unique commands: {}", command_statistics.len());

    Ok(())
}

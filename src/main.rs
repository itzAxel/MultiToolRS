use colored::Colorize;
use serde_json;
use serde_json::Value;
use std::fs::{read_to_string, write, File};
use std::io::{stdin, BufReader};
use std::path::Path;
use subprocess::Exec;
use winver::WindowsVersion;

fn print_help() {
    let help = {
        "\x1b[32mUsage:
      \x1b[32m-dism:\x1b[33m Recovery with DISM
      \x1b[32m-sfc:\x1b[33m Recovery with SFC
      \x1b[32m-activate:\x1b[33m Activate your OS with KMS server
      \x1b[32m-help:\x1b[33m Print this menu
      \x1b[32m-clear:\x1b[33m Clear %TEMP% Folder
      \x1b[32m-about:\x1b[33m About this program
    "
    };
    println!("{}", help)
}

fn print_about() {
    let help = {
        "\x1b[32mAbout:
    \x1b[32mMultiTool, version: 0.0.1 \x1b[33mTest
    \x1b[32mAuthor, main creator: ItzAxel, Special Thanks for: NikSne
    \x1b[33mProject on GitHub:\x1b[31m //No Project on GitHub currently
    "
    };
    println!("{}", help)
}

fn main() {
    let mut windows_version: &str = "";
    let stdm = stdin();
    let input = &mut String::new();
    let reader: BufReader<File>;
    let mut data: Value = Default::default();
    let mut status: bool = false;
    let mut status_txt: bool = true;
    let input_1 = &mut String::new();

    println!("{}", "Starting MultiTool RS...".yellow());

    println!("{}", "Checking files...".yellow());
    if !Path::new("keys.json").is_file() {
        println!("{}{}", "[!]ERROR:".red(), " keys.json not found.".yellow());
        status = false
    } else {
        println!("{}", "File 'keys.json' is OK".green());
        reader = BufReader::new(File::open("keys.json").unwrap());
        data = serde_json::from_reader(reader).expect("[!]ERROR: Bad Json");
        status = true;
    }

    if !Path::new("server.txt").is_file() {
        println!(
            "{}{}",
            "[!]WARNING:".red(),
            " server.txt not found.".yellow()
        );
        status_txt = false
    } else {
        println!("{}", "File 'server.txt' is OK".green())
    }

    println!("{}", "Checking Windows version...".yellow());
    let version = WindowsVersion::detect().unwrap();
    if version >= WindowsVersion::new(10, 0, 0000) && version < WindowsVersion::new(10, 0, 22000) {
        println!("{}", "Windows version: 10".blue());
        windows_version = "10"
    }
    if version >= WindowsVersion::new(10, 0, 22000) {
        println!("{}", "Windows version: 11".blue());
        windows_version = "11"
    }

    println!("{}", "MultiTool RS Successfully Started".green());
    print_help();
    loop {
        input.clear();
        println!("\x1b[32mEnter Command:");
        let _ = stdm.read_line(input);
        match input.trim() {
            "-activate" => {
                // Желательно вынести в другую функцию
                if !status {
                    continue;
                }
                let mut edition_input = String::new();
                println!("Select Redaction (home, home_sl, pro, corp, enterprise, core):");
                stdin().read_line(&mut edition_input).expect("Input Error");
                let edition_input = edition_input.trim();

                if let Value::Object(ref map) = data {
                    if let Some(version_data) = map.get(windows_version) {
                        if let Value::Object(edition_data) = version_data {
                            if let Some(value) = edition_data.get(edition_input) {
                                println!(
                                    "Key for Windows {} Redaction {}: {}",
                                    windows_version, edition_input, value
                                );
                                println!("{}", "Starting activation Procedure...".green());
                                let args: &str = &("slmgr /ipk ".to_owned() + &value.to_string());
                                println!("{}", "Setting key...".yellow());
                                shell(args);
                                println!("{}", "Setting KMS server...".yellow());
                                if status_txt {
                                    let txt =
                                        read_to_string("server.txt").expect("Unable to read file");
                                    let args = &("slmgr /skms ".to_owned() + &txt.to_string());
                                    shell(args)
                                } else {
                                    println!(
                                        "{}{}",
                                        "[!]ERROR:".red(),
                                        " server.txt not found.".yellow()
                                    );
                                    println!("{}", "Enter KMS server address:".green());
                                    let stdi = stdin();
                                    let _ = stdi.read_line(input_1);
                                    println!("{}", "Setting and saving KMS server...".yellow());
                                    let args = &("slmgr /skms ".to_owned() + &input_1.to_string());
                                    write("server.txt", input_1.clone())
                                        .expect("Unable to write file");
                                    shell(args)
                                }
                                println!("{}", "Trying to activate...".yellow());
                                shell("slmgr /ato");
                                println!("{}", "Procedure completed".green());
                            } else {
                                println!(
                                    "\x1b[33mRedaction '{}' not found for Windows '{}'.\x1b[32m",
                                    edition_input, windows_version
                                );
                            }
                        }
                    }
                } else {
                    println!("{}{}", "[!]ERROR:".red(), " Json read failed.".yellow())
                }
            }
            "-about" => print_about(),
            "-help" => print_help(),

            "-sfc" => {
                println!(
                    "{}",
                    "[!] WARNING: DO NOT OFF PC OR CLOSE THIS WINDOW!".red()
                );
                shell("sfc /scannow");
                println!("{}", "Procedure completed".green());
            }

            "-dism" => {
                println!(
                    "{}",
                    "[!] WARNING: DO NOT OFF PC OR CLOSE THIS WINDOW!".red()
                );
                shell("DISM /Online /Cleanup-Image /RestoreHealth");
                println!("{}", "Procedure completed".green());
            }
            _ => {
                println!("{}", "[!]ERROR: Unknown command".red())
            }
        } // Match
    } // Loop
} // fn main

fn shell(command: &str) {
    let output = Exec::shell(command).capture();
    let _ = match output {
        Ok(out) => println!("\x1b[35m{}", out.stdout_str()),
        Err(error) => println!("\x1b[31m[!]ERROR: {}", error),
    };
}

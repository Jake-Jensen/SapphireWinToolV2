#![allow(unused_parens)]
#![allow(non_snake_case)]

use std::{process::exit, fmt::Write};

const IsDebug: bool = true;

fn ClearTerminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn WriteMenu_Main() {
    println!("---------- Main Menu ----------");
    println!("1. Repair tools");
    println!("2. Gaming related tweaks");
    println!("3. System related tweaks");
    println!("0. Exit");

    let mut Choice = String::new();
    let UserInput = std::io::stdin().read_line(&mut Choice).unwrap();
    if (IsDebug) {
        dbg!(Choice.clone(), UserInput);
    }
    Choice = Choice.trim().to_string();
    dbg!(Choice.clone());

    // No real reason for this, just felt like being pedantic.
    let ChoiceChar = Choice.chars().nth(0).unwrap();
    dbg!(ChoiceChar.clone());

    // Sanitization and error checking
    if (ChoiceChar.len_utf16() == 0) {
        let _ = msgbox::create("Warning", "You have to enter something. Try again.", msgbox::IconType::Info);
        ClearTerminal();
        WriteMenu_Main();
    }
    if (ChoiceChar.is_alphabetic()) {
        let _ = msgbox::create("Warning", "Numbers only. Try again.", msgbox::IconType::Info);
        ClearTerminal();
        WriteMenu_Main();
    }
    if (ChoiceChar.is_control() || !ChoiceChar.is_ascii_digit()) {
        let _ = msgbox::create("Warning", "Something other than a regular ASCII digit was entered. Numbers only. Try again.", msgbox::IconType::Info);
        ClearTerminal();
        WriteMenu_Main();
    }

    let FirstCharacter = Choice.chars().nth(0).unwrap();
    match FirstCharacter {
        '0' => {exit(0);}
        '1' => {WriteMenu_Repair();}
        '2' => {WriteMenu_Gaming();}
        '3' => {WriteMenu_System();}
        _=> {
            let _ = msgbox::create("Warning", "Invalid choice made. Try again.", msgbox::IconType::Info);
            ClearTerminal();
            WriteMenu_Main();
        }
    }


}

fn WriteMenu_Repair() {
    println!("Repair menu");
    exit(0);
}

fn WriteMenu_Gaming() {
    println!("Gaming menu");
    exit(0);
}

fn WriteMenu_System() {
    println!("System menu");
    exit(0);
}

fn main() {
    ClearTerminal();

    if (!IsDebug) {
        if !is_elevated::is_elevated() {
            let _ = msgbox::create("Warning", 
                "This program requires administrative rights to perform most actions. I will automatically close after this box is closed.", 
                msgbox::IconType::Info);
                exit(1);
        } else {
            println!("------ Sapphire Win Tool V2 - Rusty ------")
        }
    }

    // Write out the menu
    WriteMenu_Main();
}
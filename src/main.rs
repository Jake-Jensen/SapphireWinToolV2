#![allow(unused_parens)]
#![allow(non_snake_case)]

fn main() {
    if !is_elevated::is_elevated() {
        let _ = msgbox::create("Warning", 
            "This program requires administrative rights to perform most actions. I will automatically close after this box is closed.", 
            msgbox::IconType::Info);
    } else {
        println!("------ Sapphire Win Tool V2 - Rusty ------")
    }
    // Continue exection.

    
}
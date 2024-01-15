use std::env;

pub const PREFIX: &'static str = "-";
pub const TEST_PREFIX: &'static str = "=";
pub const EMBED_COLOUR: u32 = 0xff66ab;
pub const ADMINS: (&str, &str, &str) = ("132638288770105344", "182383793716461570", "97132796560031744"); // this was an array and not a tuple. i'd have known that if i read the docs Lol

fn is_test() -> bool {
    let test_flag = std::env::args().nth(1);
    if let Some(value) = test_flag {
        if value == "-t" {
            return true
        } else {
            println!("test flag is '-t'. defaulting to normal account.");
            return false
        }
    } else {
        return false // default to normal account
    }
}

pub fn prefix() -> &'static str {
    if !is_test() {
        return PREFIX
    } else {
        return TEST_PREFIX
    }
}

pub fn token() -> String {
    if !is_test() {
        return env::var("DISCORD_TOKEN").unwrap(); // change to FRAGGER_MAIN_TOKEN when done
    } else {
        return env::var("FRAGGER_TEST_TOKEN").unwrap();
    }
}

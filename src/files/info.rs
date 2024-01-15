use std::env;

pub const PREFIX: &'static str = "-";
pub const TEST_PREFIX: &'static str = "=";
pub const EMBED_COLOUR: u32 = 0xff66ab;
pub const ADMINS: &'static [&str; 3] = &["132638288770105344", "182383793716461570", "97132796560031744"]; // todo: figure out how to integrate this into framework.owners()
// addendum: framework.owners relies on discord's development teams feature.
// god knows if i'll get around to using that but line 6 can be removed if i do.

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
    if is_test() {
        return TEST_PREFIX
    } else {
        return PREFIX
    }
}

pub fn token() -> String {
    if is_test() {
        return env::var("DISCORD_TEST_TOKEN").unwrap();
    } else {
        return env::var("DISCORD_TOKEN").unwrap();
    }
}

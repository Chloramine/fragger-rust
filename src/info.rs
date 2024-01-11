pub const TOKEN: &'static str = "";
pub const TEST_TOKEN: &'static str = "";
pub const PREFIX: &'static str = "-";
pub const TEST_PREFIX: &'static str = "=";
pub const EMBED_COLOUR: u32 = 0xff66ab;
pub const ADMINS: &'static [&str; 3] = &["132638288770105344", "182383793716461570", "97132796560031744"];

pub fn prefix() -> &'static str {
    if cfg!(test) {
        return TEST_PREFIX
    } else {
        return PREFIX
    }
}

pub fn token() -> &'static str {
    if cfg!(test) {
        return TEST_TOKEN
    } else {
        return TOKEN
    }
}

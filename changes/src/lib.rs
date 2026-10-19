#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
	    return Light {
	        alias: alias.to_string(),
	        brightness: 0 as u8
	    }
	}
}

pub fn change_brightness(lights: &mut [Light], name: &str, brightness: u8) {
    for light in lights.iter_mut() {
        if light.alias == name {
            light.brightness = brightness;
            break;
        }
    }
}
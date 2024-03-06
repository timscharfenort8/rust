use crate::spec::{base, Target};

pub fn target() -> Target {
    base::avr_gnu::target::<"-mmcu=atmega328">("atmega328")
}

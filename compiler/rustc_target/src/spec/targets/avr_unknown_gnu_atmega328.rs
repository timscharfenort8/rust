use crate::spec::{base, Target};

pub static TARGET: Target = base::avr_gnu::target::<"-mmcu=atmega328">("atmega328");

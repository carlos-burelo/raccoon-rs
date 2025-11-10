use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_time_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "now",
        Some("time"),
        |_args| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            (now.as_millis() as i64).to_raccoon()
        },
        0,
        Some(0),
    );

    registrar.register_fn(
        "now_secs",
        Some("time"),
        |_args| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            (now.as_secs() as i64).to_raccoon()
        },
        0,
        Some(0),
    );

    registrar.register_fn(
        "sleep",
        Some("time"),
        |args| {
            let ms = i64::from_raccoon(&args[0]).unwrap_or(0) as u64;
            thread::sleep(Duration::from_millis(ms));
            RuntimeValue::Null(crate::runtime::NullValue::new())
        },
        1,
        Some(1),
    );
}

use crate::runtime::{FromRaccoon, Registrar, ToRaccoon};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_random_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "random",
        Some("random"),
        |_args| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let nanos = now.as_nanos() as u128;

            let val = ((nanos % 1000000) as f64) / 1000000.0;
            val.to_raccoon()
        },
        0,
        Some(0),
    );

    registrar.register_fn(
        "rand_int",
        Some("random"),
        |args| {
            let min = i64::from_raccoon(&args[0]).unwrap_or(0);
            let max = i64::from_raccoon(&args[1]).unwrap_or(100);

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let nanos = now.as_nanos() as u128;
            let range = max - min;
            if range <= 0 {
                min.to_raccoon()
            } else {
                let val = min + ((nanos % range as u128) as i64);
                val.to_raccoon()
            }
        },
        2,
        Some(2),
    );

    registrar.register_fn(
        "rand_float",
        Some("random"),
        |args| {
            let min = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let max = f64::from_raccoon(&args[1]).unwrap_or(1.0);

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            let nanos = now.as_nanos() as u128;
            let frac = ((nanos % 1000000) as f64) / 1000000.0;
            (min + frac * (max - min)).to_raccoon()
        },
        2,
        Some(2),
    );
}

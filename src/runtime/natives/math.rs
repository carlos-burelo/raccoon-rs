use crate::runtime::{FromRaccoon, Registrar, ToRaccoon};

pub fn register_math_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "sqrt",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sqrt().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "pow",
        Some("math"),
        |args| {
            let base = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let exp = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            base.powf(exp).to_raccoon()
        },
        2,
        Some(2),
    );

    registrar.register_fn(
        "sin",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sin().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "cos",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.cos().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "tan",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.tan().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "log",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(1.0);
            let base = if args.len() > 1 {
                f64::from_raccoon(&args[1]).unwrap_or(std::f64::consts::E)
            } else {
                std::f64::consts::E
            };
            x.log(base).to_raccoon()
        },
        1,
        Some(2),
    );

    registrar.register_fn(
        "min",
        Some("math"),
        |args| {
            let a = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let b = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            a.min(b).to_raccoon()
        },
        2,
        Some(2),
    );

    registrar.register_fn(
        "max",
        Some("math"),
        |args| {
            let a = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let b = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            a.max(b).to_raccoon()
        },
        2,
        Some(2),
    );

    registrar.register_fn(
        "abs",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.abs().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "floor",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.floor().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "ceil",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.ceil().to_raccoon()
        },
        1,
        Some(1),
    );

    registrar.register_fn(
        "round",
        Some("math"),
        |args| {
            let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.round().to_raccoon()
        },
        1,
        Some(1),
    );
}

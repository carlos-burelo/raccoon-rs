use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

primitive! {
    math::core_sqrt(x: f64) -> f64 {
        x.sqrt()
    }
}

primitive! {
    math::core_cbrt(x: f64) -> f64 {
        x.cbrt()
    }
}

primitive! {
    math::core_sin(x: f64) -> f64 {
        x.sin()
    }
}

primitive! {
    math::core_cos(x: f64) -> f64 {
        x.cos()
    }
}

primitive! {
    math::core_tan(x: f64) -> f64 {
        x.tan()
    }
}

primitive! {
    math::core_asin(x: f64) -> f64 {
        x.asin()
    }
}

primitive! {
    math::core_acos(x: f64) -> f64 {
        x.acos()
    }
}

primitive! {
    math::core_atan(x: f64) -> f64 {
        x.atan()
    }
}

primitive! {
    math::core_atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }
}

primitive! {
    math::core_sinh(x: f64) -> f64 {
        x.sinh()
    }
}

primitive! {
    math::core_cosh(x: f64) -> f64 {
        x.cosh()
    }
}

primitive! {
    math::core_tanh(x: f64) -> f64 {
        x.tanh()
    }
}

primitive! {
    math::core_exp(x: f64) -> f64 {
        x.exp()
    }
}

primitive! {
    math::core_ln(x: f64) -> f64 {
        x.ln()
    }
}

primitive! {
    math::core_log10(x: f64) -> f64 {
        x.log10()
    }
}

primitive! {
    math::core_log(x: f64, base: f64) -> f64 {
        x.log(base)
    }
}

primitive! {
    math::core_floor(x: f64) -> f64 {
        x.floor()
    }
}

primitive! {
    math::core_ceil(x: f64) -> f64 {
        x.ceil()
    }
}

primitive! {
    math::core_round(x: f64) -> f64 {
        x.round()
    }
}

primitive! {
    math::core_trunc(x: f64) -> f64 {
        x.trunc()
    }
}

primitive! {
    math::core_abs(x: f64) -> f64 {
        x.abs()
    }
}

pub fn core_sign(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    if x > 0.0 {
        1.0_f64.to_raccoon()
    } else if x < 0.0 {
        (-1.0_f64).to_raccoon()
    } else {
        0.0_f64.to_raccoon()
    }
}

primitive! {
    math::core_pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }
}

pub fn register_math_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, math, {
        core_sqrt: 1..=1,
        core_cbrt: 1..=1,
        core_sin: 1..=1,
        core_cos: 1..=1,
        core_tan: 1..=1,
        core_asin: 1..=1,
        core_acos: 1..=1,
        core_atan: 1..=1,
        core_atan2: 2..=2,
        core_sinh: 1..=1,
        core_cosh: 1..=1,
        core_tanh: 1..=1,
        core_exp: 1..=1,
        core_ln: 1..=1,
        core_log10: 1..=1,
        core_log: 2..=2,
        core_floor: 1..=1,
        core_ceil: 1..=1,
        core_round: 1..=1,
        core_trunc: 1..=1,
        core_abs: 1..=1,
        core_sign: 1..=1,
        core_pow: 2..=2,
    });
}

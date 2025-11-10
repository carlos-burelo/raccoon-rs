//! Math context primitives
//! Basic mathematical operations

use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

// Square root
primitive! {
    math::core_sqrt(x: f64) -> f64 {
        x.sqrt()
    }
}

// Cube root
primitive! {
    math::core_cbrt(x: f64) -> f64 {
        x.cbrt()
    }
}

// Sine
primitive! {
    math::core_sin(x: f64) -> f64 {
        x.sin()
    }
}

// Cosine
primitive! {
    math::core_cos(x: f64) -> f64 {
        x.cos()
    }
}

// Tangent
primitive! {
    math::core_tan(x: f64) -> f64 {
        x.tan()
    }
}

// Arc sine
primitive! {
    math::core_asin(x: f64) -> f64 {
        x.asin()
    }
}

// Arc cosine
primitive! {
    math::core_acos(x: f64) -> f64 {
        x.acos()
    }
}

// Arc tangent
primitive! {
    math::core_atan(x: f64) -> f64 {
        x.atan()
    }
}

// Arc tangent of y/x
primitive! {
    math::core_atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }
}

// Hyperbolic sine
primitive! {
    math::core_sinh(x: f64) -> f64 {
        x.sinh()
    }
}

// Hyperbolic cosine
primitive! {
    math::core_cosh(x: f64) -> f64 {
        x.cosh()
    }
}

// Hyperbolic tangent
primitive! {
    math::core_tanh(x: f64) -> f64 {
        x.tanh()
    }
}

// Exponential function (e^x)
primitive! {
    math::core_exp(x: f64) -> f64 {
        x.exp()
    }
}

// Natural logarithm
primitive! {
    math::core_ln(x: f64) -> f64 {
        x.ln()
    }
}

// Base-10 logarithm
primitive! {
    math::core_log10(x: f64) -> f64 {
        x.log10()
    }
}

// Logarithm with custom base
primitive! {
    math::core_log(x: f64, base: f64) -> f64 {
        x.log(base)
    }
}

// Floor function
primitive! {
    math::core_floor(x: f64) -> f64 {
        x.floor()
    }
}

// Ceiling function
primitive! {
    math::core_ceil(x: f64) -> f64 {
        x.ceil()
    }
}

// Round to nearest integer
primitive! {
    math::core_round(x: f64) -> f64 {
        x.round()
    }
}

// Truncate decimal part
primitive! {
    math::core_trunc(x: f64) -> f64 {
        x.trunc()
    }
}

// Absolute value
primitive! {
    math::core_abs(x: f64) -> f64 {
        x.abs()
    }
}

// Sign function (-1, 0, or 1)
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

// Power function (base^exponent)
primitive! {
    math::core_pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }
}

/// Register all math primitives
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

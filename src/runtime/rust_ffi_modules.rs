use crate::ast::types::{FunctionType, PrimitiveType, Type};

use crate::runtime::rust_ffi::{FromRaccoon, RustFFIRegistry, ToRaccoon};
use crate::runtime::values::RuntimeValue;

macro_rules! ffi_fn {
    ($name:expr, $body:expr, $params:expr, $ret:expr) => {
        (
            $name,
            (|args: Vec<RuntimeValue>| -> RuntimeValue { $body(args) })
                as fn(Vec<RuntimeValue>) -> RuntimeValue,
            Type::Function(Box::new(FunctionType {
                params: $params,
                return_type: $ret,
                is_variadic: false,
            })),
        )
    };
}

pub struct RustMathFFI;

impl RustMathFFI {
    pub fn register(registry: &RustFFIRegistry) {
        let (name, func, sig) = ffi_fn!(
            "rust_add",
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
                    (Ok(a), Ok(b)) => (a + b).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int(), PrimitiveType::int()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_multiply",
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
                    (Ok(a), Ok(b)) => (a * b).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int(), PrimitiveType::int()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_power",
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match (f64::from_runtime(&args[0]), f64::from_runtime(&args[1])) {
                    (Ok(base), Ok(exp)) => base.powf(exp).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::float(), PrimitiveType::float()],
            PrimitiveType::float()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_fibonacci",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match i64::from_runtime(&args[0]) {
                    Ok(n) => Self::fibonacci(n).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_factorial",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match i64::from_runtime(&args[0]) {
                    Ok(n) => Self::factorial(n).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_is_prime",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match i64::from_runtime(&args[0]) {
                    Ok(n) => Self::is_prime(n).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int()],
            PrimitiveType::bool()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_gcd",
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match (i64::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
                    (Ok(a), Ok(b)) => Self::gcd(a, b).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::int(), PrimitiveType::int()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);
    }

    fn fibonacci(n: i64) -> i64 {
        if n <= 1 {
            return n;
        }
        let mut a = 0i64;
        let mut b = 1i64;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }

    fn factorial(n: i64) -> i64 {
        if n <= 1 {
            1
        } else {
            n * Self::factorial(n - 1)
        }
    }

    fn is_prime(n: i64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let limit = (n as f64).sqrt() as i64;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }
}

pub struct RustStringFFI;

impl RustStringFFI {
    pub fn register(registry: &RustFFIRegistry) {
        let (name, func, sig) = ffi_fn!(
            "rust_reverse",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match String::from_runtime(&args[0]) {
                    Ok(s) => Self::reverse(s).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::str()],
            PrimitiveType::str()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_to_uppercase",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match String::from_runtime(&args[0]) {
                    Ok(s) => Self::to_uppercase(s).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::str()],
            PrimitiveType::str()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_repeat",
            |args: Vec<RuntimeValue>| {
                if args.len() != 2 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match (String::from_runtime(&args[0]), i64::from_runtime(&args[1])) {
                    (Ok(s), Ok(n)) => Self::repeat(s, n).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::str(), PrimitiveType::int()],
            PrimitiveType::str()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_count_words",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match String::from_runtime(&args[0]) {
                    Ok(s) => Self::count_words(s).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::str()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);
    }

    fn reverse(s: String) -> String {
        s.chars().rev().collect()
    }

    fn to_uppercase(s: String) -> String {
        s.to_uppercase()
    }

    fn repeat(s: String, n: i64) -> String {
        s.repeat(n.max(0) as usize)
    }

    fn count_words(s: String) -> i64 {
        s.split_whitespace().count() as i64
    }
}

pub struct RustArrayFFI;

impl RustArrayFFI {
    pub fn register(registry: &RustFFIRegistry) {
        let (name, func, sig) = ffi_fn!(
            "rust_sum_ints",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match Vec::<i64>::from_runtime(&args[0]) {
                    Ok(numbers) => Self::sum_ints(numbers).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::any()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);

        let (name, func, sig) = ffi_fn!(
            "rust_product_ints",
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(crate::runtime::values::NullValue::new());
                }
                match Vec::<i64>::from_runtime(&args[0]) {
                    Ok(numbers) => Self::product_ints(numbers).to_runtime(),
                    _ => RuntimeValue::Null(crate::runtime::values::NullValue::new()),
                }
            },
            vec![PrimitiveType::any()],
            PrimitiveType::int()
        );
        registry.register_raw(name, func, sig);
    }

    fn sum_ints(numbers: Vec<i64>) -> i64 {
        numbers.iter().sum()
    }

    fn product_ints(numbers: Vec<i64>) -> i64 {
        numbers.iter().product()
    }
}

pub fn register_all_rust_ffi(registry: &RustFFIRegistry) {
    RustMathFFI::register(registry);
    RustStringFFI::register(registry);
    RustArrayFFI::register(registry);
}

use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::{FromRaccoon, Registrar, RuntimeValue, ToRaccoon};

primitive! {
    system::core_print(message: String) -> () {
        print!("{}", message);
    }
}

primitive! {
    system::core_println(message: String) -> () {
        println!("{}", message);
    }
}

primitive! {
    system::core_env_get(name: String) -> String {
        std::env::var(&name).unwrap_or_default()
    }
}

pub fn core_env_set(args: Vec<RuntimeValue>) -> RuntimeValue {
    let name = String::from_raccoon(&args[0]).unwrap_or_default();
    let value = String::from_raccoon(&args[1]).unwrap_or_default();
    std::env::set_var(&name, &value);
    true.to_raccoon()
}

pub fn core_exit(args: Vec<RuntimeValue>) -> RuntimeValue {
    let code = i32::from_raccoon(&args[0]).unwrap_or(0);
    std::process::exit(code);
}

primitive! {
    system::core_random() -> f64 {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let random_state = RandomState::new();
        let mut hasher = random_state.build_hasher();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        hasher.write_u128(now);
        let hash = hasher.finish();


        hash as f64 / u64::MAX as f64
    }
}

pub fn register_system_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, system, {
        core_print: 1..=1,
        core_println: 1..=1,
        core_env_get: 1..=1,
        core_env_set: 2..=2,
        core_exit: 1..=1,
        core_random: 0..=0,
    });
}

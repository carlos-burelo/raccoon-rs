use crate::primitive;
use crate::register_context_primitives;
use crate::runtime::Registrar;

primitive! {
    time::core_time_now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64
    }
}

primitive! {
    time::core_time_now_micros() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as i64
    }
}

primitive! {
    system::core_sleep(ms: i64) -> () {
        if ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        }
    }
}

pub fn register_time_primitives(registrar: &mut Registrar) {
    register_context_primitives!(registrar, time, {
        core_time_now: 0..=0,
        core_time_now_micros: 0..=0,
        core_sleep: 1..=1,
    });
}

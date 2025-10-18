use crate::ast::types::{PrimitiveType, Type};
use crate::runtime::values::{IntValue, NullValue, RuntimeValue, StrValue};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimeModule;

impl TimeModule {
    pub fn name() -> &'static str {
        "std:time"
    }

    pub fn get_exports() -> HashMap<String, RuntimeValue> {
        let mut exports = HashMap::new();

        exports.insert("now".to_string(), Self::create_now_fn());
        exports.insert("formatDate".to_string(), Self::create_format_date_fn());
        exports.insert("year".to_string(), Self::create_year_fn());
        exports.insert("month".to_string(), Self::create_month_fn());
        exports.insert("day".to_string(), Self::create_day_fn());
        exports.insert("hour".to_string(), Self::create_hour_fn());
        exports.insert("minute".to_string(), Self::create_minute_fn());
        exports.insert("second".to_string(), Self::create_second_fn());
        exports.insert("sleep".to_string(), Self::create_sleep_fn());
        exports.insert("millis".to_string(), Self::create_millis_fn());
        exports.insert("micros".to_string(), Self::create_micros_fn());
        exports.insert("nanos".to_string(), Self::create_nanos_fn());

        exports
    }

    pub fn get_export(name: &str) -> Option<RuntimeValue> {
        match name {
            "now" => Some(Self::create_now_fn()),
            "formatDate" => Some(Self::create_format_date_fn()),
            "year" => Some(Self::create_year_fn()),
            "month" => Some(Self::create_month_fn()),
            "day" => Some(Self::create_day_fn()),
            "hour" => Some(Self::create_hour_fn()),
            "minute" => Some(Self::create_minute_fn()),
            "second" => Some(Self::create_second_fn()),
            "sleep" => Some(Self::create_sleep_fn()),
            "millis" => Some(Self::create_millis_fn()),
            "micros" => Some(Self::create_micros_fn()),
            "nanos" => Some(Self::create_nanos_fn()),
            _ => None,
        }
    }

    fn create_now_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                RuntimeValue::Int(IntValue::new(timestamp))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_format_date_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let timestamp = if args.is_empty() {
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                } else {
                    match &args[0] {
                        RuntimeValue::Int(i) => i.value as u64,
                        _ => return RuntimeValue::Null(NullValue::new()),
                    }
                };

                // Simple ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
                // Use chrono-like calculation for date/time components
                let days_since_epoch = timestamp / 86400;
                let secs_today = timestamp % 86400;

                let hours = secs_today / 3600;
                let minutes = (secs_today % 3600) / 60;
                let seconds = secs_today % 60;

                // Simplified date calculation (approximation)
                // This is a basic implementation - a real one would use chrono crate
                let mut year = 1970;
                let mut remaining_days = days_since_epoch;

                loop {
                    let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
                    if remaining_days < days_in_year {
                        break;
                    }
                    remaining_days -= days_in_year;
                    year += 1;
                }

                let mut month = 1;
                let mut day = remaining_days + 1;

                let days_in_months = Self::get_days_in_months(year);
                for (m, &days) in days_in_months.iter().enumerate() {
                    if day <= days {
                        month = m + 1;
                        break;
                    }
                    day -= days;
                }

                let formatted = format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                    year, month, day, hours, minutes, seconds
                );

                RuntimeValue::Str(StrValue::new(formatted))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::int()],
                return_type: PrimitiveType::str(),
                is_variadic: false,
            })),
        ))
    }

    fn create_year_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let days_since_epoch = timestamp / 86400;
                let mut year = 1970;
                let mut remaining_days = days_since_epoch;

                loop {
                    let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
                    if remaining_days < days_in_year {
                        break;
                    }
                    remaining_days -= days_in_year;
                    year += 1;
                }

                RuntimeValue::Int(IntValue::new(year as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_month_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let days_since_epoch = timestamp / 86400;
                let mut year = 1970;
                let mut remaining_days = days_since_epoch;

                loop {
                    let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
                    if remaining_days < days_in_year {
                        break;
                    }
                    remaining_days -= days_in_year;
                    year += 1;
                }

                let mut month = 1;
                let mut day = remaining_days + 1;

                let days_in_months = Self::get_days_in_months(year);
                for (m, &days) in days_in_months.iter().enumerate() {
                    if day <= days {
                        month = m + 1;
                        break;
                    }
                    day -= days;
                }

                RuntimeValue::Int(IntValue::new(month as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_day_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let days_since_epoch = timestamp / 86400;
                let mut year = 1970;
                let mut remaining_days = days_since_epoch;

                loop {
                    let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
                    if remaining_days < days_in_year {
                        break;
                    }
                    remaining_days -= days_in_year;
                    year += 1;
                }

                let mut day = remaining_days + 1;
                let days_in_months = Self::get_days_in_months(year);
                for &days in &days_in_months {
                    if day <= days {
                        break;
                    }
                    day -= days;
                }

                RuntimeValue::Int(IntValue::new(day as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_hour_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let secs_today = timestamp % 86400;
                let hours = secs_today / 3600;

                RuntimeValue::Int(IntValue::new(hours as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_minute_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let secs_today = timestamp % 86400;
                let minutes = (secs_today % 3600) / 60;

                RuntimeValue::Int(IntValue::new(minutes as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_second_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let seconds = timestamp % 60;

                RuntimeValue::Int(IntValue::new(seconds as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn is_leap_year(year: u64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn get_days_in_months(year: u64) -> Vec<u64> {
        let feb_days = if Self::is_leap_year(year) { 29 } else { 28 };
        vec![31, feb_days, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    }

    fn create_sleep_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.len() != 1 {
                    return RuntimeValue::Null(NullValue::new());
                }
                match &args[0] {
                    RuntimeValue::Int(millis) => {
                        if millis.value > 0 {
                            std::thread::sleep(std::time::Duration::from_millis(
                                millis.value as u64,
                            ));
                        }
                        RuntimeValue::Null(NullValue::new())
                    }
                    _ => RuntimeValue::Null(NullValue::new()),
                }
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![PrimitiveType::int()],
                return_type: PrimitiveType::null(),
                is_variadic: false,
            })),
        ))
    }

    fn create_millis_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                RuntimeValue::Int(IntValue::new(timestamp as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_micros_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                RuntimeValue::Int(IntValue::new(timestamp as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }

    fn create_nanos_fn() -> RuntimeValue {
        RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
            |_args: Vec<RuntimeValue>| {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                RuntimeValue::Int(IntValue::new(timestamp as i64))
            },
            Type::Function(Box::new(crate::ast::types::FunctionType {
                params: vec![],
                return_type: PrimitiveType::int(),
                is_variadic: false,
            })),
        ))
    }
}

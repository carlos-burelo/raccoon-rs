use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::TypeHandler;
use crate::runtime::{ListValue, RuntimeValue, StrValue};
use crate::tokens::Position;
use async_trait::async_trait;

pub struct StrType;

#[async_trait]
impl TypeHandler for StrType {
    fn type_name(&self) -> &str {
        "str"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        let s = match value {
            RuntimeValue::Str(s) => &s.value,
            _ => {
                return Err(RaccoonError::new(
                    format!("Expected str, got {}", value.get_name()),
                    position,
                    file,
                ));
            }
        };

        match method {
            "toUpper" | "toUpperCase" => Ok(RuntimeValue::Str(StrValue::new(s.to_uppercase()))),
            "toLower" | "toLowerCase" => Ok(RuntimeValue::Str(StrValue::new(s.to_lowercase()))),
            "trim" => Ok(RuntimeValue::Str(StrValue::new(s.trim().to_string()))),
            "trimStart" | "trimLeft" => {
                Ok(RuntimeValue::Str(StrValue::new(s.trim_start().to_string())))
            }
            "trimEnd" | "trimRight" => {
                Ok(RuntimeValue::Str(StrValue::new(s.trim_end().to_string())))
            }
            "split" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "split requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(sep) = &args[0] {
                    let parts: Vec<RuntimeValue> = s
                        .split(&sep.value)
                        .map(|p| RuntimeValue::Str(StrValue::new(p.to_string())))
                        .collect();
                    Ok(RuntimeValue::List(ListValue::new(
                        parts,
                        PrimitiveType::str(),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "split requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "replace" => {
                if args.len() != 2 {
                    return Err(RaccoonError::new(
                        "replace requires 2 arguments (search, replacement)".to_string(),
                        position,
                        file,
                    ));
                }
                match (&args[0], &args[1]) {
                    (RuntimeValue::Str(search), RuntimeValue::Str(replacement)) => {
                        Ok(RuntimeValue::Str(StrValue::new(
                            s.replace(&search.value, &replacement.value),
                        )))
                    }
                    _ => Err(RaccoonError::new(
                        "replace requires two string arguments".to_string(),
                        position,
                        file,
                    )),
                }
            }
            "startsWith" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "startsWith requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(prefix) = &args[0] {
                    Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                        s.starts_with(&prefix.value),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "startsWith requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "endsWith" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "endsWith requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(suffix) = &args[0] {
                    Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                        s.ends_with(&suffix.value),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "endsWith requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "contains" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "contains requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(substr) = &args[0] {
                    Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                        s.contains(&substr.value),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "contains requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "toStr" => Ok(RuntimeValue::Str(StrValue::new(s.to_string()))),
            "join" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "join requires 1 argument (separator)".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(sep) = &args[0] {
                    let joined = s
                        .chars()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(&sep.value);
                    Ok(RuntimeValue::Str(StrValue::new(joined)))
                } else {
                    Err(RaccoonError::new(
                        "join requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }
            "slice" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(RaccoonError::new(
                        "slice requires 1 or 2 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                let start = match &args[0] {
                    RuntimeValue::Int(i) => i.value as isize,
                    _ => {
                        return Err(RaccoonError::new(
                            "slice requires integer arguments".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let end = if args.len() == 2 {
                    match &args[1] {
                        RuntimeValue::Int(i) => Some(i.value as isize),
                        _ => {
                            return Err(RaccoonError::new(
                                "slice requires integer arguments".to_string(),
                                position,
                                file,
                            ));
                        }
                    }
                } else {
                    None
                };

                let len = s.len() as isize;
                let real_start = if start < 0 { len + start } else { start };
                let real_end = match end {
                    Some(e) => {
                        if e < 0 {
                            len + e
                        } else {
                            e
                        }
                    }
                    None => len,
                };

                if real_start < 0 || real_end > len || real_start > real_end {
                    return Err(RaccoonError::new(
                        "slice indices out of bounds".to_string(),
                        position,
                        file,
                    ));
                }

                Ok(RuntimeValue::Str(StrValue::new(
                    s[real_start as usize..real_end as usize].to_string(),
                )))
            }

            "indexOf" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "indexOf requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(substr) = &args[0] {
                    if let Some(index) = s.find(&substr.value) {
                        Ok(RuntimeValue::Int(crate::runtime::IntValue::new(
                            index as i64,
                        )))
                    } else {
                        Ok(RuntimeValue::Int(crate::runtime::IntValue::new(-1)))
                    }
                } else {
                    Err(RaccoonError::new(
                        "indexOf requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }

            "lastIndexOf" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "lastIndexOf requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Str(substr) = &args[0] {
                    if let Some(index) = s.rfind(&substr.value) {
                        Ok(RuntimeValue::Int(crate::runtime::IntValue::new(
                            index as i64,
                        )))
                    } else {
                        Ok(RuntimeValue::Int(crate::runtime::IntValue::new(-1)))
                    }
                } else {
                    Err(RaccoonError::new(
                        "lastIndexOf requires string argument".to_string(),
                        position,
                        file,
                    ))
                }
            }

            "repeat" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "repeat requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Int(count) = &args[0] {
                    if count.value < 0 {
                        return Err(RaccoonError::new(
                            "repeat count must be non-negative".to_string(),
                            position,
                            file,
                        ));
                    }
                    Ok(RuntimeValue::Str(StrValue::new(
                        s.repeat(count.value as usize),
                    )))
                } else {
                    Err(RaccoonError::new(
                        "repeat requires integer argument".to_string(),
                        position,
                        file,
                    ))
                }
            }

            "padStart" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(RaccoonError::new(
                        "padStart requires 1 or 2 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                let target_len = match &args[0] {
                    RuntimeValue::Int(i) => i.value as usize,
                    _ => {
                        return Err(RaccoonError::new(
                            "padStart requires integer as first argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let pad_str = if args.len() == 2 {
                    match &args[1] {
                        RuntimeValue::Str(p) => p.value.clone(),
                        _ => {
                            return Err(RaccoonError::new(
                                "padStart requires string as second argument".to_string(),
                                position,
                                file,
                            ));
                        }
                    }
                } else {
                    " ".to_string()
                };

                if target_len <= s.len() || pad_str.is_empty() {
                    return Ok(RuntimeValue::Str(StrValue::new(s.to_string())));
                }

                let pad_needed = target_len - s.len();
                let pad_count = (pad_needed + pad_str.len() - 1) / pad_str.len();
                let padding = pad_str.repeat(pad_count);
                let result = format!("{}{}", &padding[..pad_needed], s);
                Ok(RuntimeValue::Str(StrValue::new(result)))
            }

            "padEnd" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(RaccoonError::new(
                        "padEnd requires 1 or 2 arguments".to_string(),
                        position,
                        file,
                    ));
                }
                let target_len = match &args[0] {
                    RuntimeValue::Int(i) => i.value as usize,
                    _ => {
                        return Err(RaccoonError::new(
                            "padEnd requires integer as first argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let pad_str = if args.len() == 2 {
                    match &args[1] {
                        RuntimeValue::Str(p) => p.value.clone(),
                        _ => {
                            return Err(RaccoonError::new(
                                "padEnd requires string as second argument".to_string(),
                                position,
                                file,
                            ));
                        }
                    }
                } else {
                    " ".to_string()
                };

                if target_len <= s.len() || pad_str.is_empty() {
                    return Ok(RuntimeValue::Str(StrValue::new(s.to_string())));
                }

                let pad_needed = target_len - s.len();
                let pad_count = (pad_needed + pad_str.len() - 1) / pad_str.len();
                let padding = pad_str.repeat(pad_count);
                let result = format!("{}{}", s, &padding[..pad_needed]);
                Ok(RuntimeValue::Str(StrValue::new(result)))
            }

            "charCodeAt" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "charCodeAt requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                if let RuntimeValue::Int(index) = &args[0] {
                    let idx = index.value as usize;
                    if idx >= s.len() {
                        return Err(RaccoonError::new(
                            "charCodeAt index out of bounds".to_string(),
                            position,
                            file,
                        ));
                    }
                    if let Some(ch) = s.chars().nth(idx) {
                        Ok(RuntimeValue::Int(crate::runtime::IntValue::new(ch as i64)))
                    } else {
                        Err(RaccoonError::new(
                            "charCodeAt index out of bounds".to_string(),
                            position,
                            file,
                        ))
                    }
                } else {
                    Err(RaccoonError::new(
                        "charCodeAt requires integer argument".to_string(),
                        position,
                        file,
                    ))
                }
            }

            _ => Err(RaccoonError::new(
                format!("Method '{}' not found on string", method),
                position,
                file,
            )),
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "isNullOrEmpty" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "isNullOrEmpty requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Null(_) => {
                        Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(true)))
                    }
                    RuntimeValue::Str(s) => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                        s.value.is_empty(),
                    ))),
                    _ => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                }
            }
            "isNullOrWhiteSpace" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "isNullOrWhiteSpace requires 1 argument".to_string(),
                        position,
                        file,
                    ));
                }
                match &args[0] {
                    RuntimeValue::Null(_) => {
                        Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(true)))
                    }
                    RuntimeValue::Str(s) => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(
                        s.value.trim().is_empty(),
                    ))),
                    _ => Ok(RuntimeValue::Bool(crate::runtime::BoolValue::new(false))),
                }
            }
            "concat" => {
                let parts: Vec<String> = args
                    .iter()
                    .map(|v| match v {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => v.to_string(),
                    })
                    .collect();
                Ok(RuntimeValue::Str(StrValue::new(parts.join(""))))
            }
            "join" => {
                if args.len() < 2 {
                    return Err(RaccoonError::new(
                        "join requires at least 2 arguments (separator, values...)".to_string(),
                        position,
                        file,
                    ));
                }
                let separator = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "join requires string separator as first argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };
                let parts: Vec<String> = args[1..]
                    .iter()
                    .map(|v| match v {
                        RuntimeValue::Str(s) => s.value.clone(),
                        RuntimeValue::List(l) => l
                            .elements
                            .iter()
                            .map(|e| match e {
                                RuntimeValue::Str(s) => s.value.clone(),
                                _ => e.to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(separator),
                        _ => v.to_string(),
                    })
                    .collect();
                Ok(RuntimeValue::Str(StrValue::new(parts.join(separator))))
            }
            "format" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "format requires at least 1 argument (format string)".to_string(),
                        position,
                        file,
                    ));
                }
                let format_str = match &args[0] {
                    RuntimeValue::Str(s) => &s.value,
                    _ => {
                        return Err(RaccoonError::new(
                            "format requires string as first argument".to_string(),
                            position,
                            file,
                        ));
                    }
                };

                let mut result = format_str.clone();
                for (i, arg) in args[1..].iter().enumerate() {
                    let placeholder = format!("{{{}}}", i);
                    let value = match arg {
                        RuntimeValue::Str(s) => s.value.clone(),
                        _ => arg.to_string(),
                    };
                    result = result.replace(&placeholder, &value);
                }
                Ok(RuntimeValue::Str(StrValue::new(result)))
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on str type", method),
                position,
                file,
            )),
        }
    }

    fn get_static_property(
        &self,
        property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match property {
            "empty" => Ok(RuntimeValue::Str(StrValue::new(String::new()))),
            _ => Err(RaccoonError::new(
                format!("Static property '{}' not found on str type", property),
                position,
                file,
            )),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toUpper"
                | "toUpperCase"
                | "toLower"
                | "toLowerCase"
                | "trim"
                | "trimStart"
                | "trimLeft"
                | "trimEnd"
                | "trimRight"
                | "split"
                | "replace"
                | "startsWith"
                | "endsWith"
                | "contains"
                | "indexOf"
                | "slice"
                | "join"
                | "toStr"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isNullOrEmpty" | "isNullOrWhiteSpace" | "concat" | "join" | "format"
        )
    }
}

/// StrType - String primitive type with text manipulation methods
use crate::ast::types::PrimitiveType;
use crate::error::RaccoonError;
use crate::runtime::types::helpers::*;
use crate::runtime::types::TypeHandler;
use crate::runtime::{BoolValue, IntValue, ArrayValue, RuntimeValue, StrValue};
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
        let s = extract_str(value, "this", position, file.clone())?;

        match method {
            // Case conversion
            "toUpper" | "toUpperCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.to_uppercase())))
            }
            "toLower" | "toLowerCase" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.to_lowercase())))
            }

            // Trimming
            "trim" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.trim().to_string())))
            }
            "trimStart" | "trimLeft" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.trim_start().to_string())))
            }
            "trimEnd" | "trimRight" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.trim_end().to_string())))
            }

            // Splitting
            "split" => {
                require_args(&args, 1, method, position, file.clone())?;
                let separator = extract_str(&args[0], "separator", position, file)?;
                let parts: Vec<RuntimeValue> = s
                    .split(separator)
                    .map(|p| RuntimeValue::Str(StrValue::new(p.to_string())))
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    parts,
                    PrimitiveType::str(),
                )))
            }

            // Joining
            "join" => {
                require_args(&args, 1, method, position, file.clone())?;
                let separator = extract_str(&args[0], "separator", position, file)?;
                let joined = s
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(separator);
                Ok(RuntimeValue::Str(StrValue::new(joined)))
            }

            // Replacement
            "replace" | "replaceAll" => {
                require_args(&args, 2, method, position, file.clone())?;
                let search = extract_str(&args[0], "search", position, file.clone())?;
                let replacement = extract_str(&args[1], "replacement", position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(
                    s.replace(search, replacement),
                )))
            }

            // Searching
            "startsWith" => {
                require_args(&args, 1, method, position, file.clone())?;
                let prefix = extract_str(&args[0], "prefix", position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(s.starts_with(prefix))))
            }
            "endsWith" => {
                require_args(&args, 1, method, position, file.clone())?;
                let suffix = extract_str(&args[0], "suffix", position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(s.ends_with(suffix))))
            }
            "contains" => {
                require_args(&args, 1, method, position, file.clone())?;
                let substring = extract_str(&args[0], "substring", position, file)?;
                Ok(RuntimeValue::Bool(BoolValue::new(s.contains(substring))))
            }
            "indexOf" => {
                require_args(&args, 1, method, position, file.clone())?;
                let substring = extract_str(&args[0], "substring", position, file)?;
                let index = s.find(substring).map(|i| i as i64).unwrap_or(-1);
                Ok(RuntimeValue::Int(IntValue::new(index)))
            }
            "lastIndexOf" => {
                require_args(&args, 1, method, position, file.clone())?;
                let substring = extract_str(&args[0], "substring", position, file)?;
                let index = s.rfind(substring).map(|i| i as i64).unwrap_or(-1);
                Ok(RuntimeValue::Int(IntValue::new(index)))
            }

            // Slicing
            "slice" => {
                require_args_range(&args, 1, 2, method, position, file.clone())?;
                let start = extract_int(&args[0], "start", position, file.clone())? as isize;
                let end = if args.len() == 2 {
                    Some(extract_int(&args[1], "end", position, file.clone())? as isize)
                } else {
                    None
                };

                let len = s.len() as isize;
                let real_start = if start < 0 {
                    (len + start).max(0)
                } else {
                    start.min(len)
                };
                let real_end = end
                    .map(|e| if e < 0 { (len + e).max(0) } else { e.min(len) })
                    .unwrap_or(len);

                if real_start > real_end {
                    return Err(RaccoonError::new(
                        "slice: start index cannot be greater than end index".to_string(),
                        position,
                        file,
                    ));
                }

                Ok(RuntimeValue::Str(StrValue::new(
                    s[real_start as usize..real_end as usize].to_string(),
                )))
            }
            "substring" => {
                require_args_range(&args, 1, 2, method, position, file.clone())?;
                let start = extract_int(&args[0], "start", position, file.clone())?.max(0) as usize;
                let end = if args.len() == 2 {
                    Some(extract_int(&args[1], "end", position, file.clone())?.max(0) as usize)
                } else {
                    None
                };

                let len = s.len();
                let real_start = start.min(len);
                let real_end = end.unwrap_or(len).min(len);

                if real_start <= real_end {
                    Ok(RuntimeValue::Str(StrValue::new(
                        s[real_start..real_end].to_string(),
                    )))
                } else {
                    Ok(RuntimeValue::Str(StrValue::new(
                        s[real_end..real_start].to_string(),
                    )))
                }
            }

            // Character access
            "charAt" => {
                require_args(&args, 1, method, position, file.clone())?;
                let index = extract_int(&args[0], "index", position, file)? as usize;
                let ch = s
                    .chars()
                    .nth(index)
                    .map(|c| c.to_string())
                    .unwrap_or_default();
                Ok(RuntimeValue::Str(StrValue::new(ch)))
            }
            "charCodeAt" => {
                require_args(&args, 1, method, position, file.clone())?;
                let index = extract_int(&args[0], "index", position, file.clone())? as usize;
                match s.chars().nth(index) {
                    Some(ch) => Ok(RuntimeValue::Int(IntValue::new(ch as i64))),
                    None => Err(RaccoonError::new(
                        format!("charCodeAt: index {} out of bounds", index),
                        position,
                        file,
                    )),
                }
            }

            // Padding
            "padStart" => {
                require_args_range(&args, 1, 2, method, position, file.clone())?;
                let target_len =
                    extract_int(&args[0], "targetLength", position, file.clone())? as usize;
                let pad_str = if args.len() == 2 {
                    extract_str(&args[1], "padString", position, file)?.to_string()
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
                require_args_range(&args, 1, 2, method, position, file.clone())?;
                let target_len =
                    extract_int(&args[0], "targetLength", position, file.clone())? as usize;
                let pad_str = if args.len() == 2 {
                    extract_str(&args[1], "padString", position, file)?.to_string()
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

            // Other transformations
            "repeat" => {
                require_args(&args, 1, method, position, file.clone())?;
                let count = extract_int(&args[0], "count", position, file.clone())?;
                if count < 0 {
                    return Err(RaccoonError::new(
                        "repeat: count must be non-negative".to_string(),
                        position,
                        file,
                    ));
                }
                Ok(RuntimeValue::Str(StrValue::new(s.repeat(count as usize))))
            }
            "reverse" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.chars().rev().collect())))
            }
            "match" => {
                require_args(&args, 1, method, position, file.clone())?;
                let pattern = extract_str(&args[0], "pattern", position, file)?;
                let matches: Vec<RuntimeValue> = s
                    .match_indices(pattern)
                    .map(|(_, m)| RuntimeValue::Str(StrValue::new(m.to_string())))
                    .collect();
                Ok(RuntimeValue::Array(ArrayValue::new(
                    matches,
                    PrimitiveType::str(),
                )))
            }

            // Conversion
            "toStr" => {
                require_args(&args, 0, method, position, file)?;
                Ok(RuntimeValue::Str(StrValue::new(s.to_string())))
            }

            _ => Err(method_not_found_error("str", method, position, file)),
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
                require_args(&args, 1, method, position, file)?;
                let result = match &args[0] {
                    RuntimeValue::Null(_) => true,
                    RuntimeValue::Str(s) => s.value.is_empty(),
                    _ => false,
                };
                Ok(RuntimeValue::Bool(BoolValue::new(result)))
            }
            "isNullOrWhiteSpace" => {
                require_args(&args, 1, method, position, file)?;
                let result = match &args[0] {
                    RuntimeValue::Null(_) => true,
                    RuntimeValue::Str(s) => s.value.trim().is_empty(),
                    _ => false,
                };
                Ok(RuntimeValue::Bool(BoolValue::new(result)))
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
                require_min_args(&args, 2, method, position, file.clone())?;
                let separator = extract_str(&args[0], "separator", position, file)?;
                let parts: Vec<String> = args[1..]
                    .iter()
                    .map(|v| match v {
                        RuntimeValue::Str(s) => s.value.clone(),
                        RuntimeValue::Array(l) => l
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
                require_min_args(&args, 1, method, position, file.clone())?;
                let format_str = extract_str(&args[0], "template", position, file)?;
                let mut result = format_str.to_string();
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
            _ => Err(static_method_not_found_error("str", method, position, file)),
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
            _ => Err(property_not_found_error("str", property, position, file)),
        }
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "toUpper" | "toUpperCase" | "toLower" | "toLowerCase" | "trim" | "trimStart"
                | "trimLeft" | "trimEnd" | "trimRight" | "split" | "join" | "replace"
                | "replaceAll" | "startsWith" | "endsWith" | "contains" | "indexOf"
                | "lastIndexOf" | "slice" | "substring" | "charAt" | "charCodeAt"
                | "padStart" | "padEnd" | "repeat" | "reverse" | "match" | "toStr"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(
            method,
            "isNullOrEmpty" | "isNullOrWhiteSpace" | "concat" | "join" | "format"
        )
    }

    fn has_async_instance_method(&self, _method: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_upper() {
        let handler = StrType;
        let mut value = RuntimeValue::Str(StrValue::new("hello".to_string()));
        let result = handler
            .call_instance_method(&mut value, "toUpper", vec![], Position::default(), None)
            .unwrap();

        match result {
            RuntimeValue::Str(s) => assert_eq!(s.value, "HELLO"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_str_split() {
        let handler = StrType;
        let mut value = RuntimeValue::Str(StrValue::new("a,b,c".to_string()));
        let result = handler
            .call_instance_method(
                &mut value,
                "split",
                vec![RuntimeValue::Str(StrValue::new(",".to_string()))],
                Position::default(),
                None,
            )
            .unwrap();

        match result {
            RuntimeValue::Array(l) => assert_eq!(l.elements.len(), 3),
            _ => panic!("Expected list"),
        }
    }

}

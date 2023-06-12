use crate::{store::Store, utils::GenericResult, error::CustomError};

fn call_func<T: Store>(store: &T, args: &Vec<&str>) -> GenericResult<String> {
    match *args.get(0).ok_or(CustomError::new("too few arguments"))? {
        "path_segment" => {
            let var_name = *args.get(1).ok_or(CustomError::new("too few arguments"))?;
            let index = i32::from_str_radix(*args.get(2).ok_or(CustomError::new("too few arguments"))?, 10)?;
            if let Some(var) = store.get(var_name) {
                let splitted: Vec<&str> = var.split("/").collect();
                let index = if index < 0 { splitted.len() as i32 + index } else { index };
                if index < 0 || splitted.len() <= index as usize {
                    return Err(Box::new(CustomError::new("index out of range")));
                }
                let v = *splitted.get(index as usize).ok_or(CustomError::new("Index out of range"))?;
                Ok(v.to_owned())
            } else {
                return Ok(String::default());
            }
        },
        _ => Err(Box::new(CustomError::new("unknown function"))),
    }
}

pub fn expand_template<T: Store>(store: &T, template: &str) -> String {
    let mut result = String::new();
    let mut remain_text = &template[0..];

    while remain_text.len() > 0 {
        if let Some(index) = remain_text.find('$') {
            result.push_str(&remain_text[0..index]);
            remain_text = &remain_text[(index + 1)..];

            match remain_text.chars().next() {
                None => {
                    break;
                },
                Some('$') => {
                    result.push_str("$");
                    remain_text = &remain_text[1..];
                },
                Some('{') => {
                    remain_text = &remain_text[1..];
                    if let Some((var_name, remain)) = remain_text.split_once("}") {
                        let val = store.get(var_name).unwrap_or(Default::default());
                        result.push_str(&val);
                        remain_text = remain;
                    } else {
                        break;
                    }
                },
                Some('(') => {
                    remain_text = &remain_text[1..];
                    if let Some((content, remain)) = remain_text.split_once(")") {
                        let parsed: Vec<&str> = content.split(" ").collect();
                        if let Ok(v) = call_func(store, &parsed) {
                            result.push_str(&v);
                        }
                        remain_text = remain;
                    } else {
                        break;
                    }
                },
                Some(_) => {}
            }
        } else {
            result.push_str(remain_text);
            break;
        }
    }

    result
}

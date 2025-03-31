use crate::parser::Token;
use crate::stack::Stack;
use std::collections::HashMap;

pub fn execute_tokens<'a>(
    stack: &mut Stack,
    tokens: &'a [Token],
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<(), String> {
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => {
                handle_number(stack, *n)?;
                i += 1;
            }
            Token::StringLiteral(s) => {
                handle_string_literal(s);
                i += 1;
            }
            Token::Word(word) => {
                i = handle_word_token(stack, word, tokens, i, dict)?;
            }
        }
    }
    Ok(())
}

fn handle_number(stack: &mut Stack, number: i16) -> Result<(), String> {
    stack.push(number)
}

fn handle_string_literal(s: &str) {
    print!("{}", s);
}

fn handle_word_token<'a>(
    stack: &mut Stack,
    word: &str,
    tokens: &'a [Token],
    i: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    let word_upper = word.to_uppercase();
    if word_upper == ":" {
        handle_definition(tokens, i, dict)
    } else if let Some(def_tokens) = dict.get(&word_upper) {
        execute_tokens(stack, def_tokens, dict)?;
        Ok(i + 1)
    } else if word_upper == "IF" {
        execute_conditional(stack, tokens, i, dict)
    } else {
        handle_word(stack, word)?;
        Ok(i + 1)
    }
}

fn handle_definition<'a>(
    tokens: &'a [Token],
    mut i: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    i += 1;
    if i >= tokens.len() {
        return Err("invalid-word".to_string());
    }
    let name_token = &tokens[i];
    let name = if let Token::Word(w) = name_token {
        w.to_uppercase()
    } else {
        return Err("invalid-word".to_string());
    };
    if name.parse::<i16>().is_ok() {
        return Err("invalid-word".to_string());
    }
    i += 1;
    let start = i;
    while i < tokens.len() {
        if let Token::Word(ref w) = tokens[i] {
            if w.to_uppercase() == ";" {
                break;
            }
        }
        i += 1;
    }
    if i == tokens.len() {
        return Err("invalid-word".to_string());
    }
    dict.insert(name, &tokens[start..i]); // Almacena una referencia al slice
    Ok(i + 1)
}

fn execute_conditional<'a>(
    stack: &mut Stack,
    tokens: &'a [Token],
    if_index: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    let (else_index, then_index) = find_else_then_indices(tokens, if_index)?;

    let cond = stack.pop()?;
    let condition_true = cond != 0;

    let then_idx = then_index.unwrap();
    if condition_true {
        let end = else_index.unwrap_or(then_idx);
        let branch_tokens = &tokens[if_index + 1..end];
        execute_tokens(stack, branch_tokens, dict)?;
    } else if let Some(else_idx) = else_index {
        let branch_tokens = &tokens[else_idx + 1..then_idx];
        execute_tokens(stack, branch_tokens, dict)?;
    }
    Ok(then_idx + 1)
}

fn find_else_then_indices(
    tokens: &[Token],
    if_index: usize,
) -> Result<(Option<usize>, Option<usize>), String> {
    let mut else_index: Option<usize> = None;
    let mut then_index: Option<usize> = None;
    let mut j = if_index + 1;

    while j < tokens.len() {
        if let Token::Word(ref w) = tokens[j] {
            let w_upper = w.to_uppercase();
            if w_upper == "THEN" {
                then_index = Some(j);
                break;
            } else if w_upper == "ELSE" {
                else_index = Some(j);
            }
        }
        j += 1;
    }

    if then_index.is_none() {
        return Err("Estructura IF sin THEN".to_string());
    }
    Ok((else_index, then_index))
}

fn handle_word(stack: &mut Stack, word: &str) -> Result<(), String> {
    match word.to_uppercase().as_str() {
        "+" => apply_binary_op(stack, |a, b| a + b),
        "-" => apply_binary_op(stack, |a, b| a - b),
        "*" => apply_binary_op(stack, |a, b| a * b),
        "/" => {
            let b = stack.pop()?;
            if b == 0 {
                return Err("division-by-zero".to_string());
            }
            let a = stack.pop()?;
            stack.push(a / b)
        }
        "DUP" => {
            let val = stack.peek().map_err(|e| e.to_string())?;
            stack.push(val).map_err(|e| e.to_string())
        }
        "DROP" => {
            stack.pop().map_err(|e| e.to_string())?;
            Ok(())
        }
        "SWAP" => {
            let b = stack.pop().map_err(|e| e.to_string())?;
            let a = stack.pop().map_err(|e| e.to_string())?;
            stack.push(b).map_err(|e| e.to_string())?;
            stack.push(a).map_err(|e| e.to_string())
        }
        "OVER" => {
            let val = stack.peek_n(1).map_err(|e| e.to_string())?;
            stack.push(val).map_err(|e| e.to_string())
        }
        "ROT" => {
            let c = stack.pop().map_err(|e| e.to_string())?;
            let b = stack.pop().map_err(|e| e.to_string())?;
            let a = stack.pop().map_err(|e| e.to_string())?;
            stack.push(b).map_err(|e| e.to_string())?;
            stack.push(c).map_err(|e| e.to_string())?;
            stack.push(a).map_err(|e| e.to_string())
        }
        "=" => apply_binary_op(stack, |a, b| if a == b { -1 } else { 0 }),
        "<" => apply_binary_op(stack, |a, b| if a < b { -1 } else { 0 }),
        ">" => apply_binary_op(stack, |a, b| if a > b { -1 } else { 0 }),
        "AND" => apply_binary_op(stack, |a, b| if a != 0 && b != 0 { -1 } else { 0 }),
        "OR" => apply_binary_op(stack, |a, b| if a != 0 || b != 0 { -1 } else { 0 }),
        "NOT" => {
            let a = stack.pop()?;
            let result = if a == 0 { -1 } else { 0 };
            stack.push(result)
        }
        "EMIT" => {
            let code = stack.pop()?;
            let c = std::char::from_u32(code as u32)
                .ok_or_else(|| "Valor para EMIT no es un carácter válido".to_string())?;
            print!("{}", c);
            Ok(())
        }
        "." => {
            let val = stack.pop().map_err(|e| e.to_string())?;
            print!("{} ", val);
            Ok(())
        }
        "CR" => {
            println!();
            Ok(())
        }
        _ => Err("?".to_string()),
    }
}

fn apply_binary_op<F>(stack: &mut Stack, op: F) -> Result<(), String>
where
    F: Fn(i16, i16) -> i16,
{
    let b = stack.pop().map_err(|e| e.to_string())?;
    let a = stack.pop().map_err(|e| e.to_string())?;
    stack.push(op(a, b)).map_err(|e| e.to_string())
}

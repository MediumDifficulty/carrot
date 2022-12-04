use crate::{tokeniser::Token, util::ErrStr};

pub fn funcify(tokens: Vec<Token>) -> Result<Vec<FunctionNode>, String> {
    let functions = split_functions(tokens)
        .iter()
        .map(|func| parse_function(func.clone()))
        .collect::<Vec<Result<FunctionNode, String>>>();

    let mut functions_out = Vec::new();
    for function in functions {
        functions_out.push(function?)
    }
    
    Ok(functions_out)
}

fn parse_function(tokens: Vec<Token>) -> Result<FunctionNode, String> {
    let name = match tokens[0].clone() {
        Token::Identifier(s) => s,
        _ => return "Function name not identifier!".to_err()
    };

    let body = tokens.iter()
        .enumerate()
        .filter(|(i, _)| (2..(tokens.len() - 1)).contains(i))
        .map(|(_, t)| t.to_owned())
        .collect::<Vec<Token>>();

    Ok(FunctionNode { args: parse_args(body)?, name })
}

fn parse_args(tokens: Vec<Token>) -> Result<Vec<FunctionArg>, String> {
    let mut bracket_err: isize = 0;
    let mut buffer = Vec::new();
    let mut args = Vec::new();

    for token in tokens.iter() {
        let token = token.clone();

        if token == Token::Open {
            bracket_err += 1;
        }
        
        if token == Token::Close {
            bracket_err -= 1;
        }
        
        if bracket_err == 0 && token == Token::ArgumentSeperator {
            args.push(buffer.clone());
            buffer.clear();
            continue;
        }

        buffer.push(token.clone());
    }
    args.push(buffer.clone());

    let args = args.iter()
        .map(|arg| {
            let first = arg[0].to_owned();

            // Literal
            if arg.len() <= 1 {
                if let Token::Identifier(i) = first {
                    return Ok(FunctionArg::Indentifier(i));
                }

                if let Token::Number(n) = first {
                    return Ok(FunctionArg::Number(n));
                }

                if let Token::String(s) = first {
                    return Ok(FunctionArg::String(s));
                }
            }

            // Function call
            if let Token::Identifier(_) = first {
                return Ok(FunctionArg::Block(vec![parse_function(arg.to_owned())?]));
            }

            // Block
            Ok(FunctionArg::Block(funcify({
                let mut iter = arg.iter();
                iter.next();
                iter.next_back();
                iter.cloned()
                    .collect()
            })?))
        })
        .collect::<Vec<Result<FunctionArg, String>>>();
    
    let mut args_out = Vec::new();
    for arg in args {
        args_out.push(arg?)
    }


    Ok(args_out)
}

fn split_functions(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut bracket_err: isize = 0;
    let mut buffer = Vec::new();
    let mut functions = Vec::new();

    for token in tokens.iter() {
        let token = token.clone();

        if token == Token::Open {
            bracket_err += 1;
        }
        
        if token == Token::Close {
            bracket_err -= 1;
        }
        
        if bracket_err == 0 && token == Token::Seperator {
            functions.push(buffer.clone());
            buffer.clear();
            continue;
        }
        
        buffer.push(token.clone());
    }

    functions
}

#[derive(Debug)]
pub struct FunctionNode {
    pub args: Vec<FunctionArg>,
    pub name: String,
}

#[derive(Debug)]
pub enum FunctionArg {
    String(String),
    Number(isize),
    Indentifier(String),
    Block(Vec<FunctionNode>),
}
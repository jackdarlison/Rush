use crate::architecture::{ast::{AstProgram, AstCompound, AstStatement, AstControlFlow, AstCommand}, shell_result::ShellResult, shell_error::ShellError};

use super::session::Session;


pub fn execute_program(input: AstProgram, session: &mut Session) -> Result<Vec<ShellResult>, ShellError> {
    match input {
        AstProgram::Program(c) => execute_compound(c, session, &mut vec![])
    }
}

pub fn execute_compound(input: Box<AstCompound>, session: &mut Session, results: &mut Vec<ShellResult>) -> Result<Vec<ShellResult>, ShellError> {
    match *input {
        AstCompound::And(compound, st) => {
            if let Err(e) = execute_compound(compound, session, results) {
                return Err(e);
            } else {
                if let Err(e) = execute_statement(st, session, results) {
                    return Err(e)
                }
            }
        },
        AstCompound::Or(compound, st) => {
            if let Err(_) = execute_compound(compound, session, results) {
                if let Err(e) = execute_statement(st, session, results) {
                    return Err(e)
                }
            } 
        },
        AstCompound::List(compound, st) => {
            execute_compound(compound, session, results);
            execute_statement(st, session, results);
            //TODO: handle if last is error
        },
        AstCompound::Statement(s) => {
            match execute_statement(s, session, results) {
                Ok(_r) => (),
                Err(e) => return Err(e),
            }
        },
    }
    Ok(results.to_vec())
}

pub fn execute_statement(input: AstStatement, session: &mut Session, results: &mut Vec<ShellResult>) -> Result<Vec<ShellResult>, ShellError> {
    match input {
        AstStatement::Command(c) => {
            match execute_command(c, session) {
                Ok(r) => results.push(r),
                Err(e) => return Err(e),
            };
        },
        AstStatement::ControlFlow(cf) => {
            match execute_control_flow(cf, session, results) {
                Ok(_rs) => (),
                Err(e) => return Err(e),
            }
        },
    }
    Ok(results.to_vec())
}

pub fn execute_control_flow(input: AstControlFlow, session: &mut Session, results: &mut Vec<ShellResult>) -> Result<Vec<ShellResult>, ShellError> {
    match input {
        AstControlFlow::For { var, range, body } => {
            for i in range {
                session.vars.insert(var.clone(), i.to_string());
                match execute_compound(body.clone(), session, results) {
                    Ok(_rs) => (),
                    Err(e) => return Err(e),
                }
            }
        },
        // AstControlFlow::If => todo!(),
        // AstControlFlow::While => todo!(),
        // AstControlFlow::Until => todo!(),
        // AstControlFlow::Switch => todo!(),
        _ => return Err(ShellError::UnknownError("Control flow type not yet implemented".to_string()))
    }
    Ok(results.to_vec())
}

pub fn execute_command(input: AstCommand, session: &mut Session) -> Result<ShellResult, ShellError> {
    let res = input.command.run(session, input.options, input.arguments);
    session.set_last_result(res.clone());
    res
}
use crate::architecture::{ast::{AstProgram, AstCompound, AstStatement, AstControlFlow, AstCommand}, shell_result::ShellResult, shell_error::ShellError};

use super::session::Session;


pub fn execute_program(input: AstProgram, session: &mut Session) -> Result<ShellResult, ShellError> {
    match input {
        AstProgram::Program(c) => execute_compound(c, session)
    }
}

pub fn execute_compound(input: Box<AstCompound>, session: &mut Session) -> Result<ShellResult, ShellError> {
    match *input {
        AstCompound::And(comp, st) => todo!(),
        AstCompound::Or(_, _) => todo!(),
        AstCompound::List(_, _) => todo!(),
        AstCompound::Statement(s) => execute_statement(s, session),
    }
}

pub fn execute_statement(input: AstStatement, session: &mut Session) -> Result<ShellResult, ShellError> {
    match input {
        AstStatement::Command(c) => execute_command(c, session),
        AstStatement::ControlFlow(cf) => execute_control_flow(cf, session),
    }
}

pub fn execute_control_flow(input: AstControlFlow, session: &mut Session) -> Result<ShellResult, ShellError> {
    match input {
        AstControlFlow::For { var, range, body } => {
            let mut res = Ok(ShellResult::None);
            for i in range {
                session.vars.insert(var.clone(), i.to_string());
                res = execute_compound(body.clone(), session);
            }
            return res
        },
        AstControlFlow::If => todo!(),
        AstControlFlow::While => todo!(),
        AstControlFlow::Until => todo!(),
        AstControlFlow::Switch => todo!(),
    }
}

pub fn execute_command(input: AstCommand, session: &mut Session) -> Result<ShellResult, ShellError> {
    input.command.run(session, input.options, input.arguments)
}
use std::fmt;
use std::fmt::{Display, Formatter};

use super::ioc_variable_command::IocVariableCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IocShellCommand {
    DbGetField(String),
    DbPutField(String, String),
    Exit,
}

impl From<IocVariableCommand> for IocShellCommand {
    fn from(command: IocVariableCommand) -> IocShellCommand {
        match command {
            IocVariableCommand::Get(name) => IocShellCommand::DbGetField(name),
            IocVariableCommand::Set(name, value) => {
                IocShellCommand::DbPutField(name, value)
            }
        }
    }
}

impl Display for IocShellCommand {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            IocShellCommand::DbGetField(ref name) => {
                write!(formatter, "dbgf {}", name)
            }
            IocShellCommand::DbPutField(ref name, ref value) => {
                write!(formatter, "dbpf {} {}", name, value)
            }
            IocShellCommand::Exit => write!(formatter, "exit"),
        }
    }
}

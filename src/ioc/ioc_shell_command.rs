use super::ioc_variable_command::IocVariableCommand;

#[derive(Clone, Eq, PartialEq)]
pub enum IocShellCommand {
    DbGetField(String),
    DbPutField(String, String),
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

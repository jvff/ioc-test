use super::super::ioc::IocShellCommand;

pub enum IocTestVariableAction {
    Set(String, String),
    Check(String, String),
}

impl IocTestVariableAction {
    pub fn ioc_shell_command(&self) -> IocShellCommand {
        match *self {
            IocTestVariableAction::Set(ref name, ref value) => {
                IocShellCommand::DbPutField(name.clone(), value.clone())
            }
            IocTestVariableAction::Check(ref name, _) => {
                IocShellCommand::DbGetField(name.clone())
            }
        }
    }
}

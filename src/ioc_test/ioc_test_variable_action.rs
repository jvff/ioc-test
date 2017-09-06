use super::super::ioc::{EpicsDataType, IocShellCommand};

#[derive(Clone)]
pub enum IocTestVariableAction {
    Set(String, EpicsDataType),
    Check(String, EpicsDataType),
}

impl IocTestVariableAction {
    pub fn ioc_shell_command(&self) -> IocShellCommand {
        self.clone().into()
    }

    pub fn expected_output(&self) -> String {
        let value = match *self {
            IocTestVariableAction::Set(_, ref value) => value,
            IocTestVariableAction::Check(_, ref value) => value,
        };

        format!("{}:          {}", value.type_name(), value)
    }

    pub fn expected_variable_value(&self) -> EpicsDataType {
        match *self {
            IocTestVariableAction::Set(_, ref value) => value.clone(),
            IocTestVariableAction::Check(_, ref value) => value.clone(),
        }
    }
}

impl Into<IocShellCommand> for IocTestVariableAction {
    fn into(self) -> IocShellCommand {
        match self {
            IocTestVariableAction::Set(name, value) => {
                IocShellCommand::DbPutField(name, value.to_string())
            }
            IocTestVariableAction::Check(name, _) => {
                IocShellCommand::DbGetField(name)
            }
        }
    }
}

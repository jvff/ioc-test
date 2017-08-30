use super::super::ioc::{EpicsDataType, IocShellCommand};

#[derive(Clone)]
pub enum IocTestVariableAction {
    Set(String, EpicsDataType),
    Check(String, EpicsDataType),
}

impl IocTestVariableAction {
    pub fn ioc_shell_command(&self) -> IocShellCommand {
        match *self {
            IocTestVariableAction::Set(ref name, ref value) => {
                IocShellCommand::DbPutField(name.clone(), value.to_string())
            }
            IocTestVariableAction::Check(ref name, _) => {
                IocShellCommand::DbGetField(name.clone())
            }
        }
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

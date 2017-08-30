use super::super::ioc::{EpicsDataType, IocShellCommand};

#[derive(Clone)]
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

    pub fn expected_output(&self) -> String {
        let value = match *self {
            IocTestVariableAction::Set(_, ref value) => value,
            IocTestVariableAction::Check(_, ref value) => value,
        };

        format!("DBR_STRING:          \"{}\"", value)
    }

    pub fn expected_variable_value(&self) -> EpicsDataType {
        let value = match *self {
            IocTestVariableAction::Set(_, ref value) => value,
            IocTestVariableAction::Check(_, ref value) => value,
        };

        EpicsDataType::DbrString(value.clone())
    }
}

mod frequency;
mod function;
mod phase;
mod voltage;

use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::request::ScpiRequest;
use super::super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScpiSourceSubsystem {
    source: usize,
    command: SourceCommand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SourceCommand {
    FrequencyGet,
    PhaseGet,
    VoltageGet,
    VoltageOffsetGet,
    FunctionQuery,
    ArbitraryFunctionFileQuery,
    ArbitraryFunctionSampleRateGet,
    NoiseFunctionBandwidthGet,
    PrbsFunctionBitRateGet,
    PrbsFunctionPolynomialGet,
    PrbsFunctionTransitionGet,
    PulseFunctionLeadingEdgeTransitionGet,
    PulseFunctionTrailingEdgeTransitionGet,
    PulseFunctionPulseWidthGet,
    RampFunctionSymmetryGet,
    SquareFunctionDutyCycleGet,
}

impl ScpiSourceSubsystem {
    pub fn new(source: usize, command: SourceCommand) -> Self {
        Self { source, command }
    }
}

impl Display for ScpiSourceSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let source = self.source;

        match self.command {
            SourceCommand::FrequencyGet => {
                write!(formatter, "SOUR{}:FREQ?", source)
            }
            SourceCommand::PhaseGet => {
                write!(formatter, "SOUR{}:PHAS?", source)
            }
            SourceCommand::VoltageGet => {
                write!(formatter, "SOUR{}:VOLT?", source)
            }
            SourceCommand::VoltageOffsetGet => {
                write!(formatter, "SOUR{}:VOLT:OFFSet?", source)
            }
            SourceCommand::FunctionQuery => {
                write!(formatter, "SOUR{}:FUNC?", source)
            }
            SourceCommand::ArbitraryFunctionFileQuery => {
                write!(formatter, "SOUR{}:FUNC:ARB?", source)
            }
            SourceCommand::ArbitraryFunctionSampleRateGet => {
                write!(formatter, "SOUR{}:FUNC:ARB:SRAT?", source)
            }
            SourceCommand::NoiseFunctionBandwidthGet => {
                write!(formatter, "SOUR{}:FUNC:NOIS:BAND?", source)
            }
            SourceCommand::PrbsFunctionBitRateGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:BRAT?", source)
            }
            SourceCommand::PrbsFunctionPolynomialGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:DATA?", source)
            }
            SourceCommand::PrbsFunctionTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:TRAN?", source)
            }
            SourceCommand::PulseFunctionLeadingEdgeTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:TRAN:LEAD?", source)
            }
            SourceCommand::PulseFunctionTrailingEdgeTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:TRAN:TRA?", source)
            }
            SourceCommand::PulseFunctionPulseWidthGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:WIDT?", source)
            }
            SourceCommand::RampFunctionSymmetryGet => {
                write!(formatter, "SOUR{}:FUNC:RAMP:SYMM?", source)
            }
            SourceCommand::SquareFunctionDutyCycleGet => {
                write!(formatter, "SOUR{}:FUNC:SQU:DCYC?", source)
            }
        }
    }
}

impl ScpiRequest for ScpiSourceSubsystem {
    fn decode(message: &str) -> Option<Self> {
        if message.view_first_chars(4) == "SOUR" {
            return decode_source_message(message);
        }

        None
    }
}

pub fn decode_source_message(string: &str) -> Option<ScpiSourceSubsystem> {
    let request_data = string.skip_expected_chars("SOURce");

    if let Some((source, command)) = request_data.parse_integer() {
        if command.starts_with(":") {
            let command = command.skip_chars(1);

            let decoded_command = match command.view_first_chars(4) {
                "FREQ" => frequency::decode(command),
                "FUNC" => function::decode(command),
                "PHAS" => phase::decode(command),
                "VOLT" => voltage::decode(command),
                _ => None,
            };

            if let Some(command) = decoded_command {
                return Some(ScpiSourceSubsystem { source, command });
            }
        }
    }

    None
}

pub type Subsystem = ScpiSourceSubsystem;

pub struct Builder {
    source: usize,
}

pub fn builder(source: usize) -> Builder {
    Builder { source }
}

impl Builder {
    pub fn get_frequency(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(self.source, SourceCommand::FrequencyGet)
    }

    pub fn get_phase(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(self.source, SourceCommand::PhaseGet)
    }

    pub fn get_voltage(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(self.source, SourceCommand::VoltageGet)
    }

    pub fn get_voltage_offset(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(self.source, SourceCommand::VoltageOffsetGet)
    }

    pub fn query_function(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(self.source, SourceCommand::FunctionQuery)
    }

    pub fn query_arbitrary_function_file(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::ArbitraryFunctionFileQuery,
        )
    }

    pub fn get_arbitrary_function_sample_rate(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::ArbitraryFunctionSampleRateGet,
        )
    }

    pub fn get_noise_function_bandwidth(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::NoiseFunctionBandwidthGet,
        )
    }

    pub fn get_prbs_function_bit_rate(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PrbsFunctionBitRateGet,
        )
    }

    pub fn get_prbs_function_polynomial(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PrbsFunctionPolynomialGet,
        )
    }

    pub fn get_prbs_function_transition(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PrbsFunctionTransitionGet,
        )
    }

    pub fn get_pulse_function_leading_edge(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PulseFunctionLeadingEdgeTransitionGet,
        )
    }

    pub fn get_pulse_function_trailing_edge(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PulseFunctionTrailingEdgeTransitionGet,
        )
    }

    pub fn get_pulse_function_pulse_width(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::PulseFunctionPulseWidthGet,
        )
    }

    pub fn get_ramp_function_symmetry(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::RampFunctionSymmetryGet,
        )
    }

    pub fn get_square_function_duty_cycle(self) -> ScpiSourceSubsystem {
        ScpiSourceSubsystem::new(
            self.source,
            SourceCommand::SquareFunctionDutyCycleGet,
        )
    }
}

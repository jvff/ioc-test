use std::fmt;
use std::fmt::{Display, Formatter};

use bytes::BytesMut;

use super::str_extensions::StrExtensions;
use super::super::errors::{ErrorKind, Result};
use super::super::extension::ScpiExtension;

use super::{output, source};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ScpiRequest<X>
where
    X: ScpiExtension,
{
    CalibrationQuery,
    OutputOn(usize),
    OutputOff(usize),
    OutputStatus(usize),
    SourceFrequencyGet(usize),
    SourcePhaseGet(usize),
    SourceVoltageGet(usize),
    SourceVoltageOffsetGet(usize),
    SourceFunctionQuery(usize),
    SourceArbitraryFunctionFileQuery(usize),
    SourceArbitraryFunctionSampleRateGet(usize),
    SourceNoiseFunctionBandwidthGet(usize),
    SourcePrbsFunctionBitRateGet(usize),
    SourcePrbsFunctionPolynomialGet(usize),
    SourcePrbsFunctionTransitionGet(usize),
    SourcePulseFunctionLeadingEdgeTransitionGet(usize),
    SourcePulseFunctionTrailingEdgeTransitionGet(usize),
    SourcePulseFunctionPulseWidthGet(usize),
    SourceRampFunctionSymmetryGet(usize),
    SourceSquareFunctionDutyCycleGet(usize),
    Other(X),
}

impl<X> ScpiRequest<X>
where
    X: ScpiExtension,
{
    pub fn from(string: &str) -> Result<Self> {
        let decoded_request = match string.view_first_chars(4) {
            "*CAL" if string == "*CAL?" => Some(ScpiRequest::CalibrationQuery),
            "OUTP" => output::decode(string),
            "SOUR" => source::decode(string),
            _ => None,
        };

        if let Some(request) = decoded_request {
            Ok(request)
        } else if let Some(extended_request) = X::decode(string) {
            Ok(ScpiRequest::Other(extended_request))
        } else {
            Err(ErrorKind::UnknownScpiRequest(String::from(string)).into())
        }
    }

    pub fn encode(&self, buffer: &mut BytesMut) {
        buffer.extend(self.to_string().as_bytes());
        buffer.extend("\n".as_bytes())
    }
}

impl<X> Display for ScpiRequest<X>
where
    X: ScpiExtension,
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiRequest::CalibrationQuery => write!(formatter, "*CAL?"),
            ScpiRequest::OutputOn(channel) => {
                write!(formatter, "OUTP{} ON", channel)
            }
            ScpiRequest::OutputOff(channel) => {
                write!(formatter, "OUTP{} OFF", channel)
            }
            ScpiRequest::OutputStatus(channel) => {
                write!(formatter, "OUTP{}?", channel)
            }
            ScpiRequest::SourceFrequencyGet(source) => {
                write!(formatter, "SOUR{}:FREQ?", source)
            }
            ScpiRequest::SourcePhaseGet(source) => {
                write!(formatter, "SOUR{}:PHAS?", source)
            }
            ScpiRequest::SourceVoltageGet(source) => {
                write!(formatter, "SOUR{}:VOLT?", source)
            }
            ScpiRequest::SourceVoltageOffsetGet(source) => {
                write!(formatter, "SOUR{}:VOLT:OFFSet?", source)
            }
            ScpiRequest::SourceFunctionQuery(source) => {
                write!(formatter, "SOUR{}:FUNC?", source)
            }
            ScpiRequest::SourceArbitraryFunctionFileQuery(source) => {
                write!(formatter, "SOUR{}:FUNC:ARB?", source)
            }
            ScpiRequest::SourceArbitraryFunctionSampleRateGet(source) => {
                write!(formatter, "SOUR{}:FUNC:ARB:SRAT?", source)
            }
            ScpiRequest::SourceNoiseFunctionBandwidthGet(source) => {
                write!(formatter, "SOUR{}:FUNC:NOIS:BAND?", source)
            }
            ScpiRequest::SourcePrbsFunctionBitRateGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:BRAT?", source)
            }
            ScpiRequest::SourcePrbsFunctionPolynomialGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:DATA?", source)
            }
            ScpiRequest::SourcePrbsFunctionTransitionGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:TRAN?", source)
            }
            ScpiRequest::SourcePulseFunctionLeadingEdgeTransitionGet(
                source,
            ) => write!(formatter, "SOUR{}:FUNC:PULS:TRAN:LEAD?", source),
            ScpiRequest::SourcePulseFunctionTrailingEdgeTransitionGet(
                source,
            ) => write!(formatter, "SOUR{}:FUNC:PULS:TRAN:TRA?", source),
            ScpiRequest::SourcePulseFunctionPulseWidthGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PULS:WIDT?", source)
            }
            ScpiRequest::SourceRampFunctionSymmetryGet(source) => {
                write!(formatter, "SOUR{}:FUNC:RAMP:SYMM?", source)
            }
            ScpiRequest::SourceSquareFunctionDutyCycleGet(source) => {
                write!(formatter, "SOUR{}:FUNC:SQU:DCYC?", source)
            }
            ScpiRequest::Other(ref request_extension) => {
                request_extension.fmt(formatter)
            }
        }
    }
}


use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;

use super::super::scpi;
use super::super::scpi::ScpiProtocol;
use super::super::scpi::ScpiRequest;
use super::super::scpi::ScpiResponse;

pub trait Protocol
    : ServerProto<
    TcpStream,
    Request = ScpiRequest,
    Response = ScpiResponse,
    Error = scpi::Error,
> {
}

impl Protocol for ScpiProtocol {}

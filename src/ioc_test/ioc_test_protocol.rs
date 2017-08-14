use std::fmt::Display;
use std::hash::Hash;
use std::io;

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;

use super::super::{mock_server, scpi};
use super::super::scpi::{ScpiProtocol, ScpiRequest, ScpiResponse};

pub trait IocTestProtocol {
    type Request: 'static + Clone + Display + Eq + Hash;
    type Response: 'static + Clone;
    type Error: From<io::Error> + Into<mock_server::Error>;
    type Protocol: ServerProto<
        TcpStream,
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;
}

impl IocTestProtocol for ScpiProtocol {
    type Request = ScpiRequest;
    type Response = ScpiResponse;
    type Error = scpi::Error;
    type Protocol = ScpiProtocol;
}

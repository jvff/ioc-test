#[macro_export]
macro_rules! scpi_subsystems {
    ( $( $name:ident ),+ $(,)*) => {
        mod custom_scpi_extension {
            use std::fmt;
            use std::fmt::{Display, Formatter};

            use ioc_test::scpi;

            #[allow(non_camel_case_types)]
            #[derive(Clone, Debug, Eq, Hash, PartialEq)]
            pub enum ScpiRequest {
                $( $name (scpi::subsystems::$name::Subsystem), )*
            }

            impl Display for ScpiRequest {
                fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
                    match *self {
                        $( ScpiRequest::$name(ref subsystem) => {
                            subsystem.fmt(formatter)
                        } )*
                    }
                }
            }

            impl scpi::ScpiRequest for ScpiRequest {
                fn decode(message: &str) -> Option<Self> {
                    $( if let Some(request) =
                       scpi::subsystems::$name::Subsystem::decode(message)
                    {
                        Some(ScpiRequest::$name(request))
                    } else )* {
                        None
                    }
                }
            }

            $( impl From<scpi::subsystems::$name::Subsystem> for ScpiRequest {
                fn from(subsystem: scpi::subsystems::$name::Subsystem) -> Self {
                    ScpiRequest::$name(subsystem)
                }
            })*

            pub type ScpiProtocol = scpi::ScpiProtocol<ScpiRequest>;
        }

        pub use self::custom_scpi_extension::ScpiRequest;
        pub use self::custom_scpi_extension::ScpiProtocol;
        pub use ioc_test::scpi::ScpiResponse;

        $( pub use ioc_test::scpi::subsystems::$name::builder as $name; )*
    }
}

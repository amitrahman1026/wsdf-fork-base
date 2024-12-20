use wsdf::{protocol, version, Dissect, Proto};

protocol!(BadProtocol);
version!("0.0.1", 4, 4);

#[derive(Proto, Dissect)]
#[wsdf(decode_from = 123)] // Error: decode_from must be string or tuple
struct BadProtocol {
    #[wsdf(typ = "INVALID_TYPE")] // Error: Invalid field type
    field1: u32,

    #[wsdf(display = "BAD_BASE")] // Error: Invalid display base
    field2: u16,

    good_subdissector: u16,
    #[wsdf(subdissector = ("udp.port", "bad_subdissector1", "good_subdissector", "bad_subdissector2"))]
    // Error: Some referenced fields don't exist
    payload: Vec<u8>,
}

fn main() {}

use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(transparent)]
pub struct ConstructorParams {
    pub address: Address,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(transparent)]
pub struct PubkeyAddressReturn {
    pub address: Address,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct AuthenticateMessageParams {
    #[serde(with = "strict_bytes")]
    pub signature: Vec<u8>,
    #[serde(with = "strict_bytes")]
    pub message: Vec<u8>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(transparent)]
pub struct AuthenticateMessageReturn {
    pub authenticated: bool,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXArguments {
    #[serde(with = "strict_bytes")]
    pub k: Vec<u8>,
    #[serde(with = "strict_bytes")]
    pub h: Vec<u8>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXResult {
    #[serde(with = "strict_bytes")]
    pub result: [u8; 32],
}

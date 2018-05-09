//! this module defines traits which represent INDY SDK API calls

use super::payment_address_config::PaymentAddressConfig;

/**
    This defines the interfaces which can be replaced with different implementations
    (aka production vs test time)

    modeling: master/libindy/src/api/crypto.rs
*/
pub trait CryptoAPI {
    fn indy_create_key(&self, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String;
}
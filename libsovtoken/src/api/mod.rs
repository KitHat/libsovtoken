//
// export public methods
//
//
//
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use std;
use libc::c_char;
use indy::api::ErrorCode;
use logic::payment_address_config::PaymentAddressConfig;
use logic::payments::{CreatePaymentSDK, CreatePaymentHandler};
use logic::output_mint_config::{OutputMintConfig, MintRequest};
use logic::request::Request;
use utils::ffi_support::{str_from_char_ptr, cstring_from_str, string_from_char_ptr};
use utils::json_conversion::JsonDeserialize;
use utils::general::ResultExtension;
use logic::fees_config::{SetFeesRequest, Fees};

/// # Description
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// from tokens-interface.md/CreatePaymentAddressCB
///
/// # Params
/// command_handle: command handle to map callback to context
/// config_str: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
/// cb: description
///
/// # Returns
/// on Success:  payment_address will have the format:
///              pay:sov:{32 byte public key}{4 digit check sum}
///
/// # Errors
/// description of errors
#[no_mangle]
pub extern "C" fn create_payment_address_handler(command_handle: i32,
                                                 wallet_handle: i32,
                                                 config_str: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) -> ErrorCode>) -> ErrorCode {
    // TODO:  missing wallet id

    if cb.is_none() {
        return ErrorCode::CommonInvalidParam3;
    }

    if config_str.is_null() {
        return ErrorCode::CommonInvalidParam2
    }

    let json_config_str: String = match string_from_char_ptr(config_str) {
        Some(s) => s,
        None => return ErrorCode::CommonInvalidParam2
    };

    let config: PaymentAddressConfig = match PaymentAddressConfig::from_json(&json_config_str) {
        Ok(c) => c,
        Err(_) => return ErrorCode::CommonInvalidStructure,
    };


    // to return both payment address and private key pair so that we can write the private
    // key into the ledger
    let handler = CreatePaymentHandler::new(CreatePaymentSDK {} );
    let payment_address = handler.create_payment_address(command_handle, wallet_handle, config);
    let payment_address_cstring = cstring_from_str(payment_address);
    let payment_address_ptr = payment_address_cstring.as_ptr();

    match cb {
        Some(f) => f(command_handle, ErrorCode::Success, payment_address_ptr),
        None => panic!("cb was null even after check"),
    };

    return ErrorCode::Success;
}

/// Description
/// call made to wallet to list payment addresses
///    * missing from Slava
///
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn list_payment_addresses_handler() -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/AddRequestFeesCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn add_request_fees_handler(command_handle: i32,
                                           wallet_handle: i32,
                                           submitter_did: *const c_char,
                                           req_json: *const c_char,
                                           inputs_json: *const c_char,
                                           outputs_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               req_with_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParseResponseWithFeesCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_response_with_fees_handler(command_handle: i32,
                                                   req_json: *const c_char,
                                                   cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    return ErrorCode::Success;
}


/// Description
///
///
/// from tokens-interface.md/BuildPaymentReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
pub extern "C" fn build_payment_req_handler(command_handle: i32,
                                            wallet_handle: i32,
                                            submitter_did: *const c_char,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_req_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParsePaymentResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
pub extern "C" fn parse_payment_response_handler(command_handle: i32,
                                                 resp_json: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {

    return ErrorCode::Success;
}


/// Description
///
///
/// from tokens-interface.md/BuildGetUTXORequestCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_utxo_request_handler(command_handle: i32,
                                                 payment_address: *const c_char,
                                                 cb: Option<extern fn(command_handle_: i32,
                                                                      err: ErrorCode,
                                                                      get_utxo_txn_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
///
/// from tokens-interface.md/ParseGetUTXOResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_get_utxo_response_handler(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: i32,
                                                                       err: ErrorCode,
                                                                       utxo_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/BuildSetTxnFeesReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_set_txn_fees_handler(command_handle: i32,
                                         wallet_handle: i32,
                                         submitter_did: *const c_char,
                                         fees_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32, err: ErrorCode, set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {

    let handle_result = |result: Result<*const c_char, ErrorCode>| {
        let result_error_code = result.and(Ok(ErrorCode::Success)).ok_or_err();
        if cb.is_some() {
            let json_pointer = result.unwrap_or(std::ptr::null());
            cb.unwrap()(command_handle, result_error_code, json_pointer);
        }
        return result_error_code;
    };

    if cb.is_some() == false {
        return ErrorCode::CommonInvalidParam3;
    }

    let fees_json_str : &str = match str_from_char_ptr(fees_json) {

        Some(s) => s,
        None => return handle_result(Err(ErrorCode::CommonInvalidParam2))
    };

    let fees_config: Fees = match Fees::from_json(fees_json_str) {
        Ok(c) => c,
        Err(_) => return handle_result(Err(ErrorCode::CommonInvalidStructure))
    };

    let fees_request = SetFeesRequest::from_fee_config(fees_config);
    let fees_request = fees_request.serialize_to_cstring().unwrap();

    return handle_result(Ok(fees_request.as_ptr()));
}

/// Description
///
///
/// from tokens-interface.md/BuildGetTxnFeesReqCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn build_get_fees_txn_handler(command_handle: i32,
                                             cb: Option<extern fn(command_handle_: i32, err: ErrorCode, get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    return ErrorCode::Success;
}

/// Description
///
///
/// from tokens-interface.md/ParseGetTxnFeesResponseCB
/// #Params
/// param1: description.
///
/// #Returns
/// description. example if json, etc...
///
/// #Errors
/// description of errors
#[no_mangle]
pub extern "C" fn parse_get_fees_txn_response_handler(command_handle: i32,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                fees_json: *const c_char) -> ErrorCode>)-> ErrorCode {
    return ErrorCode::Success;
}


/// Builds a Mint Request to mint tokens
#[no_mangle]
pub extern "C" fn build_mint_txn_handler(command_handle: i32, outputs_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char) -> ErrorCode>)-> ErrorCode {

    let handle_result = |result: Result<*const c_char, ErrorCode>| {
        let result_error_code = result.and(Ok(ErrorCode::Success)).ok_or_err();
        if cb.is_some() {
            let json_pointer = result.unwrap_or(std::ptr::null());
            cb.unwrap()(command_handle, result_error_code, json_pointer);
        }
        return result_error_code;
    };

    if cb.is_none() {
        return handle_result(Err(ErrorCode::CommonInvalidParam3));
    }

    let outputs_json_str = match str_from_char_ptr(outputs_json) {
        Some(s) => s,
        None => return handle_result(Err(ErrorCode::CommonInvalidParam2))
    };

    let outputs_config: OutputMintConfig = match OutputMintConfig::from_json(outputs_json_str) {
        Ok(c) => c,
        Err(_) => return handle_result(Err(ErrorCode::CommonInvalidStructure))
    };

    let mint_request = MintRequest::from_config(outputs_config);
    let mint_request = mint_request.serialize_to_cstring().unwrap();

    return handle_result(Ok(mint_request.as_ptr()));
}
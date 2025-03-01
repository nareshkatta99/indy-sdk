pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pairwise;
pub mod pool;
pub mod did;
pub mod wallet;
pub mod blob_storage;
pub mod non_secrets;
pub mod payments;
pub mod logger;
pub mod cache;

use libc::c_char;

use domain::IndyConfig;
use errors::prelude::*;

use utils::ctypes;

pub type IndyHandle = i32;

//pub type WalletHandle = i32;
#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct WalletHandle(pub i32);
pub const INVALID_WALLET_HANDLE : WalletHandle = WalletHandle(0);

pub type CallbackHandle = i32;
pub type PoolHandle = i32;
pub type CommandHandle = i32;
pub type StorageHandle = i32;
pub type SearchHandle = i32;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2 = 101,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3 = 102,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5 = 104,

    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6 = 105,

    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7 = 106,

    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8 = 107,

    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10 = 109,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11 = 110,

    // Caller passed invalid value as param 12 (null, invalid json and etc..)
    CommonInvalidParam12 = 111,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112,

    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    CommonInvalidStructure = 113,

    // IO Error
    CommonIOError = 114,

    // Caller passed invalid value as param 13 (null, invalid json and etc..)
    CommonInvalidParam13 = 115,

    // Caller passed invalid value as param 14 (null, invalid json and etc..)
    CommonInvalidParam14 = 116,

    // Caller passed invalid value as param 15 (null, invalid json and etc..)
    CommonInvalidParam15 = 117,

    // Caller passed invalid value as param 16 (null, invalid json and etc..)
    CommonInvalidParam16 = 118,

    // Caller passed invalid value as param 17 (null, invalid json and etc..)
    CommonInvalidParam17 = 119,

    // Caller passed invalid value as param 18 (null, invalid json and etc..)
    CommonInvalidParam18 = 120,

    // Caller passed invalid value as param 19 (null, invalid json and etc..)
    CommonInvalidParam19 = 121,

    // Caller passed invalid value as param 20 (null, invalid json and etc..)
    CommonInvalidParam20 = 122,

    // Caller passed invalid value as param 21 (null, invalid json and etc..)
    CommonInvalidParam21 = 123,

    // Caller passed invalid value as param 22 (null, invalid json and etc..)
    CommonInvalidParam22 = 124,

    // Caller passed invalid value as param 23 (null, invalid json and etc..)
    CommonInvalidParam23 = 125,

    // Caller passed invalid value as param 24 (null, invalid json and etc..)
    CommonInvalidParam24 = 126,

    // Caller passed invalid value as param 25 (null, invalid json and etc..)
    CommonInvalidParam25 = 127,

    // Caller passed invalid value as param 26 (null, invalid json and etc..)
    CommonInvalidParam26 = 128,

    // Caller passed invalid value as param 27 (null, invalid json and etc..)
    CommonInvalidParam27 = 129,

    // Wallet errors
    // Caller passed invalid wallet handle
    WalletInvalidHandle = 200,

    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 201,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError = 202,

    // Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    WalletNotFoundError = 204,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError = 205,

    // Trying to open wallet that was opened already
    WalletAlreadyOpenedError = 206,

    // Attempt to open encrypted wallet with invalid credentials
    WalletAccessFailed = 207,

    // Input provided to wallet operations is considered not valid
    WalletInputError = 208,

    // Decoding of wallet data during input/output failed
    WalletDecodingError = 209,

    // Storage error occurred during wallet operation
    WalletStorageError = 210,

    // Error during encryption-related operations
    WalletEncryptionError = 211,

    // Requested wallet item not found
    WalletItemNotFound = 212,

    // Returned if wallet's add_record operation is used with record name that already exists
    WalletItemAlreadyExists = 213,

    // Returned if provided wallet query is invalid
    WalletQueryError = 214,

    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300,

    // Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle = 301,

    // Pool ledger terminated
    PoolLedgerTerminated = 302,

    // No consensus during ledger operation
    LedgerNoConsensusError = 303,

    // Attempt to parse invalid transaction response
    LedgerInvalidTransaction = 304,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError = 305,

    // Attempt to create pool ledger config with name used for another existing pool
    PoolLedgerConfigAlreadyExistsError = 306,

    // Timeout for action
    PoolLedgerTimeout = 307,

    // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
    // Call pool.indy_set_protocol_version to set correct Protocol version.
    PoolIncompatibleProtocolVersion = 308,

    // Item not found on ledger.
    LedgerNotFound = 309,

    // Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFullError = 400,

    AnoncredsInvalidUserRevocId = 401,

    // Attempt to generate master secret with duplicated name
    AnoncredsMasterSecretDuplicateNameError = 404,

    AnoncredsProofRejected = 405,

    AnoncredsCredentialRevoked = 406,

    // Attempt to create credential definition with duplicated id
    AnoncredsCredDefAlreadyExistsError = 407,

    // Crypto errors
    // Unknown format of DID entity keys
    UnknownCryptoTypeError = 500,

    // Attempt to create duplicate did
    DidAlreadyExistsError = 600,

    // Unknown payment method was given
    PaymentUnknownMethodError = 700,

    //No method were scraped from inputs/outputs or more than one were scraped
    PaymentIncompatibleMethodsError = 701,

    // Insufficient funds on inputs
    PaymentInsufficientFundsError = 702,

    // No such source on a ledger
    PaymentSourceDoesNotExistError = 703,

    // Operation is not supported for payment method
    PaymentOperationNotSupportedError = 704,

    // Extra funds on inputs
    PaymentExtraFundsError = 705,

    // The transaction is not allowed to a requester
    TransactionNotAllowedError = 706,

}

/// Set libindy runtime configuration. Can be optionally called to change current params.
///
/// #Params
/// config: {
///     "crypto_thread_pool_size": Optional<int> - size of thread pool for the most expensive crypto operations. (4 by default)
///     "collect_backtrace": Optional<bool> - whether errors backtrace should be collected.
///         Capturing of backtrace can affect library performance.
///         NOTE: must be set before invocation of any other API functions.
/// }
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_set_runtime_config(config: *const c_char) -> ErrorCode {
    trace!("indy_set_runtime_config >>> config: {:?}", config);

    check_useful_json!(config, ErrorCode::CommonInvalidParam1, IndyConfig);

    ::commands::indy_set_runtime_config(config);

    let res = ErrorCode::Success;

    trace!("indy_set_runtime_config: <<< res: {:?}", res);

    res
}

/// Get details for last occurred error.
///
/// This function should be called in two places to handle both cases of error occurrence:
///     1) synchronous  - in the same application thread
///     2) asynchronous - inside of function callback
///
/// NOTE: Error is stored until the next one occurs in the same execution thread or until asynchronous callback finished.
///       Returning pointer has the same lifetime.
///
/// #Params
/// * `error_json_p` - Reference that will contain error details (if any error has occurred before)
///  in the format:
/// {
///     "backtrace": Optional<str> - error backtrace.
///         Collecting of backtrace can be enabled by:
///             1) setting environment variable `RUST_BACKTRACE=1`
///             2) calling `indy_set_runtime_config` API function with `collect_backtrace: true`
///     "message": str - human-readable error description
/// }
///
#[no_mangle]
pub extern fn indy_get_current_error(error_json_p: *mut *const c_char) {
    trace!("indy_get_current_error >>> error_json_p: {:?}", error_json_p);

    let error = get_current_error_c_json();
    unsafe { *error_json_p = error };

    trace!("indy_get_current_error: <<<");
}

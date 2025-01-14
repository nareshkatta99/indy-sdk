# Indy SDK configuration

Indy SDK contains several components, some of which can be configured. 
This configuration is useful when you need to define some specific customizations on your project. 
Like predefined config files, additional plugins, logging and else.

This document contains information on how Indy-SDK components can be configured.

* [Libindy](#libindy)
    * [Pool](#pool)
    * [Wallet](#wallet)
    * [Payment](#payment)
    * [Logging](#logging)
    * [Error Handling](#error-handling)
    * [Runtime](#runtime)
* [Indy-CLI](#indy-cLI)
    * [Options](#options)
    * [Config](#config)
    * [Execution mode](#execution-mode)

## Libindy

#### Pool

* Genesis Transactions - predefined transactions which will be used for a connection to Pool.
`indy_create_pool_ledger_config` function accepts a `config` parameter that looks like:
```
{
    "genesis_txn": string (optional), Path to a file containg genesis transactions.
        - If NULL, then a default one will be used (<pool_name>.txn). 
}
```
An example of genesis transactions can be found [here](../cli/docker_pool_transactions_genesis)

* Connection
`indy_open_pool_ledger` function opens the pool ledger and forms connections to the nodes defined in the genesis transactions file.
This function accepts a config that defines the behavior of the client-side connection:
```
{
    "timeout": int (optional) - specifies the maximum number of seconds to wait for pool response (ACK, REPLY).
    "extended_timeout": int (optional), an additional number of seconds to wait for REPLY in case ACK has been received.
    "preordered_nodes": array<string> -  (optional), names of nodes which will have priority during request sending.
        This can be useful if a user prefers querying specific nodes.
        Note: Nodes not specified will be placed randomly.
}
```

* Protocol Version - specifies the version of Indy Node which Libindy works with (There is a global property PROTOCOL_VERSION that used in every request to the pool).
```
1 - for Indy Node 1.3
2 - for Indy Node 1.4 and greater
```

* State Proof
There are some types of requests to Nodes in the Pool which support State Proof optimization in
Client-Node communication. Instead of sending requests to all nodes in the Pool, a client can send a request
to a single Node and expect a State Proof.

Libindy allows the building and sending of custom requests via a pluggable interface (like Payment).
`indy_register_transaction_parser_for_sp` API function allows registering of State Proof parsers for these custom requests.

#### Wallet

* Storage Type - libindy allows plugging different wallet implementations to handle storage layers. 
Libindy uses Sqlite as the default storage layer.
`indy_register_wallet_storage` function allows registering of custom wallet storage implementation, passing the set of callbacks.

* Wallet Configuration
```
{
  "id": string, Identifier of the wallet.
        Configured storage uses this identifier to lookup exact wallet data placement.
  "storage_type": string (optional), Type of the wallet storage. Defaults to 'default'.
                 'Default' storage type allows to store wallet data in the local file.
                 Custom storage types can be registered with indy_register_wallet_storage call.
  "storage_config": object (optional), Storage configuration json. Storage type defines set of supported keys.
                    Can be optional if storage supports default configuration.
                    Configuration for 'default' storage type:
  {
    "path": string (optional), Path to the directory with wallet files.
            Defaults to $HOME/.indy_client/wallet.
            Wallet will be stored in the file {path}/{id}/sqlite.db
  }
}
```

* Wallet Credentials
```
{
  "key": string, Key or passphrase used for wallet key derivation.
                 Look to key_derivation_method param for information about supported key derivation methods.
  "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                         Can be optional if storage supports default configuration.
                         Should be empty for 'default' storage type.
  "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                         ARGON2I_MOD - derive secured wallet master key (used by default)
                         ARGON2I_INT - derive secured wallet master key (less secure but faster)
                         RAW - raw wallet key master provided (skip derivation).
                               RAW keys can be generated with indy_generate_wallet_key call
}
```

#### Payment

Libindy provides a generic API for building payment-related transactions. 
These functions look at registered payment methods and call corresponding handlers.
Libindy doesn't provide default support of any payment method. There is `indy_register_payment_method` API function to register a payment method.
Any payment method (e.g., Sovrin tokens, Bitcoin, Visa, etc.) may be added to Libindy through plugins.

#### Logging
Libindy provides two options for Logger initialization:

* `indy_set_default_logger` function sets default logger implementation. 
Rust `env_logger` is used by default.  This is a simple logger which writes to stdout (can be configured via `RUST_LOG` environment variable).
More details about `env_logger` and its customization can be found [here](https://crates.io/crates/env_logger).

* `indy_set_logger` function registers custom logger implementation. 
Library user can provide a custom logger implementation by passing a set of handlers which will be called in correspondent cases.

#### Error Handling

Every Libindy API function returns an error code that indicates result status of function execution. 
Some errors occur in only one specific case but some are common for different functions.
Libindy provides `indy_get_current_error` function which allows getting of details for last occurred error in addition to an error code.

This function should be called in two places to handle both cases of error occurrence:
1) synchronous  - in the same application thread
2) asynchronous - inside of function callback

#### Runtime
`indy_set_runtime_config` function allows setting of Libindy runtime configuration. 
```
{
    "crypto_thread_pool_size": int (optional) - size of thread pool for the most expensive crypto operations like keys generation. (4 by default)
    "collect_backtrace": Optional<bool> - whether errors backtrace should be collected.
        Capturing of backtrace can affect library performance.
        NOTE: must be set before invocation of any other API functions.
}
```

## Indy-CLI
There is a Command Line Interface (CLI) built over Libindy which provides a set of commands to:
* Manage wallets
* Manage pool configurations
* Manage DIDs
* Send transactions to the ledger

#### Options
* -h and --help - Print usage. (usage: `indy-cli --help`)

* --logger-config - Init logger according to a config file. \
                    Indy Cli uses [log4rs](https://crates.io/crates/log4rs) logging framework \
                    Usage: `indy-cli --logger-config <path-to-config-file>` \
                    Example: `indy-cli --logger-config logger.yml` \
                    An example config file can be found [here](../cli/logger.yml) \
                    By default no logger initialized.
            
* --plugins - Load custom plugins in Libindy (e.g., wallet storage type, payment, etc.) \
              Usage: `indy-cli --plugins <lib-1-name>:<init-func-1-name>,...,<lib-n-name>:<init-func-n-name>)` \
              Example: `indy-cli --plugins libnullpay.so:nullpay_init`

* --config - Define config file for CLI initialization. \
             Usage: `indy-cli --config <path-to-config-json-file>` \
             Example: `indy-cli --config linux-sample-config.json`

#### Config
Indy-CLI supports initialization with a predefined config file. 
A config file can contain the following fields:
```
{
  "loggerConfig": string, - path to a logger config file (is equal to usage of "--logger-config" option)
  "plugins": string, - a list of plugins to load in Libindy (is equal to usage of "--plugins" option)
  "taaAcceptanceMechanism": string - transaction author agreement acceptance mechanism to be used when sending write transactions to the Ledger.
}
```
An example config file can be found [here](../cli/linux-sample-config.json)

#### Execution mode
Indy-CLI supports two execution modes:
* Interactive. In this mode CLI will read commands from terminal interactively (command by command).
* Batch. In this mode all commands will be read from file or pipe and executed in series. \
Usage: `indy-cli <path-to-file>` \
An example of a batch script:
    ```
    wallet create w1 key=k1
    wallet open w1 key=k1
    did new
    did list
    wallet close
    exit
    ```

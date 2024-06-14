# Soroban Phonebook

This phonebook sample code shows how to build a smart contract to store a simple data structure, and how to retrieve the data from the smart contract, using Stellar Soroban. The smart contract will allow you to store a contact with first name, last name and phone number. It also allows you to list all contacts.

## Prerequisites
Soroban smart contracts are written in Rust, so in order to run this code, a Rust toolchain must be installed. To follow the steps of building this project on your own system, these are the prerequisites:

- A Rust toolchain
- A code editor supporting Rust
- The Soroban CLI

The [Soroban documentation](https://developers.stellar.org/docs/smart-contracts/getting-started/setup) has a step-by-step guide to setting up your system developing and deploying Soroban smart contracts.

## Code
The smart contract has two public functions, one for creating a contact in the phonebook, and one for listing the contacts in the phonebook. 

### Configurations
The code is based on the default project created with the `soroban contract init <project name>` command. The **Cargo.toml** file only has one dependency by default, and that's all that is needed for this project as well.

```rust
[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

The Soroban SDK is used in the code with the following macros and types:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, Vec, String};
```

No further configuration or imports are needed for the phonebook smart contract.

### Define Structs
The phonebook smart contract has two functions - to create a contact and to list the contacts. In this simple project only the contact's first name, last name and phone number is stored, so the `Contact` struct is very simple, and looks like this:

```rust
#[derive(Clone)]
#[contracttype]
pub struct Contact {
    pub phone_number: u32,
    pub first_name: String,
    pub last_name: String,
}
```

A second struct for listing the contacts in the phonebook, `Contacts`, is also added:

```rust
#[derive(Clone)]
#[contracttype]
pub struct Contacts {
    pub contacts: Vec<Contact>,
}
```

Finally the two structs are enumerated as `DataKey`.

```rust
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contact,
    Contacts,
}
```

### Define Contract
The smart contract is defined by the `#[contract]` attribute, and an implementation of smart contract functions can be associated with this contract. The implemented functions can be invoked from a frontend, from another smart contract or commandline by making them public. For the phonebook smart contract there are two public functions - `create` and `list`.

```rust
#[contract]
pub struct PhoneBookContract;

#[contractimpl]
impl PhoneBookContract {
    // pub fn create

    // pub fn list
}
```

### `create` function

sdf









## Build & Deploy




## Usage













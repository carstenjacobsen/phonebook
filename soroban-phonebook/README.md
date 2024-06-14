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

The two public functions are calling the private functions `create_contact` and `list_contacts`, which are doing the actual work of storing and retrieving the contact data.

### Function `create_contact`
The `create_contact` function takes two arguments, the **env** argument is giving the function access to the environment features like storing and retrieving data, and **contact** argument takes data in the `Contact` format. 

```rust
fn create_contact(env: &Env, contact: &Contact) {
    if env.storage().instance().has(&DataKey::Contacts) {
        let current_contacts: Contacts = env.storage().instance().get(&DataKey::Contacts).unwrap();
        let mut contacts_vec: Vec<Contact> = current_contacts.contacts;
        
        contacts_vec.push_back(contact.clone());

        let contacts = Contacts {
            contacts: contacts_vec.clone(),
        };

        env.storage().instance().set(&DataKey::Contacts, &contacts);
    } else {
        let contacts = Contacts {
            contacts: vec![&env, contact.clone()],
        };

        env.storage().instance().set(&DataKey::Contacts, &contacts);
    }    
}
```

The function first checks if there's already contacts stored, and append the new contact to the vector, if that's the case. If no contacts are stored, the new contact is stored as the first contact.

### Function `list_contacts`
The `list_contacts` function only takes one argument - the Soroban environment. The function returns a vector of `Contact` elements. 

```rust
fn list_contacts(env: &Env) -> Contacts {
    if env.storage().instance().has(&DataKey::Contacts) {
        env.storage().instance().get(&DataKey::Contacts).unwrap()
    } else {
        Contacts {
            contacts: vec![&env],
        }
    }
}
```

If `get()` is called and there's no contacts stored, the code will panic. So in order to avoid a panic, the function will first check if there are contacts stored before trying to get contacts. If the there's no contacts stored, and empty vector is returned.

### Complete Code
The only task left is to implement calls to the two functions in the public smart contract main code.

```rust
#[contractimpl]
impl PhoneBookContract {
    pub fn create(
        env: Env,
        phone_number: u32,
        first_name: String,
        last_name: String,
    ) {
        create_contact(
            &env,
            &Contact {
                phone_number,
                first_name,
                last_name,
            },
        )
    }

    pub fn list(env: Env) -> Contacts {
        list_contacts(&env)
    }
}
```

## Build & Deploy
The Rust code is not directly executable on-chain, it has to be compiled into WebAssembly binary code first. The binary code can be deployed on the blockchain and then invoked. 

### Build
The Rust code is compiled with the Soroban CLI `build` command. The output of the build command is a WASM-file, whch is stored in the `target` directory.

```bash
% soroban contract build

cargo rustc --manifest-path=contracts/phonebook/Cargo.toml --crate-type=cdylib --target=wasm32-unknown-unknown --release
   Compiling phonebook v0.0.0 (/Users/carstenjacobsen/Documents/Stellar Soroban/soroban-phonebook/contracts/phonebook)
    Finished release [optimized] target(s) in 1.66s
```

### Deploy
Deployment to testnet or mainnet requires an identity to be setup for signing transactions. If you don't already have a Soroban identity, a new identity can be setup with the Soroban CLI:

```bash
% soroban keys generate --global myname --network testnet
```

See the public key with this command:

```bash
% soroban keys address myname

GAP2QMTJOYNAMSTKNLNCAJ3PSAAYYXLLDR6QXUYQJMYO7UOUBZV3LYDL
```

With the compiled WASM-file, and the identity, the smart contract can now be deployed. The smart contract can be deployed to the testnet with the Soroban CLI. The deployment will return the smart contract ID.

```bash
% soroban contract deploy \   
  --wasm target/wasm32-unknown-unknown/release/phonebook.wasm \
  --source myname \
  --network testnet

CA5O7GWG6DOIKLK72YX6JEYW63M3EYSCUSKJUZMGPTGJU23R6BXXXXXX  
```

## Usage
The two public smart contract functions `create` and `list` can be tested by invoking them with Soroban CLI commands now the smart contract has been deployed to the testnet.

### Create Contact
First the `create` function is used to add a couple of contacts to the phonebook. Just change the phone number, first name and last name every time the function is invoked.

```bash
% soroban contract invoke \   
  --id CA5O7GWG6DOIKLK72YX6JEYW63M3EYSCUSKJUZMGPTGJU23R6BXXXXXX \
  --source myname \
  --network testnet \
  -- \
  create \   
  --phone_number 1231231234 \
  --first_name Bob \
  --last_name Smith
```

### List Contacts
The `list` function will return the contacts created in the previous step. 

```bash
% soroban contract invoke \
  --id CA5O7GWG6DOIKLK72YX6JEYW63M3EYSCUSKJUZMGPTGJU23R6BXXXXXX \
  --source myname \
  --network testnet \
  -- \
  list 

{"contacts":[{"first_name":"Bob","last_name":"Smith","phone_number":1231231234},{"first_name":"Alice","last_name":"Johnson","phone_number":3213214321}]}
```






## Usage













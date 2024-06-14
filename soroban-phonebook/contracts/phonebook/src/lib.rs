#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, Vec, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contact,
    Contacts,
}

#[derive(Clone)]
#[contracttype]
pub struct Contact {
    pub phone_number: u32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone)]
#[contracttype]
pub struct Contacts {
    pub contacts: Vec<Contact>,
}

#[contract]
pub struct PhoneBookContract;

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

fn list_contacts(env: &Env) -> Contacts {
    if env.storage().instance().has(&DataKey::Contacts) {
        env.storage().instance().get(&DataKey::Contacts).unwrap()
    } else {
        Contacts {
            contacts: vec![&env],
        }
    }
}

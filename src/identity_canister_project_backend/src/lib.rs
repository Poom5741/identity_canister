use ic_cdk_macros::{init, update, query};
use candid::{CandidType, Deserialize}; // Import from candid crate
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Identity {
    name: String,
    email: String,
    did: String,
}

type IdentityStore = HashMap<String, Identity>;

static mut IDENTITIES: Option<IdentityStore> = None;

#[init]
fn init() {
    unsafe {
        IDENTITIES = Some(HashMap::new());
    }
}

#[update]
fn register_identity(id: String, name: String, email: String) -> bool {
    unsafe {
        let identities = IDENTITIES.as_mut().unwrap();
        if identities.contains_key(&id) {
            return false;
        }

        let identity = Identity {
            name,
            email,
            did: format!("did:icp:{}", id),
        };

        identities.insert(id, identity);
        true
    }
}

#[query]
fn get_identity(id: String) -> Option<Identity> {
    unsafe {
        let identities = IDENTITIES.as_ref().unwrap();
        identities.get(&id).cloned()
    }
}

#[update]
fn update_identity(id: String, name: String, email: String) -> bool {
    unsafe {
        let identities = IDENTITIES.as_mut().unwrap();
        if !identities.contains_key(&id) {
            return false;
        }

        let identity = Identity {
            name,
            email,
            did: format!("did:icp:{}", id),
        };

        identities.insert(id, identity);
        true
    }
}

#[update]
fn delete_identity(id: String) -> bool {
    unsafe {
        let identities = IDENTITIES.as_mut().unwrap();
        if !identities.contains_key(&id) {
            return false;
        }

        identities.remove(&id);
        true
    }
}
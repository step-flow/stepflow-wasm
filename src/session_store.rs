use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use std::error::Error;
use once_cell::sync::OnceCell;
use stepflow::object::ObjectStore;
use stepflow::{Session, SessionId};


static SESSIONS: OnceCell<RwLock<ObjectStore<Session, SessionId>>> = OnceCell::new();

fn get_store_rwlock() -> &'static RwLock<ObjectStore<Session, stepflow::SessionId>> {
    SESSIONS.get_or_init(|| RwLock::new(ObjectStore::with_capacity(1)))
}

pub fn get_session_store() -> Result<RwLockReadGuard<'static, ObjectStore<Session, stepflow::SessionId>>, Box<dyn Error>> {
    get_store_rwlock()
        .read()
        .map_err(|e| Box::new(e).into())
}

pub fn get_session_store_mut() -> Result<RwLockWriteGuard<'static, ObjectStore<Session, stepflow::SessionId>>, Box<dyn Error>> {
    get_store_rwlock()
        .write()
        .map_err(|e| Box::new(e).into())
}

pub fn new_session() -> Result<SessionId, Box<dyn Error>> {
    let mut store_mut = get_session_store_mut()?;
    store_mut
        .insert_new(|id| Ok(Session::new(id)))
        .map_err(|e| Box::new(stepflow::Error::SessionId(e)).into())
}

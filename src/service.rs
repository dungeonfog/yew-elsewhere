use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use yew::prelude::*;

use once_cell::unsync::Lazy;

/// Service used for coordinating elsewhere stuff.
/// Using this requires you to clean up after yourself: An elsewhere component
/// must call `unregister_component` at the end of its lifetime.
pub struct ElsewhereService {
    // Global register for components.
    callbacks: RwLock<HashMap<String, Callback<Html>>>,
    // Calls that came in before the component registered.
    cached_calls: RwLock<HashMap<String, Html>>,
}

thread_local! {
    static SERVICE: Lazy<Arc<ElsewhereService>> = Lazy::new(|| Arc::new(ElsewhereService::init()));
}

impl ElsewhereService {
    fn init() -> Self {
        ElsewhereService {
            callbacks: RwLock::new(HashMap::new()),
            cached_calls: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a one-time use reference to the service singleton.
    pub fn get() -> Arc<ElsewhereService> {
        SERVICE.with(|service| Arc::clone(service))
    }

    /// Registers a new component with the name `name`.
    /// Returns Some(html) if there was already a request sent to that name.
    ///
    /// You will only need to use this method if you implement your own component!
    pub fn register_component(
        self: Arc<Self>,
        name: &str,
        callback: Callback<Html>,
    ) -> Option<Html> {
        if let Ok(mut callbacks) = self.callbacks.write() {
            if callbacks.contains_key(name) {
                log::warn!("Tried to register elsewhere component twice: {:?}", name);
            } else {
                callbacks.insert(name.to_string(), callback);
                if let Some(html) = self
                    .cached_calls
                    .write()
                    .ok()
                    .and_then(|mut cache| cache.remove(name))
                {
                    return Some(html);
                }
            }
        } else {
            log::warn!("Tried to write lock on a read locked RwLock");
        }
        None
    }

    /// Unregisters a currently registered component.
    ///
    /// You will only need to use this method if you implement your own component!
    pub fn unregister_component(self: Arc<Self>, name: &str) {
        if let Ok(mut callbacks) = self.callbacks.write() {
            if callbacks.remove(name).is_none() {
                log::warn!("Tried to remove non-registered callback: {:?}", name);
            }
        } else {
            log::warn!("Tried to write lock on a read locked RwLock");
        }
    }

    /// Sends new Html content to the elsewhere component with the given name.
    pub fn send(self: Arc<Self>, target: &str, html: Html) {
        if let Ok(callbacks) = self.callbacks.read() {
            if let Some(callback) = callbacks.get(target) {
                // Alright, we have our callback, that's it.
                callback.emit(html);
            } else {
                // We won't need this read lock anymore...
                drop(callbacks);
                // We couldn't find a callback.
                // Maybe the component hasn't registered yet.
                // Store the data until it gets registered.
                if let Ok(mut cache) = self.cached_calls.write() {
                    if cache.insert(target.to_string(), html).is_some() {
                        log::warn!("Possible error: Failed to send update to Elsewhere {:?} more than once", target);
                    }
                } else {
                    log::warn!(
                        "Failed to acquire write lock on cache: Lost update to Elsewhere {:?}",
                        target
                    );
                }
            }
        } else {
            log::warn!(
                "Failed to get read access to callbacks. Updating Elsewhere {:?} failed.",
                target
            );
        }
    }
}

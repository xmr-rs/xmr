// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::sync::Arc;

use command::{Id, Command};
use net::handlers::{NotificationHandler, InvokationHandler, RemoteHandler};

/// A reference to an `IoHandler`.
pub type IoHandlerRef = Arc<IoHandler>;

/// Handles external IO.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct IoHandler {
    handlers: HashMap<Id, RemoteHandler>,
}

impl IoHandler {
    /// Creates an empty `IoHandler`.
    pub fn new() -> IoHandler {
        IoHandler { handlers: HashMap::new() }
    }

    /// Creates an `IoHandler` with the given capacity.
    pub fn with_capacity(cap: usize) -> IoHandler {
        IoHandler { handlers: HashMap::with_capacity(cap) }
    }

    /// Add a notification to this handler.
    pub fn add_notification<C, F>(&mut self, handler: F)
        where C: Command,
              F: NotificationHandler + 'static
    {
        let result = self.handlers
            .insert(C::ID, RemoteHandler::Notification(Arc::new(handler)));
        if result.is_some() {
            warn!("Command #{} was previosly added.", C::ID);
        }
        trace!("Adding notification #{}", C::ID);
    }

    /// Add a notification to this handler.
    pub fn add_invokation<C, F>(&mut self, handler: F)
        where C: Command,
              F: InvokationHandler + 'static
    {
        let result = self.handlers
            .insert(C::ID, RemoteHandler::Invokation(Arc::new(handler)));
        if result.is_some() {
            warn!("Command #{} was previosly added.", C::ID);
        }
        trace!("Adding invokation #{}", C::ID);
    }

    /// Get a handler.
    pub(crate) fn get(&self, id: Id) -> Option<RemoteHandler> {
        self.handlers.get(&id).cloned()
    }

    /// Converts this `IoHandler` to an `IoHandlerRef`
    pub fn to_ref(self) -> IoHandlerRef {
        Arc::new(self)
    }
}

// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;
use std::net::SocketAddr;

use portable_storage::Section;

/// A trait that handles notifications.
pub trait NotificationHandler: Send + Sync + 'static {
    /// This is the function that handles the notification.
    fn call(&self, addr: SocketAddr, request: Section);
}

impl<F> NotificationHandler for F
    where F: Send + Sync + 'static + Fn(SocketAddr, Section)
{
    fn call(&self, addr: SocketAddr, request: Section) {
        self(addr, request)
    }
}

/// A trait that handles invokations.
pub trait InvokationHandler: Send + Sync + 'static {
    /// This handles the invokation.
    fn call(&self, addr: SocketAddr, request: Section) -> Result<Option<Section>, i32>;
}

impl<F> InvokationHandler for F
    where F: Send + Sync + 'static + Fn(SocketAddr, Section) -> Result<Option<Section>, i32>
{
    fn call(&self, addr: SocketAddr, request: Section) -> Result<Option<Section>, i32> {
        self(addr, request)
    }
}

/// A handler for a invokation/notification.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub enum RemoteHandler {
    /// A notification.
    Notification(Arc<NotificationHandler>),

    /// An invokation.
    Invokation(Arc<InvokationHandler>),
}

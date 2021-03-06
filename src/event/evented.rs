use crate::{Interests, Registry, Token};
use std::io;
use std::ops::Deref;

/// A value that may be registered with [`Registry`].
///
/// Handles that implement `Evented` can be registered with `Registry`. Users of
/// Mio **should not** use the `Evented` trait functions directly. Instead, the
/// equivalent functions on `Registry` should be used.
///
/// See [`Registry`] for more details.
///
/// [`Registry`]: crate::Registry
///
/// # Implementing `Evented`
///
/// `Evented` values are always backed by **system** handles, which are backed
/// by sockets or other system handles. These `Evented` handles will be
/// monitored by the system selector. An implementation of `Evented` will almost
/// always delegates to a lower level handle. Examples of this are
/// [`TcpStream`]s, or the *unix only* [`EventedFd`].
///
/// [`TcpStream`]: crate::net::TcpStream
/// [`EventedFd`]: crate::unix::EventedFd
///
/// # Dropping `Evented` types
///
/// All `Evented` types, unless otherwise specified, need to be [deregistered]
/// before being dropped for them to not leak resources. This goes against the
/// normal drop behaviour of types in Rust which cleanup after themselves, e.g.
/// a `File` will close itself. However since deregistering needs access to
/// [`Registry`] this cannot be done while being dropped.
///
/// [deregistered]: crate::Registry::deregister
///
/// # Examples
///
/// Implementing `Evented` on a struct containing a socket:
///
/// ```
/// use mio::{Interests, Registry, Token};
/// use mio::event::Evented;
/// use mio::net::TcpStream;
///
/// use std::io;
///
/// # #[allow(dead_code)]
/// pub struct MyEvented {
///     socket: TcpStream,
/// }
///
/// impl Evented for MyEvented {
///     fn register(&self, registry: &Registry, token: Token, interests: Interests)
///         -> io::Result<()>
///     {
///         // Delegate the `register` call to `socket`
///         self.socket.register(registry, token, interests)
///     }
///
///     fn reregister(&self, registry: &Registry, token: Token, interests: Interests)
///         -> io::Result<()>
///     {
///         // Delegate the `reregister` call to `socket`
///         self.socket.reregister(registry, token, interests)
///     }
///
///     fn deregister(&self, registry: &Registry) -> io::Result<()> {
///         // Delegate the `deregister` call to `socket`
///         self.socket.deregister(registry)
///     }
/// }
/// ```
pub trait Evented {
    /// Register `self` with the given `Registry` instance.
    ///
    /// This function should not be called directly. Use [`Registry::register`]
    /// instead. Implementors should handle registration by delegating the call
    /// to another `Evented` type.
    ///
    /// [`Registry::register`]: crate::Registry::register
    fn register(&self, registry: &Registry, token: Token, interests: Interests) -> io::Result<()>;

    /// Re-register `self` with the given `Registry` instance.
    ///
    /// This function should not be called directly. Use
    /// [`Registry::reregister`] instead. Implementors should handle
    /// re-registration by either delegating the call to another `Evented` type.
    ///
    /// [`Registry::reregister`]: crate::Registry::reregister
    fn reregister(&self, registry: &Registry, token: Token, interests: Interests)
        -> io::Result<()>;

    /// Deregister `self` from the given `Registry` instance.
    ///
    /// This function should not be called directly. Use
    /// [`Registry::deregister`] instead. Implementors should handle
    /// deregistration by delegating the call to another `Evented` type.
    ///
    /// [`Registry::deregister`]: crate::Registry::deregister
    fn deregister(&self, registry: &Registry) -> io::Result<()>;
}

impl<T, E> Evented for T
where
    T: Deref<Target = E>,
    E: Evented + ?Sized,
{
    fn register(&self, registry: &Registry, token: Token, interests: Interests) -> io::Result<()> {
        self.deref().register(registry, token, interests)
    }

    fn reregister(
        &self,
        registry: &Registry,
        token: Token,
        interests: Interests,
    ) -> io::Result<()> {
        self.deref().reregister(registry, token, interests)
    }

    fn deregister(&self, registry: &Registry) -> io::Result<()> {
        self.deref().deregister(registry)
    }
}

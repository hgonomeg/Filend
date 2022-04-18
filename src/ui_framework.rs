use glib::Sender;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

/// Wraps a custom-managed object with Glib/GTK objects
/// in a way that makes it convenient to work with multiple references to them
pub struct Handle<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Handle<T> {
    /// create a non-owning reference to the handle
    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle {
            inner: Rc::downgrade(&self.inner),
        }
    }
}

/// Non-owning version of Handle
pub struct WeakHandle<T> {
    inner: Weak<RefCell<T>>,
}

impl<T> WeakHandle<T> {
    pub fn upgrade(&self) -> Option<Handle<T>> {
        self.inner.upgrade().map(|x| Handle { inner: x })
    }
    /// creates an empty reference
    pub fn new() -> WeakHandle<T> {
        WeakHandle { inner: Weak::new() }
    }
}

impl<T> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        WeakHandle {
            inner: Weak::clone(&self.inner),
        }
    }
}

pub trait IntoHandle: Sized {
    fn into_handle(self) -> Handle<Self>;
}

impl<T> IntoHandle for T {
    fn into_handle(self) -> Handle<Self> {
        Handle {
            inner: Rc::from(RefCell::from(self)),
        }
    }
}
impl<T> Deref for Handle<T> {
    type Target = Rc<RefCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            inner: Rc::clone(&self.inner),
        }
    }
}

/// Provides an interface for custom-managed object with Glib/GTK objects such as a window
/// that can react to events and can be interacted declaratively via message-passing
/// so as to make it easy to work with GTK/Glib objects from other threads.
///
/// Inspired by Relm and Actix.
pub trait MessageHandler<M>: Sized + 'static {
    /// Returns a sender which can be used to send messages of a given type to the object.
    ///
    /// The corresponding receiver will handle the messages using the `handle()` method.
    fn create_sender(source: &Handle<Self>) -> Sender<M> {
        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());
        let handle = Handle::clone(source);
        rx.attach(None, move |msg|{
            let mut ref_mut = handle.try_borrow_mut().expect("Failed to mutably borrow object implementing MessageHandler<M> (such as a Handle) for the sake of processing incoming message");
            ref_mut.handle(handle.clone(),msg);
            glib::Continue(true)
        });
        tx
    }

    /// The proper message-handler.
    ///
    /// The passed handle object should only be used to pass it to elsewhere, e.g.
    /// into closures connected to GUI events.
    /// Trying to derefence the handle will panic.
    fn handle(&mut self, handle: Handle<Self>, msg: M);
}

/// Convenience trait for Handle<T>, providing Handle<T>::send(M) for T implementing MessageHandler
pub trait MessageHandlerProxy<M>: Sized + 'static {
    /// Convenience method that creates a glib::Sender<M> for a single use and uses it to send the given message.
    fn send(&self, msg: M);
}
impl<T, M> MessageHandlerProxy<M> for Handle<T>
where
    T: MessageHandler<M>,
{
    fn send(&self, msg: M) {
        let tx = T::create_sender(self);
        let _ = tx.send(msg);
    }
}

impl<T, M> MessageHandlerProxy<M> for WeakHandle<T>
where
    T: MessageHandler<M>,
{
    fn send(&self, msg: M) {
        let handle = self
            .upgrade()
            .expect("Handle<T> to which a WeakHandle<T> refers, no longer exists.");
        let tx = T::create_sender(&handle);
        let _ = tx.send(msg);
    }
}
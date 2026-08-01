use crate::Listener;
use accessory::Accessors;
use derive_more::{Deref, DerefMut};
use fancy_constructor::new;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::fmt;
use web_sys::{Event, EventTarget};

pub(crate) type TupleVec<'a> = SmallVec<[ListenTuple<'a>; 1]>;

#[derive(new, Accessors, Debug)]
pub(crate) struct ListenTuple<'a> {
    #[access(get(ty(&str)))]
    event_name: Cow<'a, str>,

    #[access(get(ty(&EventTarget)))]
    target: Cow<'a, EventTarget>,
}

/// A listener that's listening to `1..` events on `1..` targets.
///
/// It will automatically remove the event listener off the targets when dropped.
#[derive(new, Deref, DerefMut)]
#[new(vis(pub(crate)))]
pub struct AttachedListener<'a, T = Event> {
    #[deref]
    #[deref_mut]
    listener: Listener<T>,
    tuples: TupleVec<'a>,
}

impl<T> fmt::Debug for AttachedListener<'_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AttachedListener")
            .field("listener", &self.listener)
            .field("tuples", &self.tuples)
            .finish()
    }
}

impl<'a, T> Drop for AttachedListener<'a, T> {
    fn drop(&mut self) {
        self.listener.rm_multi(self.tuples.iter());
    }
}

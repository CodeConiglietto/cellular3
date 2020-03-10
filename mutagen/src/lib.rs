//! A small crate with big macros to make all the tedious bits of generation and mutation less cumbersome.
//!
//! # Generatable
//!
//! When derived on a struct, it will construct it by recursively generating its fields.
//!
//! When derived on an enum, it will choose a variant at random and recursively generate its fields.
//!
//! # Mutatable
//!
//! When derived on a struct, it will pick a field at random
//!
//! When derived on an enum, it requires [Generatable] to also be implemented for all fields, unless mut_reroll is 0.
//! It will then choose whether to re-roll a new variant with probability mut_reroll, or to mutate its current variant.
//!
//! # Attributes
//!
//! This crate makes extensive use of key-value pairs in attributes to customize the behaviour of its derive macros.
//! Key-value pairs are always contained inside a #[mutagen()], as shown in the example below.
//! Both floating point literals and function names are allowed as values.
//! When a function name is used, its signature should be `fn(&mutagen::State) -> f64`
//!
//! ```rust
//! use mutagen::{Generatable, Mutatable};
//!
//! #[derive(Generatable, Mutatable)]
//! #[mutagen(mut_reroll = 0.78)]
//! enum Foo {
//!   // Bar is 10 times as likely as Baz or Bax,
//!   // but it always rerolls into a different one when mutating
//!   #[mutagen(gen_weight = 10.0, mut_reroll = 1.0)]
//!   Bar,
//!
//!   // Baz never changes into a different variant when mutating
//!   #[mutagen(mut_reroll = 0.0)]
//!   Baz(Baz),
//!
//!   // All other variants have reroll probability of 0.78, as specified on Foo
//!   Bax {
//!      // a mutates twice as often as b
//!      #[mutagen(mut_weight = 0.5)]
//!      a: Baz,
//!
//!      b: Baz,
//!
//!      // c mutates only if it's at depth 2 or deeper
//!      #[mutagen(mut_weight = 1.0)]
//!      c: Baz,
//!   },
//!
//!   // This variant will never generate, so its fields don't need to implement Generatable
//!   #[mutagen(gen_weight = 0.0)]
//!   Boo(NotGeneratable),
//! }
//!
//! fn depth_at_least_2(state: &mutagen::State) -> f64 {
//!   if state.depth >= 2 {
//!     1.0
//!   } else {
//!     0.0
//!   }
//! }
//!
//! #[derive(Mutatable)]
//! struct Boz {
//!   // frob will never mutate, so it doesn't need to implement Mutatable
//!   #[mutagen(mut_weight = 0.0)]
//!   not_mutatable: NotMutatable,
//!
//!   mutatable: Baz,
//! }
//!
//! #[derive(Mutatable)]
//! struct NotGeneratable;
//!
//! #[derive(Generatable)]
//! struct NotMutatable;
//!
//! #[derive(Generatable, Mutatable)]
//! struct Baz;
//! ```
//!
//! **`#[mutagen(gen_weight = 1.0)]`**
//!
//! When applied to an enum variant, it affects how often that variant is generated.
//! By default, all variants have weight 1.
//!
//! Note that when an enum variant has a weight of 0, it will never be generated, so the derived impl
//! will not expect its fields to implement Generatable.
//!
//! **`#[mutagen(mut_weight = 1.0)]`**
//!
//! When applied to a struct field, it affects how often that field is mutated.
//! By default, all fields have weight 1.
//!
//! Note that when a field has a weight of 0, it will never be mutated, so the derived impl
//! will not expect its fields to implement Mutatable.
//!
//! **`#[mutagen(mut_reroll = 0.5)]`**
//!
//! When applied to an enum, it sets the probability that an enum variant will be rerolled.
//! When applied to an enum variant, it overrides the value set on the enum for that particular variant.

#[doc(no_inline)]
/// The `rand` dependency, re-exported for ease of access
pub use rand;

#[doc(hidden)]
pub use mutagen_derive::*;

use std::{ops::DerefMut, rc::Rc, sync::Arc};

use rand::Rng;

#[derive(Clone, Copy, Default)]
pub struct State {
    pub depth: usize,
}

impl State {
    pub fn deepen(self) -> Self {
        Self {
            depth: self.depth + 1,
            ..self
        }
    }
}

/// A trait denoting that the type may be randomly generated
///
/// For more information, consult the [crate docs](crate).
pub trait Generatable: Sized {
    /// Convenience shorthand for `Self::generate_rng(&mut rand::thread_rng())`
    fn generate() -> Self {
        Self::generate_rng(&mut rand::thread_rng(), State::default())
    }

    /// The main required method for generation
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: State) -> Self;
}

impl<T: Generatable> Generatable for Box<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: State) -> Self {
        Box::new(T::generate_rng(rng, state))
    }
}

impl<T: Generatable> Generatable for Rc<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: State) -> Self {
        Rc::new(T::generate_rng(rng, state))
    }
}

impl<T: Generatable> Generatable for Arc<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: State) -> Self {
        Arc::new(T::generate_rng(rng, state))
    }
}

impl Generatable for () {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R, _state: State) -> Self {
        ()
    }
}

/// A trait denoting that the type may be randomly mutated
///
/// # Derive
/// When derived on a struct, it will randomly pick a field to mutate and call that field's [`mutate()`](crate::Mutatable::mutate)
///
/// When derived on an enum, it requires the enum to also implement [Generatable](crate::Generatable).
/// It will randomly choose between mutating a different variant, in which case it will generate it with [Generate](crate::Generatable),
/// or it will mutate the contents of its current variant.
///
/// ## Attributes
///
pub trait Mutatable {
    fn mutate(&mut self) {
        self.mutate_rng(&mut rand::thread_rng(), State::default())
    }

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: State);
}

impl<T: Mutatable> Mutatable for Box<T> {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: State) {
        self.deref_mut().mutate_rng(rng, state)
    }
}

impl Mutatable for () {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R, _state: State) {}
}

/// A trait denoting that the type may be updated.
///
/// # Derive
/// When derived on a struct, it will call each field's [`update()`](crate::Updatable::update)
///
/// When derived on an enum, it will call the current variant's [`update()`](crate::Updatable::update)
pub trait Updatable {
    fn update(&mut self, state: State);
}

impl<T: Updatable> Updatable for Box<T> {
    fn update(&mut self, state: State) {
        self.deref_mut().update(state)
    }
}

impl Updatable for () {
    fn update(&mut self, _state: State) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Generatable, Mutatable, Updatable)]
    struct Foo {
        #[mutagen(mut_weight = 10.0)]
        bar: Bar,
        baz: Baz,
        bax: Bax,
        bap: Bap,
    }

    #[derive(Generatable, Mutatable, Updatable)]
    struct Bar;

    #[derive(Generatable, Mutatable, Updatable)]
    #[mutagen(mut_reroll = 0.123)]
    enum Baz {
        #[mutagen(gen_weight = 10.0, mut_reroll = 1.0)]
        Boz,
        Bop(Bar),
        Bof(Bar, Bar),
        Bob {
            bar: Bar,
        },
    }

    #[derive(Generatable, Mutatable, Updatable)]
    struct Bax(Bar);

    #[derive(Generatable, Mutatable, Updatable)]
    struct Bap(Bar, Bar);
}

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
//! When derived on an enum, it requires [Generatable] to also be implemented.
//! It will then choose whether to re-roll a new variant (by default with probability 0.5),
//! or to mutate its current variant.
//!
//! # Attributes
//!
//! This crate makes extensive use of attributes to customize the behaviour of its derive macros.
//!
//! Here's a rather contrived example that showcases all of the above:
//!
//! ```rust
//! use mutagen::{Generatable, Mutatable};
//!
//! #[derive(Generatable)]
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
//!   // All other variants have reroll probability of 0.78, as specified by Foo
//!   Bax {
//!      // a mutates twice as often as b
//!      #[mutagen(mut_weight = 0.5)]
//!      a: Baz,
//!      b: Baz,
//!   },
//! }
//!
//! #[derive(Generatable)]
//! struct Baz;
//! ```
//!
//! **`#[mutagen(gen_weight = 1.0)]`**
//!
//! When applied to an enum variant, it affects how often that variant is generated.
//! By default, all variants have weight 1.
//!
//! **`#[mutagen(mut_weight = 1.0)]`**
//!
//! When applied to a struct field, it affects how often that field is mutated.
//! By default, all fields have weight 1.
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

use std::{rc::Rc, sync::Arc};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// A trait denoting that the type may be randomly generated
///
/// For more information, consult the [crate docs](crate).
pub trait Generatable: Sized {
    /// Convenience shorthand for `Self::generate_rng(&mut rand::thread_rng())`
    fn generate() -> Self {
        Self::generate_rng(&mut rand::thread_rng())
    }

    /// The main required method for generation
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

impl<T: Generatable> Generatable for Box<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Box::new(T::generate_rng(rng))
    }
}

impl<T: Generatable> Generatable for Rc<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Rc::new(T::generate_rng(rng))
    }
}

impl<T: Generatable> Generatable for Arc<T> {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Arc::new(T::generate_rng(rng))
    }
}

impl Generatable for () {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        ()
    }
}

/// A trait denoting that the type may be randomly mutated
///
/// This trait is already implemented for all types that `[rand::distributions::Standard]` applies to,
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
    fn mutate(&mut self);
}

impl<T> Mutatable for T
where
    Standard: Distribution<T>,
{
    fn mutate(&mut self) {}
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[derive(Generatable)]
    struct Foo {
        bar: Bar,
        baz: Baz,
        bax: Bax,
        bap: Bap,
    }

    #[derive(Generatable)]
    struct Bar;

    #[derive(Generatable)]
    enum Baz {
        #[mutagen(gen_weight = 10.0)]
        Boz,
        Bop(Bar),
        Bof(Bar, Bar),
        Bob {
            bar: Bar,
        },
    }

    #[derive(Generatable)]
    struct Bax(Bar);

    #[derive(Generatable)]
    struct Bap(Bar, Bar);
}
*/

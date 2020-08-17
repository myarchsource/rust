//! This module contains impls of `Fold` for those types that
//! introduce binders.
//!
//! The more interesting impls of `Fold` remain in the `fold` module.

use crate::interner::TargetInterner;
use crate::*;

impl<I: Interner, TI: TargetInterner<I>> Fold<I, TI> for Fn<I> {
    type Result = Fn<TI>;
    fn fold_with<'i>(
        &self,
        folder: &mut dyn Folder<'i, I, TI>,
        outer_binder: DebruijnIndex,
    ) -> Fallible<Self::Result>
    where
        I: 'i,
        TI: 'i,
    {
        let Fn {
            num_binders,
            substitution,
        } = self;
        Ok(Fn {
            num_binders: *num_binders,
            substitution: substitution.fold_with(folder, outer_binder.shifted_in())?,
        })
    }
}

impl<T, I: Interner, TI: TargetInterner<I>> Fold<I, TI> for Binders<T>
where
    T: HasInterner<Interner = I> + Fold<I, TI>,
    <T as Fold<I, TI>>::Result: HasInterner<Interner = TI>,
    I: Interner,
{
    type Result = Binders<T::Result>;
    fn fold_with<'i>(
        &self,
        folder: &mut dyn Folder<'i, I, TI>,
        outer_binder: DebruijnIndex,
    ) -> Fallible<Self::Result>
    where
        I: 'i,
        TI: 'i,
    {
        let Binders {
            binders: self_binders,
            value: self_value,
        } = self;
        let value = self_value.fold_with(folder, outer_binder.shifted_in())?;
        let binders = ParameterKinds {
            interned: TI::transfer_parameter_kinds(self_binders.interned().clone()),
        };
        Ok(Binders::new(binders, value))
    }
}

impl<I, T, TI> Fold<I, TI> for Canonical<T>
where
    I: Interner,
    T: HasInterner<Interner = I> + Fold<I, TI>,
    <T as Fold<I, TI>>::Result: HasInterner<Interner = TI>,
    TI: TargetInterner<I>,
{
    type Result = Canonical<T::Result>;
    fn fold_with<'i>(
        &self,
        folder: &mut dyn Folder<'i, I, TI>,
        outer_binder: DebruijnIndex,
    ) -> Fallible<Self::Result>
    where
        I: 'i,
        TI: 'i,
    {
        let Canonical {
            binders: self_binders,
            value: self_value,
        } = self;
        let value = self_value.fold_with(folder, outer_binder.shifted_in())?;
        let binders = CanonicalVarKinds {
            interned: TI::transfer_canonical_var_kinds(self_binders.interned().clone()),
        };
        Ok(Canonical {
            binders: binders,
            value: value,
        })
    }
}

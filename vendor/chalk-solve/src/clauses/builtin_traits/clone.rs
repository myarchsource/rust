use crate::clauses::ClauseBuilder;
use crate::{Interner, RustIrDatabase, TraitRef};
use chalk_ir::TyData;

use super::copy::add_copy_program_clauses;

pub fn add_clone_program_clauses<I: Interner>(
    db: &dyn RustIrDatabase<I>,
    builder: &mut ClauseBuilder<'_, I>,
    trait_ref: &TraitRef<I>,
    ty: &TyData<I>,
) {
    let _interner = db.interner();

    // Implement Clone for types that automaticly implement Copy
    add_copy_program_clauses(db, builder, trait_ref, ty);
}

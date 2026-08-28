//! P0.4A source-only typed orchestration for one Hepta Intelligence mutation.
//!
//! This module defines the legal mutation order, immutable operation/lease/
//! revision/generation bindings, causal transition receipts, bounded replay,
//! and crash reconciliation semantics. It is deliberately dormant: no SQLite
//! journal, production writer, caller, projection pointer, outbox, or external
//! effect is wired by this source tranche.

// Kept as bounded include files so each contract surface is reviewable in
// isolation while the compiler still sees one private module.
include!("intelligence_mutation_state/types_prelude.rs");
include!("intelligence_mutation_state/binding.rs");
include!("intelligence_mutation_state/reconciliation.rs");
include!("intelligence_mutation_state/actions.rs");
include!("intelligence_mutation_state/records.rs");
include!("intelligence_mutation_state/state_core.rs");
include!("intelligence_mutation_state/state_invariants.rs");
include!("intelligence_mutation_state/state_transitions.rs");
include!("intelligence_mutation_state/receipt_errors.rs");
include!("intelligence_mutation_state/receipt_digest.rs");
include!("intelligence_mutation_state/receipt_helpers.rs");
include!("intelligence_mutation_state/tests_wrapper.rs");

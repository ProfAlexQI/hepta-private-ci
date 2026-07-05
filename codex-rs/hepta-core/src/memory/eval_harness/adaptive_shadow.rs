mod comparison;
mod report;
mod result;

use super::ContextMemoryEvalFixtureKind;

pub use comparison::ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict;
pub use report::ContextMemoryAdaptiveAllocatorEvalShadowReport;
pub use result::ContextMemoryAdaptiveAllocatorEvalArm;
pub use result::ContextMemoryAdaptiveAllocatorEvalShadowResult;
pub use result::ContextMemoryAdaptiveAllocatorEvalShadowVerdict;

pub(in crate::memory) fn adaptive_allocator_eval_required_fixture_kinds()
-> [ContextMemoryEvalFixtureKind; 2] {
    [
        ContextMemoryEvalFixtureKind::SyntheticLongSession,
        ContextMemoryEvalFixtureKind::RedactedTrace,
    ]
}

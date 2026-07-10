const CURRENT_REALITY_MATRIX_REPORT_SOURCE: &str =
    include_str!("../../../scripts/hepta-systems-current-reality-capability-matrix-report.sh");

/// Count the capability rows in the matrix's canonical `$capabilities` registry.
///
/// The shell report owns readiness evaluation. Rust readbacks consume only this
/// registry-derived row count and do not claim a separately maintained ready
/// count.
pub fn current_reality_capability_registry_count() -> usize {
    let end = CURRENT_REALITY_MATRIX_REPORT_SOURCE
        .find("\n  ] as $capabilities |")
        .expect("current reality matrix must close the capability registry");
    let before_end = &CURRENT_REALITY_MATRIX_REPORT_SOURCE[..end];
    let start = before_end
        .rfind("\n  [\n    {\n      id:")
        .expect("current reality matrix must open the capability registry");
    CURRENT_REALITY_MATRIX_REPORT_SOURCE[start..end]
        .matches("\n      id:")
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_count_is_derived_from_the_matrix_registry() {
        let first = current_reality_capability_registry_count();
        let second = current_reality_capability_registry_count();

        assert!(first > 0);
        assert_eq!(first, second);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_boundary_source_has_no_runtime_or_authority_activation() {
        assert!(!B1_PROVIDER_BOUNDARY_RUNTIME_REGISTERED);
        assert!(!B1_PROVIDER_BOUNDARY_PRODUCTION_CALLER);
        assert!(!B1_PROVIDER_BOUNDARY_PRODUCTION_WRITER);
        assert!(!B1_PROVIDER_BOUNDARY_MODEL_INVOCATION);
        assert!(!B1_PROVIDER_BOUNDARY_PROVIDER_DISPATCH);
        assert!(!B1_PROVIDER_BOUNDARY_EXTERNAL_EFFECT);
        assert!(!B1_PROVIDER_BOUNDARY_OPERATOR_ACCEPTANCE);
        assert!(!B1_PROVIDER_BOUNDARY_PROMOTION);
        assert!(!B1_PROVIDER_BOUNDARY_RELEASE);
    }
}

use super::LiveSnapshotHttpExecutor;

/// Private proof carried only by an activation issued from this sealed module.
///
/// Production code currently has no constructor for this proof. A future
/// backend integration must land its authenticated in-process handshake in
/// this module before it can issue an activation to the App.
struct BackendAuthenticationProof {
    _sealed: (),
}

impl BackendAuthenticationProof {
    fn is_authenticated(&self) -> bool {
        true
    }
}

/// One-shot, backend-owned activation material consumed by the real App.
///
/// The fields and production constructor are deliberately unavailable outside
/// this sealed module. In particular, Matrix actions, environment variables,
/// fixtures, and arbitrary sibling modules cannot construct this value in a
/// production build. It contains no Matrix access token.
pub(crate) struct BackendAuthenticatedBridgeActivation<E> {
    proof: BackendAuthenticationProof,
    matrix_user_id: String,
    endpoint: String,
    explicit_user_opt_in: bool,
    authoritative_snapshot_contract: bool,
    executor: E,
}

impl<E> BackendAuthenticatedBridgeActivation<E>
where
    E: LiveSnapshotHttpExecutor,
{
    pub(crate) fn matrix_user_id(&self) -> &str {
        &self.matrix_user_id
    }

    pub(super) fn into_parts(self) -> (bool, String, String, bool, bool, E) {
        (
            self.proof.is_authenticated(),
            self.matrix_user_id,
            self.endpoint,
            self.explicit_user_opt_in,
            self.authoritative_snapshot_contract,
            self.executor,
        )
    }

    /// Synthetic constructor compiled only into the crate's negative and
    /// lifecycle unit tests. Its values can never enter a production binary.
    #[cfg(test)]
    pub(crate) fn for_test(
        matrix_user_id: impl Into<String>,
        endpoint: impl Into<String>,
        explicit_user_opt_in: bool,
        authoritative_snapshot_contract: bool,
        executor: E,
    ) -> Self {
        Self {
            proof: BackendAuthenticationProof { _sealed: () },
            matrix_user_id: matrix_user_id.into(),
            endpoint: endpoint.into(),
            explicit_user_opt_in,
            authoritative_snapshot_contract,
            executor,
        }
    }
}

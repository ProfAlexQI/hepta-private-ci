use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;

use crate::AuthorityAction;
use crate::AuthorityGrant;
use crate::RuntimeAuthorityProfile;
use crate::Sha256Digest;

pub const PRODUCT_GRAPH_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductComponentId {
    Supervisor,
    Agentd,
    AppServer,
    MemoryRuntime,
    AutomationRuntime,
    MatrixIngress,
    QualificationPlane,
}

impl ProductComponentId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supervisor => "supervisor",
            Self::Agentd => "agentd",
            Self::AppServer => "app_server",
            Self::MemoryRuntime => "memory_runtime",
            Self::AutomationRuntime => "automation_runtime",
            Self::MatrixIngress => "matrix_ingress",
            Self::QualificationPlane => "qualification_plane",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductEdgeKind {
    DependsOn,
    Composes,
    Hosts,
    ReadsFrom,
    SubmitsTo,
}

impl ProductEdgeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependsOn => "depends_on",
            Self::Composes => "composes",
            Self::Hosts => "hosts",
            Self::ReadsFrom => "reads_from",
            Self::SubmitsTo => "submits_to",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductEdge {
    pub from: ProductComponentId,
    pub to: ProductComponentId,
    pub kind: ProductEdgeKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DataDomain {
    FleetRegistry,
    AgentLifecycle,
    ThreadSession,
    MemoryLedger,
    KnowledgeProjection,
    AutomationSchedule,
    IngressProjection,
    RuntimeHealth,
}

impl DataDomain {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FleetRegistry => "fleet_registry",
            Self::AgentLifecycle => "agent_lifecycle",
            Self::ThreadSession => "thread_session",
            Self::MemoryLedger => "memory_ledger",
            Self::KnowledgeProjection => "knowledge_projection",
            Self::AutomationSchedule => "automation_schedule",
            Self::IngressProjection => "ingress_projection",
            Self::RuntimeHealth => "runtime_health",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DataStoreKind {
    JsonRegistry,
    AgentPrivateSqlite,
    ProcessMemory,
}

impl DataStoreKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonRegistry => "json_registry",
            Self::AgentPrivateSqlite => "agent_private_sqlite",
            Self::ProcessMemory => "process_memory",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DataAuthority {
    pub domain: DataDomain,
    pub writer: ProductComponentId,
    pub store: DataStoreKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductGraph {
    schema_version: u32,
    authority_grant_sha256: Sha256Digest,
    components: Vec<ProductComponentId>,
    edges: Vec<ProductEdge>,
    data_authorities: Vec<DataAuthority>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductGraphError {
    UnsupportedAuthorityProfile,
    EscapedAuthority(Vec<AuthorityAction>),
    MissingRequiredAuthority(AuthorityAction),
    DuplicateComponent,
    QualificationComponentInProductGraph,
    EdgeReferencesUnknownComponent,
    DuplicateEdge,
    DependencyCycle,
    DuplicateDataWriter(DataDomain),
    MissingDataWriter(DataDomain),
    DataWriterNotInGraph,
}

impl fmt::Display for ProductGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedAuthorityProfile => {
                formatter.write_str("authority profile cannot host the Agent product graph")
            }
            Self::EscapedAuthority(actions) => {
                write!(formatter, "product graph received escaped authority: {actions:?}")
            }
            Self::MissingRequiredAuthority(action) => write!(
                formatter,
                "product graph is missing required authority {}",
                action.as_str()
            ),
            Self::DuplicateComponent => formatter.write_str("product graph has duplicate components"),
            Self::QualificationComponentInProductGraph => formatter
                .write_str("qualification plane cannot be a product runtime component"),
            Self::EdgeReferencesUnknownComponent => {
                formatter.write_str("product graph edge references an unknown component")
            }
            Self::DuplicateEdge => formatter.write_str("product graph has a duplicate edge"),
            Self::DependencyCycle => formatter.write_str("product graph dependency cycle detected"),
            Self::DuplicateDataWriter(domain) => write!(
                formatter,
                "data domain {} has more than one writer",
                domain.as_str()
            ),
            Self::MissingDataWriter(domain) => {
                write!(formatter, "data domain {} has no writer", domain.as_str())
            }
            Self::DataWriterNotInGraph => {
                formatter.write_str("data authority writer is not a product component")
            }
        }
    }
}

impl std::error::Error for ProductGraphError {}

impl ProductGraph {
    pub fn agent_local(authority: &AuthorityGrant) -> Result<Self, ProductGraphError> {
        if !matches!(
            authority.profile(),
            RuntimeAuthorityProfile::AgentLocal
                | RuntimeAuthorityProfile::QualificationCognitiveWrite
        ) {
            return Err(ProductGraphError::UnsupportedAuthorityProfile);
        }
        let escaped = authority.dangerous_actions();
        if !escaped.is_empty() {
            return Err(ProductGraphError::EscapedAuthority(escaped));
        }
        for required in [
            AuthorityAction::ServeSession,
            AuthorityAction::ReadMemory,
            AuthorityAction::MutateMemoryFederation,
            AuthorityAction::MutateAutomation,
        ] {
            if !authority.allows(required) {
                return Err(ProductGraphError::MissingRequiredAuthority(required));
            }
        }

        let graph = Self {
            schema_version: PRODUCT_GRAPH_SCHEMA_VERSION,
            authority_grant_sha256: authority.digest(),
            components: vec![
                ProductComponentId::Supervisor,
                ProductComponentId::Agentd,
                ProductComponentId::AppServer,
                ProductComponentId::MemoryRuntime,
                ProductComponentId::AutomationRuntime,
                ProductComponentId::MatrixIngress,
            ],
            edges: vec![
                ProductEdge { from: ProductComponentId::Agentd, to: ProductComponentId::Supervisor, kind: ProductEdgeKind::DependsOn },
                ProductEdge { from: ProductComponentId::Agentd, to: ProductComponentId::MemoryRuntime, kind: ProductEdgeKind::Composes },
                ProductEdge { from: ProductComponentId::Agentd, to: ProductComponentId::AutomationRuntime, kind: ProductEdgeKind::Composes },
                ProductEdge { from: ProductComponentId::Agentd, to: ProductComponentId::AppServer, kind: ProductEdgeKind::Hosts },
                ProductEdge { from: ProductComponentId::AppServer, to: ProductComponentId::MemoryRuntime, kind: ProductEdgeKind::ReadsFrom },
                ProductEdge { from: ProductComponentId::AutomationRuntime, to: ProductComponentId::AppServer, kind: ProductEdgeKind::SubmitsTo },
                ProductEdge { from: ProductComponentId::MatrixIngress, to: ProductComponentId::Agentd, kind: ProductEdgeKind::SubmitsTo },
            ],
            data_authorities: vec![
                DataAuthority { domain: DataDomain::FleetRegistry, writer: ProductComponentId::Supervisor, store: DataStoreKind::JsonRegistry },
                DataAuthority { domain: DataDomain::AgentLifecycle, writer: ProductComponentId::Supervisor, store: DataStoreKind::JsonRegistry },
                DataAuthority { domain: DataDomain::ThreadSession, writer: ProductComponentId::AppServer, store: DataStoreKind::AgentPrivateSqlite },
                DataAuthority { domain: DataDomain::MemoryLedger, writer: ProductComponentId::MemoryRuntime, store: DataStoreKind::AgentPrivateSqlite },
                DataAuthority { domain: DataDomain::KnowledgeProjection, writer: ProductComponentId::MemoryRuntime, store: DataStoreKind::AgentPrivateSqlite },
                DataAuthority { domain: DataDomain::AutomationSchedule, writer: ProductComponentId::AutomationRuntime, store: DataStoreKind::AgentPrivateSqlite },
                DataAuthority { domain: DataDomain::IngressProjection, writer: ProductComponentId::MatrixIngress, store: DataStoreKind::AgentPrivateSqlite },
                DataAuthority { domain: DataDomain::RuntimeHealth, writer: ProductComponentId::Agentd, store: DataStoreKind::ProcessMemory },
            ],
        };
        graph.validate()?;
        Ok(graph)
    }

    pub fn schema_version(&self) -> u32 { self.schema_version }
    pub fn authority_grant_sha256(&self) -> &Sha256Digest { &self.authority_grant_sha256 }
    pub fn components(&self) -> &[ProductComponentId] { &self.components }
    pub fn edges(&self) -> &[ProductEdge] { &self.edges }
    pub fn data_authorities(&self) -> &[DataAuthority] { &self.data_authorities }

    pub fn writer_for(&self, domain: DataDomain) -> Option<ProductComponentId> {
        self.data_authorities.iter().find(|authority| authority.domain == domain).map(|authority| authority.writer)
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:product-graph:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.authority_grant_sha256.as_str().as_bytes());
        for component in &self.components { frame(&mut bytes, component.as_str().as_bytes()); }
        for edge in &self.edges {
            frame(&mut bytes, edge.from.as_str().as_bytes());
            frame(&mut bytes, edge.to.as_str().as_bytes());
            frame(&mut bytes, edge.kind.as_str().as_bytes());
        }
        for authority in &self.data_authorities {
            frame(&mut bytes, authority.domain.as_str().as_bytes());
            frame(&mut bytes, authority.writer.as_str().as_bytes());
            frame(&mut bytes, authority.store.as_str().as_bytes());
        }
        Sha256Digest::for_bytes(&bytes)
    }

    pub fn validate(&self) -> Result<(), ProductGraphError> {
        let component_set: BTreeSet<_> = self.components.iter().copied().collect();
        if component_set.len() != self.components.len() { return Err(ProductGraphError::DuplicateComponent); }
        if component_set.contains(&ProductComponentId::QualificationPlane) { return Err(ProductGraphError::QualificationComponentInProductGraph); }
        let mut edge_set = BTreeSet::new();
        for edge in &self.edges {
            if !component_set.contains(&edge.from) || !component_set.contains(&edge.to) { return Err(ProductGraphError::EdgeReferencesUnknownComponent); }
            if !edge_set.insert((edge.from, edge.to, edge.kind)) { return Err(ProductGraphError::DuplicateEdge); }
        }
        validate_acyclic(&component_set, &self.edges)?;
        let mut writers = BTreeMap::new();
        for authority in &self.data_authorities {
            if !component_set.contains(&authority.writer) { return Err(ProductGraphError::DataWriterNotInGraph); }
            if writers.insert(authority.domain, authority.writer).is_some() { return Err(ProductGraphError::DuplicateDataWriter(authority.domain)); }
        }
        for domain in [DataDomain::FleetRegistry, DataDomain::AgentLifecycle, DataDomain::ThreadSession, DataDomain::MemoryLedger, DataDomain::KnowledgeProjection, DataDomain::AutomationSchedule, DataDomain::IngressProjection, DataDomain::RuntimeHealth] {
            if !writers.contains_key(&domain) { return Err(ProductGraphError::MissingDataWriter(domain)); }
        }
        Ok(())
    }
}

fn validate_acyclic(components: &BTreeSet<ProductComponentId>, edges: &[ProductEdge]) -> Result<(), ProductGraphError> {
    let mut indegree = components.iter().copied().map(|component| (component, 0usize)).collect::<BTreeMap<_, _>>();
    let mut outgoing = components.iter().copied().map(|component| (component, Vec::new())).collect::<BTreeMap<_, _>>();
    for edge in edges {
        if let Some(value) = indegree.get_mut(&edge.to) { *value += 1; }
        if let Some(targets) = outgoing.get_mut(&edge.from) { targets.push(edge.to); }
    }
    let mut ready = indegree.iter().filter_map(|(component, degree)| (*degree == 0).then_some(*component)).collect::<BTreeSet<_>>();
    let mut visited = 0usize;
    while let Some(component) = ready.pop_first() {
        visited += 1;
        if let Some(targets) = outgoing.get(&component) {
            for target in targets {
                if let Some(degree) = indegree.get_mut(target) {
                    *degree -= 1;
                    if *degree == 0 { ready.insert(*target); }
                }
            }
        }
    }
    if visited != components.len() { return Err(ProductGraphError::DependencyCycle); }
    Ok(())
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;
    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
    fn agent_id() -> AgentId { AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("test AgentId must parse: {error}")) }

    #[test]
    fn agent_product_graph_is_acyclic_and_single_writer() {
        let grant = AuthorityGrant::agent_local(agent_id(), 1).unwrap_or_else(|error| panic!("grant must be valid: {error}"));
        let graph = ProductGraph::agent_local(&grant).unwrap_or_else(|error| panic!("product graph must be valid: {error}"));
        assert!(graph.validate().is_ok());
        assert_eq!(graph.writer_for(DataDomain::MemoryLedger), Some(ProductComponentId::MemoryRuntime));
        assert_eq!(graph.writer_for(DataDomain::ThreadSession), Some(ProductComponentId::AppServer));
        assert!(!graph.components().contains(&ProductComponentId::QualificationPlane));
    }

    #[test]
    fn qualification_profile_changes_authority_not_topology() {
        let local = AuthorityGrant::agent_local(agent_id(), 4).unwrap_or_else(|error| panic!("local grant must be valid: {error}"));
        let qualification = AuthorityGrant::qualification_cognitive_write(agent_id(), 4).unwrap_or_else(|error| panic!("qualification grant must be valid: {error}"));
        let local_graph = ProductGraph::agent_local(&local).unwrap_or_else(|error| panic!("local graph must be valid: {error}"));
        let qualification_graph = ProductGraph::agent_local(&qualification).unwrap_or_else(|error| panic!("qualification graph must be valid: {error}"));
        assert_eq!(local_graph.components(), qualification_graph.components());
        assert_eq!(local_graph.edges(), qualification_graph.edges());
        assert_eq!(local_graph.data_authorities(), qualification_graph.data_authorities());
        assert_ne!(local_graph.authority_grant_sha256(), qualification_graph.authority_grant_sha256());
    }

    #[test]
    fn snapshot_profile_cannot_host_agent_product_graph() {
        let snapshot = AuthorityGrant::snapshot_read_only(agent_id(), 1).unwrap_or_else(|error| panic!("snapshot grant must be valid: {error}"));
        assert!(matches!(ProductGraph::agent_local(&snapshot), Err(ProductGraphError::UnsupportedAuthorityProfile)));
    }

    #[test]
    fn duplicate_data_writer_fails_closed() {
        let grant = AuthorityGrant::agent_local(agent_id(), 2).unwrap_or_else(|error| panic!("grant must be valid: {error}"));
        let mut graph = ProductGraph::agent_local(&grant).unwrap_or_else(|error| panic!("product graph must be valid: {error}"));
        graph.data_authorities.push(DataAuthority { domain: DataDomain::MemoryLedger, writer: ProductComponentId::Agentd, store: DataStoreKind::ProcessMemory });
        assert!(matches!(graph.validate(), Err(ProductGraphError::DuplicateDataWriter(DataDomain::MemoryLedger))));
    }
}

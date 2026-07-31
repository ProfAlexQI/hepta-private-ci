use crate::route_manifest::RouteDefinition;
use crate::route_manifest::RouteDispatchHandler;
use crate::route_manifest::RouteResponsePolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReportDescriptor {
    pub(crate) renderer: ReportRenderer,
    pub(crate) response_policy: RouteResponsePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReportRenderer {
    NativeGatewayJson,
    CanonicalEvidenceJson,
}

impl RouteDefinition {
    pub(crate) fn report_descriptor(self) -> Option<ReportDescriptor> {
        if self.lifecycle.method != "GET" {
            return None;
        }
        match self.dispatch_handler {
            RouteDispatchHandler::NativeGateway => Some(ReportDescriptor {
                renderer: ReportRenderer::NativeGatewayJson,
                response_policy: self.response_policy,
            }),
            RouteDispatchHandler::EvidenceIndex => Some(ReportDescriptor {
                renderer: ReportRenderer::CanonicalEvidenceJson,
                response_policy: self.response_policy,
            }),
            _ => None,
        }
    }
}

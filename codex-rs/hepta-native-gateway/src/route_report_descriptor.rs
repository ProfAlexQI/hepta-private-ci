use crate::route_manifest::RouteDispatchHandler;
use crate::route_manifest::RouteManifestEntry;
use crate::route_manifest::RouteResponsePolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReportDescriptor {
    pub(crate) renderer: ReportRenderer,
    pub(crate) response_policy: RouteResponsePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReportRenderer {
    NativeGatewayJson,
}

impl RouteManifestEntry {
    pub(crate) fn report_descriptor(self) -> Option<ReportDescriptor> {
        (self.lifecycle.method == "GET"
            && self.dispatch_handler == RouteDispatchHandler::NativeGateway)
            .then_some(ReportDescriptor {
                renderer: ReportRenderer::NativeGatewayJson,
                response_policy: self.response_policy,
            })
    }
}

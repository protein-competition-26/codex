//! Shares the root user's routing tier as the fallback for subagents without an override.

use super::LocalAgentControl;
use std::sync::Arc;

impl LocalAgentControl {
    /// Returns the latest user-selected tier for the root and inheriting descendants.
    pub(crate) fn root_service_tier(&self) -> Option<String> {
        self.runtime
            .root_service_tier
            .load_full()
            .map(|service_tier| (*service_tier).clone())
    }

    /// Publishes a root-owned tier without mutating individual child sessions.
    pub(crate) fn set_root_service_tier(&self, service_tier: Option<String>) {
        self.runtime
            .root_service_tier
            .store(service_tier.map(Arc::new));
    }
}

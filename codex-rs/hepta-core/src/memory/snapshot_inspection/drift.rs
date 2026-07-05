use serde::Deserialize;
use serde::Serialize;

use super::super::MemoryRecord;
use super::super::SessionRecord;
use super::super::TranscriptEntry;
use super::inspected::SnapshotInspectionBundle;

/// Named inspection-bundle section used by snapshot drift reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotInspectionSection {
    MemoryManifest,
    MemoryIntegrity,
    TranscriptManifest,
    TranscriptIntegrity,
}

/// Section-level drift summary for a persisted inspection bundle.
///
/// This lets automation and import/export tooling distinguish between a fully
/// aligned envelope and one where only part of the derived inspection view has
/// drifted away from the full snapshot payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionDriftReport {
    #[serde(default)]
    pub mismatched_sections: Vec<SnapshotInspectionSection>,
}

impl SnapshotInspectionDriftReport {
    pub fn from_bundle_and_records(
        inspection: &SnapshotInspectionBundle,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        let expected =
            SnapshotInspectionBundle::from_records_and_entries(sessions, memories, transcripts);
        let mut mismatched_sections = Vec::new();

        if inspection.memory_manifest != expected.memory_manifest {
            mismatched_sections.push(SnapshotInspectionSection::MemoryManifest);
        }
        if inspection.memory_integrity != expected.memory_integrity {
            mismatched_sections.push(SnapshotInspectionSection::MemoryIntegrity);
        }
        if inspection.transcript_manifest != expected.transcript_manifest {
            mismatched_sections.push(SnapshotInspectionSection::TranscriptManifest);
        }
        if inspection.transcript_integrity != expected.transcript_integrity {
            mismatched_sections.push(SnapshotInspectionSection::TranscriptIntegrity);
        }

        Self {
            mismatched_sections,
        }
    }

    pub fn mismatch_count(&self) -> usize {
        self.mismatched_sections.len()
    }

    pub fn mismatches(&self, section: SnapshotInspectionSection) -> bool {
        self.mismatched_sections.contains(&section)
    }

    pub fn memory_mismatch_count(&self) -> usize {
        self.mismatched_sections
            .iter()
            .filter(|section| {
                matches!(
                    section,
                    SnapshotInspectionSection::MemoryManifest
                        | SnapshotInspectionSection::MemoryIntegrity
                )
            })
            .count()
    }

    pub fn transcript_mismatch_count(&self) -> usize {
        self.mismatched_sections
            .iter()
            .filter(|section| {
                matches!(
                    section,
                    SnapshotInspectionSection::TranscriptManifest
                        | SnapshotInspectionSection::TranscriptIntegrity
                )
            })
            .count()
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_mismatch_count() > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_mismatch_count() > 0
    }

    /// Returns a compact domain-level drift summary for automation that does
    /// not need per-section names.
    pub fn impact(&self) -> SnapshotInspectionDriftImpact {
        SnapshotInspectionDriftImpact::from_report(self)
    }

    pub fn is_aligned(&self) -> bool {
        self.mismatched_sections.is_empty()
    }
}

/// Compact domain-level summary derived from a section-level inspection drift
/// report.
///
/// This lets automation and doctor-style checks answer whether inspection
/// drift touches memory-derived views, transcript-derived views, or both,
/// without carrying the individual mismatched section names.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionDriftImpact {
    pub mismatch_count: usize,
    pub memory_mismatch_count: usize,
    pub transcript_mismatch_count: usize,
}

impl SnapshotInspectionDriftImpact {
    pub fn from_report(report: &SnapshotInspectionDriftReport) -> Self {
        Self {
            mismatch_count: report.mismatch_count(),
            memory_mismatch_count: report.memory_mismatch_count(),
            transcript_mismatch_count: report.transcript_mismatch_count(),
        }
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_mismatch_count > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_mismatch_count > 0
    }

    pub fn is_aligned(&self) -> bool {
        self.mismatch_count == 0
    }
}

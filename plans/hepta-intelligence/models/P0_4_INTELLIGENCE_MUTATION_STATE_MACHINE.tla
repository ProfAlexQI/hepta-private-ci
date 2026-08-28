---- MODULE P0_4_INTELLIGENCE_MUTATION_STATE_MACHINE ----
EXTENDS Naturals, TLC

CONSTANTS StartGeneration, MaxReconciliations

Phases == {
  "Planned", "SourceWitnessed", "GroundingValidated",
  "DurableIntentAppended", "MemoryFactsCommitted",
  "ProjectionPublished", "OutboxSettled", "Terminal",
  "RejectedPreCommit", "CancelledPreCommit", "Indeterminate",
  "ReconciledNotApplied", "Quarantined"
}

DurablePhases == {
  "DurableIntentAppended", "MemoryFactsCommitted",
  "ProjectionPublished", "OutboxSettled"
}

Dispositions == {
  "None", "Pending", "SettledApplied", "SettledNotApplied", "Quarantined"
}

VARIABLES phase, disposition, writeCount, publishCount, outboxSettled,
          generation, indeterminateFrom, lastRecoveryOrigin,
          reconciliationCount

vars == <<phase, disposition, writeCount, publishCount, outboxSettled,
          generation, indeterminateFrom, lastRecoveryOrigin,
          reconciliationCount>>

Init ==
  /\ phase = "Planned"
  /\ disposition = "None"
  /\ writeCount = 0
  /\ publishCount = 0
  /\ outboxSettled = FALSE
  /\ generation = StartGeneration
  /\ indeterminateFrom = "None"
  /\ lastRecoveryOrigin = "None"
  /\ reconciliationCount = 0

WitnessSource ==
  /\ phase = "Planned"
  /\ phase' = "SourceWitnessed"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

ValidateGrounding ==
  /\ phase = "SourceWitnessed"
  /\ phase' = "GroundingValidated"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

RejectPreCommit ==
  /\ phase \in {"Planned", "SourceWitnessed", "GroundingValidated"}
  /\ phase' = "RejectedPreCommit"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

CancelPreCommit ==
  /\ phase \in {"Planned", "SourceWitnessed", "GroundingValidated"}
  /\ phase' = "CancelledPreCommit"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

AppendIntent ==
  /\ phase = "GroundingValidated"
  /\ phase' = "DurableIntentAppended"
  /\ disposition' = "Pending"
  /\ UNCHANGED <<writeCount, publishCount, outboxSettled, generation,
                  indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

CommitFacts ==
  /\ phase = "DurableIntentAppended"
  /\ writeCount = 0
  /\ phase' = "MemoryFactsCommitted"
  /\ writeCount' = 1
  /\ UNCHANGED <<disposition, publishCount, outboxSettled, generation,
                  indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

PublishProjection ==
  /\ phase = "MemoryFactsCommitted"
  /\ publishCount = 0
  /\ generation = StartGeneration
  /\ phase' = "ProjectionPublished"
  /\ publishCount' = 1
  /\ generation' = StartGeneration + 1
  /\ UNCHANGED <<disposition, writeCount, outboxSettled,
                  indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

SettleOutbox ==
  /\ phase = "ProjectionPublished"
  /\ phase' = "OutboxSettled"
  /\ outboxSettled' = TRUE
  /\ disposition' = "SettledApplied"
  /\ UNCHANGED <<writeCount, publishCount, generation,
                  indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

Terminalize ==
  /\ phase = "OutboxSettled"
  /\ disposition = "SettledApplied"
  /\ writeCount = 1
  /\ publishCount = 1
  /\ outboxSettled
  /\ phase' = "Terminal"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, indeterminateFrom, lastRecoveryOrigin,
                  reconciliationCount>>

MarkIndeterminate ==
  /\ phase \in DurablePhases
  /\ reconciliationCount < MaxReconciliations
  /\ indeterminateFrom' = phase
  /\ phase' = "Indeterminate"
  /\ UNCHANGED <<disposition, writeCount, publishCount, outboxSettled,
                  generation, lastRecoveryOrigin, reconciliationCount>>

ReconcileNotApplied ==
  /\ phase = "Indeterminate"
  /\ reconciliationCount < MaxReconciliations
  /\ indeterminateFrom = "DurableIntentAppended"
  /\ writeCount = 0
  /\ publishCount = 0
  /\ ~outboxSettled
  /\ phase' = "ReconciledNotApplied"
  /\ disposition' = "SettledNotApplied"
  /\ lastRecoveryOrigin' = indeterminateFrom
  /\ indeterminateFrom' = "None"
  /\ reconciliationCount' = reconciliationCount + 1
  /\ UNCHANGED <<writeCount, publishCount, outboxSettled, generation>>

ReconcileMemoryCommitted ==
  /\ phase = "Indeterminate"
  /\ reconciliationCount < MaxReconciliations
  /\ indeterminateFrom \in {"DurableIntentAppended", "MemoryFactsCommitted"}
  /\ publishCount = 0
  /\ ~outboxSettled
  /\ phase' = "MemoryFactsCommitted"
  /\ disposition' = "Pending"
  /\ writeCount' = 1
  /\ lastRecoveryOrigin' = indeterminateFrom
  /\ indeterminateFrom' = "None"
  /\ reconciliationCount' = reconciliationCount + 1
  /\ UNCHANGED <<publishCount, outboxSettled, generation>>

ReconcileProjectionPublished ==
  /\ phase = "Indeterminate"
  /\ reconciliationCount < MaxReconciliations
  /\ indeterminateFrom \in {
       "DurableIntentAppended", "MemoryFactsCommitted", "ProjectionPublished"
     }
  /\ phase' = "ProjectionPublished"
  /\ disposition' = "Pending"
  /\ writeCount' = 1
  /\ publishCount' = 1
  /\ outboxSettled' = FALSE
  /\ generation' = StartGeneration + 1
  /\ lastRecoveryOrigin' = indeterminateFrom
  /\ indeterminateFrom' = "None"
  /\ reconciliationCount' = reconciliationCount + 1

ReconcileOutboxSettled ==
  /\ phase = "Indeterminate"
  /\ reconciliationCount < MaxReconciliations
  /\ indeterminateFrom \in DurablePhases
  /\ phase' = "OutboxSettled"
  /\ disposition' = "SettledApplied"
  /\ writeCount' = 1
  /\ publishCount' = 1
  /\ outboxSettled' = TRUE
  /\ generation' = StartGeneration + 1
  /\ lastRecoveryOrigin' = indeterminateFrom
  /\ indeterminateFrom' = "None"
  /\ reconciliationCount' = reconciliationCount + 1

Quarantine ==
  /\ phase = "Indeterminate"
  /\ reconciliationCount < MaxReconciliations
  /\ phase' = "Quarantined"
  /\ disposition' = "Quarantined"
  /\ lastRecoveryOrigin' = indeterminateFrom
  /\ indeterminateFrom' = "None"
  /\ reconciliationCount' = reconciliationCount + 1
  /\ UNCHANGED <<writeCount, publishCount, outboxSettled, generation>>

Next ==
  WitnessSource \/ ValidateGrounding \/ RejectPreCommit \/ CancelPreCommit \/
  AppendIntent \/ CommitFacts \/ PublishProjection \/ SettleOutbox \/
  Terminalize \/ MarkIndeterminate \/ ReconcileNotApplied \/
  ReconcileMemoryCommitted \/ ReconcileProjectionPublished \/
  ReconcileOutboxSettled \/ Quarantine

TypeInvariant ==
  /\ phase \in Phases
  /\ disposition \in Dispositions
  /\ writeCount \in 0..1
  /\ publishCount \in 0..1
  /\ outboxSettled \in BOOLEAN
  /\ generation \in Nat
  /\ indeterminateFrom \in DurablePhases \cup {"None"}
  /\ lastRecoveryOrigin \in DurablePhases \cup {"None"}
  /\ reconciliationCount \in 0..MaxReconciliations

NoDoubleWrite == writeCount <= 1
NoDoublePublish == publishCount <= 1
GenerationBound == generation \in {StartGeneration, StartGeneration + 1}
ProjectionImpliesWrite == publishCount = 1 => writeCount = 1
OutboxImpliesProjection == outboxSettled => publishCount = 1

PreCommitFinalHasNoIntent ==
  phase \in {"RejectedPreCommit", "CancelledPreCommit"}
    => /\ disposition = "None"
       /\ writeCount = 0
       /\ publishCount = 0
       /\ ~outboxSettled

TerminalIsComplete ==
  phase = "Terminal"
    => /\ disposition = "SettledApplied"
       /\ writeCount = 1
       /\ publishCount = 1
       /\ outboxSettled

NotAppliedIsEmpty ==
  phase = "ReconciledNotApplied"
    => /\ disposition = "SettledNotApplied"
       /\ writeCount = 0
       /\ publishCount = 0
       /\ ~outboxSettled

QuarantineIsNotAppliedAuthority ==
  phase = "Quarantined" => disposition = "Quarantined"

NoPrematureAppliedTerminal ==
  phase \in {"ReconciledNotApplied", "Quarantined"}
    => disposition # "SettledApplied"

IndeterminateHasOrigin ==
  phase = "Indeterminate" => indeterminateFrom \in DurablePhases

ConcretePhaseClearsOrigin ==
  phase # "Indeterminate" => indeterminateFrom = "None"

Spec == Init /\ [][Next]_vars

====

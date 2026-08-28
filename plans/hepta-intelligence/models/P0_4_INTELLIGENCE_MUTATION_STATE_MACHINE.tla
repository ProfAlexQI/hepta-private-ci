---- MODULE P0_4_INTELLIGENCE_MUTATION_STATE_MACHINE ----
EXTENDS Naturals, TLC

CONSTANT StartGeneration

Phases == {
  "Planned", "SourceWitnessed", "GroundingValidated",
  "DurableIntentAppended", "MemoryFactsCommitted",
  "ProjectionPublished", "OutboxSettled", "Terminal",
  "Indeterminate", "ReconciledApplied",
  "ReconciledNotApplied", "Quarantined"
}

VARIABLES phase, intentAppended, intentSettled, writeCount,
          publishCount, generation, indeterminateFrom

vars == <<phase, intentAppended, intentSettled, writeCount,
          publishCount, generation, indeterminateFrom>>

Init ==
  /\ phase = "Planned"
  /\ intentAppended = FALSE
  /\ intentSettled = FALSE
  /\ writeCount = 0
  /\ publishCount = 0
  /\ generation = StartGeneration
  /\ indeterminateFrom = "None"

WitnessSource ==
  /\ phase = "Planned"
  /\ phase' = "SourceWitnessed"
  /\ UNCHANGED <<intentAppended, intentSettled, writeCount,
                  publishCount, generation, indeterminateFrom>>

ValidateGrounding ==
  /\ phase = "SourceWitnessed"
  /\ phase' = "GroundingValidated"
  /\ UNCHANGED <<intentAppended, intentSettled, writeCount,
                  publishCount, generation, indeterminateFrom>>

AppendIntent ==
  /\ phase = "GroundingValidated"
  /\ phase' = "DurableIntentAppended"
  /\ intentAppended' = TRUE
  /\ intentSettled' = FALSE
  /\ UNCHANGED <<writeCount, publishCount, generation, indeterminateFrom>>

CommitFacts ==
  /\ phase = "DurableIntentAppended"
  /\ writeCount = 0
  /\ phase' = "MemoryFactsCommitted"
  /\ writeCount' = 1
  /\ UNCHANGED <<intentAppended, intentSettled, publishCount,
                  generation, indeterminateFrom>>

PublishProjection ==
  /\ phase = "MemoryFactsCommitted"
  /\ publishCount = 0
  /\ phase' = "ProjectionPublished"
  /\ publishCount' = 1
  /\ generation' = generation + 1
  /\ UNCHANGED <<intentAppended, intentSettled, writeCount,
                  indeterminateFrom>>

SettleOutbox ==
  /\ phase = "ProjectionPublished"
  /\ phase' = "OutboxSettled"
  /\ intentSettled' = TRUE
  /\ UNCHANGED <<intentAppended, writeCount, publishCount,
                  generation, indeterminateFrom>>

Terminalize ==
  /\ phase = "OutboxSettled"
  /\ intentSettled
  /\ phase' = "Terminal"
  /\ UNCHANGED <<intentAppended, intentSettled, writeCount,
                  publishCount, generation, indeterminateFrom>>

MarkIndeterminate ==
  /\ phase \in {"DurableIntentAppended", "MemoryFactsCommitted",
                  "ProjectionPublished"}
  /\ indeterminateFrom' = phase
  /\ phase' = "Indeterminate"
  /\ UNCHANGED <<intentAppended, intentSettled, writeCount,
                  publishCount, generation>>

ReconcileApplied ==
  /\ phase = "Indeterminate"
  /\ phase' = "ReconciledApplied"
  /\ writeCount' = 1
  /\ intentSettled' = TRUE
  /\ UNCHANGED <<intentAppended, publishCount, generation,
                  indeterminateFrom>>

ReconcileNotApplied ==
  /\ phase = "Indeterminate"
  /\ indeterminateFrom = "DurableIntentAppended"
  /\ writeCount = 0
  /\ phase' = "ReconciledNotApplied"
  /\ intentSettled' = TRUE
  /\ UNCHANGED <<intentAppended, writeCount, publishCount,
                  generation, indeterminateFrom>>

Quarantine ==
  /\ phase = "Indeterminate"
  /\ phase' = "Quarantined"
  /\ intentSettled' = TRUE
  /\ UNCHANGED <<intentAppended, writeCount, publishCount,
                  generation, indeterminateFrom>>

Next == WitnessSource \/ ValidateGrounding \/ AppendIntent \/ CommitFacts \/
        PublishProjection \/ SettleOutbox \/ Terminalize \/
        MarkIndeterminate \/ ReconcileApplied \/
        ReconcileNotApplied \/ Quarantine

TypeInvariant ==
  /\ phase \in Phases
  /\ intentAppended \in BOOLEAN
  /\ intentSettled \in BOOLEAN
  /\ writeCount \in 0..1
  /\ publishCount \in 0..1
  /\ generation \in Nat

NoDoubleWrite == writeCount <= 1
NoDoublePublish == publishCount <= 1
GenerationBound == generation \in {StartGeneration, StartGeneration + 1}
TerminalImpliesSettled == phase = "Terminal" => intentSettled
ResolvedImpliesSettled ==
  phase \in {"ReconciledApplied", "ReconciledNotApplied", "Quarantined"}
    => intentSettled
ProjectionImpliesOneWrite ==
  phase \in {"ProjectionPublished", "OutboxSettled", "Terminal"}
    => writeCount = 1

Spec == Init /\ [][Next]_vars

====

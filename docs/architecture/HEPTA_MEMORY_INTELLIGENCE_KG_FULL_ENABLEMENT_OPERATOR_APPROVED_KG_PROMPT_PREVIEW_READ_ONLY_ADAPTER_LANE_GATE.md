# Hepta Memory/Intelligence/KG Full Enablement - Operator-Approved KG Prompt-Preview Read-Only Adapter Lane Gate

This gate is the third staged enablement slice after the operator-approved
Memory durable mutation lane and the Hepta Intelligence context attachment lane.

It records that the operator-approved activation lane may now expose KG
prompt-preview and read-only adapter lane authority for Graphiti, Neo4j, and
CocoIndex. This is still a lane-status report only. The report route does not
render a prompt preview, materialize a prompt payload, attach or inject context,
read KG adapters, construct external adapter clients, capture endpoints or
credential values, read credentials, write KG state, invoke a provider/model,
send to any channel, restart the service, mutate the active binary, or publish a
public claim.

Required upstream state:

- The operator-approved Memory durable mutation lane is present and effective.
- The Hepta Intelligence context attachment and bounded prompt-preview lane is
  present and effective.
- Context attachment, prompt preview, and adapter reads still require explicit
  commands outside report routes.
- KG live write, provider/model invocation, credential reads, and channel
  delivery remain disabled.

The next safe slice is explicit prompt-preview payload shape materialization,
still without provider/model invocation, KG live write, credential reads,
channel delivery, public release, or report-route side effects.

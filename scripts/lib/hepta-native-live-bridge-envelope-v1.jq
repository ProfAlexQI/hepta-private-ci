def hepta_nonblank_string:
  type == "string" and (gsub("\\s"; "") | length) > 0;

def hepta_nonnegative_integer:
  type == "number" and floor == . and . >= 0;

def hepta_integer:
  type == "number" and floor == .;

def hepta_exact_keys($required; $optional):
  (keys | sort) as $actual
  | (($required - $actual) | length) == 0
  and (($actual - ($required + $optional)) | length) == 0;

def hepta_authoritative_origin:
  type == "object"
  and (
    (hepta_exact_keys(["kind"]; []) and .kind == "hepta_runtime")
    or (
      hepta_exact_keys(["component", "kind"]; [])
      and .kind == "bridge_adapter"
      and (.component | hepta_nonblank_string)
    )
  );

def hepta_presenter_safe_redaction:
  type == "object"
  and hepta_exact_keys(["policy", "status"]; ["removed_fields"])
  and (.status == "redacted" or .status == "not_required")
  and (.policy == null or (.policy | hepta_nonblank_string))
  and ((.removed_fields // []) | type == "array")
  and ((.removed_fields // []) | all(hepta_nonblank_string));

def hepta_valid_provenance:
  type == "object"
  and hepta_exact_keys(
    ["observed_at", "source", "source_entity_id", "source_revision"];
    []
  )
  and (.source | hepta_nonblank_string)
  and (.source_entity_id == null or (.source_entity_id | hepta_nonblank_string))
  and (.source_revision == null or (.source_revision | hepta_nonnegative_integer))
  and (.observed_at | hepta_integer);

def hepta_valid_metadata($session_id; $correlation_id):
  type == "object"
  and hepta_exact_keys(
    [
      "correlation_id",
      "cursor",
      "origin",
      "provenance",
      "redaction",
      "revision",
      "schema_version",
      "session_id",
      "stable_id",
      "timestamp"
    ];
    []
  )
  and .schema_version == 1
  and (.stable_id | hepta_nonblank_string)
  and (.revision | hepta_nonnegative_integer)
  and (.cursor == null or (.cursor | hepta_nonblank_string))
  and (.timestamp | hepta_integer)
  and .session_id == $session_id
  and .correlation_id == $correlation_id
  and (.origin | hepta_authoritative_origin)
  and (.redaction | hepta_presenter_safe_redaction)
  and (.provenance | hepta_valid_provenance);

def hepta_valid_record($session_id; $correlation_id):
  type == "object"
  and hepta_exact_keys(["metadata", "state", "summary", "title"]; ["attributes"])
  and (.metadata | hepta_valid_metadata($session_id; $correlation_id))
  and (.state | hepta_nonblank_string)
  and (.title | type == "string")
  and (.summary | type == "string")
  and ((.attributes // {}) | type == "object");

def hepta_valid_record_array($session_id; $correlation_id):
  type == "array" and all(hepta_valid_record($session_id; $correlation_id));

def hepta_valid_snapshot($session_id; $correlation_id; $outer_revision):
  type == "object"
  and hepta_exact_keys(["cursor", "revision", "runtime"]; ["activities", "approvals", "tasks", "tool_invocations"])
  and (.revision | hepta_nonnegative_integer)
  and .revision == $outer_revision
  and (.cursor == null or (.cursor | hepta_nonblank_string))
  and (.runtime == null or (.runtime | hepta_valid_record($session_id; $correlation_id)))
  and ((.tasks // []) | hepta_valid_record_array($session_id; $correlation_id))
  and ((.tool_invocations // []) | hepta_valid_record_array($session_id; $correlation_id))
  and ((.approvals // []) | hepta_valid_record_array($session_id; $correlation_id))
  and ((.activities // []) | hepta_valid_record_array($session_id; $correlation_id));

def hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id):
  . as $envelope
  |
  ($session_id | hepta_nonblank_string)
  and ($correlation_id | hepta_nonblank_string)
  and type == "object"
  and hepta_exact_keys(["binding", "metadata", "update"]; [])
  and (.metadata | hepta_valid_metadata($session_id; $correlation_id))
  and (
    .binding
    | type == "object"
    and hepta_exact_keys(["hepta_session_id", "matrix_room_id", "mirror_policy", "revision"]; [])
    and .hepta_session_id == $session_id
    and .matrix_room_id == null
    and .mirror_policy == "local_only"
    and (.revision | hepta_nonnegative_integer)
    and .revision == $envelope.metadata.revision
  )
  and (
    .update
    | type == "object"
    and hepta_exact_keys(["data", "type"]; [])
    and .type == "snapshot"
    and (
      .data
      | type == "object"
      and hepta_exact_keys(["snapshot"]; [])
      and (
        .snapshot
        | hepta_valid_snapshot(
            $session_id;
            $correlation_id;
            $envelope.metadata.revision
          )
      )
    )
  );

def hepta_native_live_bridge_envelope_v1_transport_valid(
  $session_id;
  $correlation_id;
  $sequence
):
  ($sequence | hepta_nonnegative_integer)
  and $sequence > 0
  and hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id)
  and .metadata.revision == $sequence
  and .binding.revision == $sequence
  and .update.data.snapshot.revision == $sequence;

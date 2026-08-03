#!/bin/bash -p
set +x
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
PATH="/usr/bin:/bin:/usr/sbin:/sbin"; export PATH
[[ -x /usr/bin/ruby && ! -L /usr/bin/ruby ]] || { echo "promotion verifier: canonical Ruby unavailable" >&2; exit 2; }

# The public, profile-specific entrypoints are symlinks to this implementation.
# Preserve their stable command names without maintaining executable wrappers.
case "${0##*/}" in
  hepta-ui-release-verifier-v1) set -- release "$@" ;;
  hepta-ui-device-lab-verifier-v1) set -- device "$@" ;;
  hepta-ui-accessibility-verifier-v1) set -- accessibility "$@" ;;
  hepta-ui-matrix-live-verifier-v1) set -- matrix "$@" ;;
  hepta-ui-bridge-live-verifier-v1) set -- bridge "$@" ;;
esac
exec /usr/bin/ruby --disable-gems - "$@" <<'RUBY'
require "json"
require "digest"
require "openssl"
require "pathname"

class VError < StandardError; end
class Usage < StandardError; end
class StrictObject < Hash
  def []=(key, value)
    raise VError, "duplicate JSON key" if key?(key)
    super
  end
end

HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A[0-9a-f]{64}\z/
UUID = /\A[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}\z/
CLOCK_SKEW_MS = 300_000
MANIFEST_KIND = "hepta-ui-evidence-manifest-v1"
SOURCE_KEYS = %w[schema_version kind head head_tree source_fingerprint worktree_clean repository_worktree_clean].freeze
REDACTION_KEYS = %w[credentials_redacted secrets_included tokens_included passwords_included raw_payloads_included].freeze
FORBIDDEN_EVIDENCE_KEYS = %w[accesstoken refreshtoken authorization password passwd secret clientsecret privatekey recoverykey cookie rawpayload rawresponse credentialvalue sessiontoken].freeze
FORBIDDEN_EVIDENCE_TEXT = /(?:Bearer\s+[A-Za-z0-9._~+\/-]{8,}|\bsyt_[A-Za-z0-9._~-]{8,})/i

SPECS = {
  "release" => {
    stem:"release", input:"hepta-ui-release-attestation-v1", output:"hepta-ui-release-receipt-v1",
    evidence:"macos_dmg", capability:"public_distribution_ready", max_age_ms:2_592_000_000,
    roles:{"macos_dmg"=>"application/x-apple-diskimage"},
    yes:%w[developer_id_signed signature_valid hardened_runtime sealed_resources_valid notarized stapled stapler_valid gatekeeper_accepted public_distribution_authorized],
    no:%w[public_upload_performed], fixed:{"notary_status"=>"Accepted"}, hashes:[], positive:[], zero:[],
    nonempty:[], dynamic:%w[signing_identity team_identifier], uuids:%w[notarization_submission_id]
  },
  "device" => {
    stem:"device-lab", input:"hepta-ui-device-lab-attestation-v1", output:"hepta-ui-device-lab-receipt-v1",
    evidence:"device_lab_evidence_bundle", capability:"real_device_lab_ready", max_age_ms:604_800_000,
    roles:{"ios_device_audit"=>"application/json", "android_device_audit"=>"application/json"},
    yes:%w[ios_real_device android_real_device app_install_verified cold_launch_verified foreground_verified authenticated_workflow_verified background_resume_verified rotation_verified software_keyboard_verified safe_area_or_insets_verified rtl_verified text_scale_verified performance_budget_verified secure_credential_storage_verified crash_free],
    no:%w[simulators_or_emulators], fixed:{}, hashes:%w[ios_device_identifier_sha256 android_device_identifier_sha256],
    positive:[], zero:[], nonempty:%w[ios_model ios_os_version android_model android_os_version], dynamic:[], uuids:[]
  },
  "accessibility" => {
    stem:"accessibility", input:"hepta-ui-accessibility-attestation-v1", output:"hepta-ui-accessibility-receipt-v1",
    evidence:"accessibility_evidence_bundle", capability:"accessibility_ready", max_age_ms:604_800_000,
    roles:{"voiceover_audit"=>"application/json", "talkback_audit"=>"application/json"},
    yes:%w[voiceover_real_device talkback_real_device services_enabled_during_audit settings_baseline_captured settings_restored focus_order_verified all_actionable_controls_reachable roles_states_values_verified dynamic_updates_announced modal_focus_contained no_focus_trap text_scaling_verified rtl_verified contrast_verified reduced_motion_verified],
    no:[], fixed:{"voiceover_service"=>"VoiceOver", "talkback_service"=>"TalkBack"},
    hashes:%w[voiceover_device_identifier_sha256 talkback_device_identifier_sha256],
    positive:%w[semantic_node_count actionable_control_count labeled_actionable_control_count],
    zero:%w[unlabeled_actionable_control_count duplicate_actionable_label_count blocking_issue_count], nonempty:[], dynamic:[], uuids:[]
  },
  "matrix" => {
    stem:"matrix-live", input:"hepta-ui-matrix-live-attestation-v1", output:"hepta-ui-matrix-live-receipt-v1",
    evidence:"matrix_live_evidence_bundle", capability:"matrix_live_ready", max_age_ms:900_000,
    roles:{"matrix_workflow_audit"=>"application/json"},
    yes:%w[real_homeserver authenticated_session login_success room_list_loaded timeline_loaded encrypted_room_verified message_send_roundtrip_verified logout_verified credentials_redacted fixture_or_mock_absent],
    no:%w[credentials_embedded synthetic_server mutation_outside_test_room], fixed:{"protocol"=>"matrix-client-server-api"},
    hashes:%w[homeserver_origin_sha256 session_identifier_sha256 test_room_identifier_sha256],
    positive:%w[timeline_event_count login_observed_unix_ms authenticated_workflow_observed_unix_ms logout_observed_unix_ms],
    zero:[], nonempty:[], dynamic:[], uuids:[]
  },
  "bridge" => {
    stem:"bridge-live", input:"hepta-ui-bridge-live-attestation-v1", output:"hepta-ui-bridge-live-receipt-v1",
    evidence:"hepta_bridge_live_evidence_bundle", capability:"hepta_live_bridge_ready", max_age_ms:900_000,
    roles:{"bridge_get_audit"=>"application/json"},
    yes:%w[canonical_loopback_endpoint exact_get_request http_status_200 response_deserialized matrix_session_authenticated explicit_user_opt_in fixture_or_mock_absent run_match session_match correlation_match sequence_match authoritative_origin_valid redaction_valid provenance_valid raw_source_payload_rejected logout_transport_dropped login_failure_transport_dropped],
    no:%w[subscribe prepare confirm reject cancel provider_invocation channel_delivery cursor_write gateway_mutation external_mutation],
    fixed:{"platform"=>"macos", "surface"=>"authenticated_post_login", "endpoint"=>"/api/hepta-native-bridge/v1/snapshot", "method"=>"GET", "content_type"=>"application/json"},
    hashes:%w[request_descriptor_sha256 response_sha256 transport_run_identifier_sha256 session_identifier_sha256 correlation_identifier_sha256 matrix_attestation_sha256],
    positive:%w[request_expected_sequence response_sequence response_byte_count bridge_get_observed_unix_ms], zero:%w[mutation_capability_count],
    nonempty:[], dynamic:[], uuids:[]
  }
}.freeze

def check(condition, message)
  raise VError, message unless condition
end

def keys(value, expected, label)
  check(value.is_a?(Hash) && value.keys.sort == expected.sort, "#{label} field set invalid")
end

def identity(stat)
  [stat.dev, stat.ino, stat.mode, stat.size, stat.mtime.to_i, stat.mtime.nsec, stat.ctime.to_i, stat.ctime.nsec]
end

def read_once(path, label, keep, limit)
  before = File.lstat(path)
  check(before.file? && !before.symlink?, "#{label} not a regular file")
  check(!limit || before.size <= limit, "#{label} too large")
  digest = Digest::SHA256.new
  data = keep ? String.new.b : nil
  total = 0
  File.open(path, File::RDONLY | File::NOFOLLOW) do |io|
    check(identity(io.stat) == identity(before), "#{label} changed before read")
    while (chunk = io.read(1_048_576))
      total += chunk.bytesize
      check(!limit || total <= limit, "#{label} too large")
      digest.update(chunk)
      data << chunk if keep
    end
    check(identity(io.stat) == identity(before), "#{label} changed during read")
  end
  check(identity(File.lstat(path)) == identity(before), "#{label} changed after read")
  {sha:digest.hexdigest, bytes:total, data:data, id:[before.dev, before.ino], identity:identity(before)}
rescue SystemCallError
  raise VError, "#{label} unreadable or unsafe"
end

def snapshot(path, label, keep:false, limit:nil)
  check(path.is_a?(String) && path.start_with?("/") && !path.match?(/[[:cntrl:]]/), "#{label} path not explicit absolute")
  check(Pathname.new(path).cleanpath.to_s == path && File.realpath(path) == path, "#{label} path not canonical or contains symlink")
  first = read_once(path, label, keep, limit)
  second = read_once(path, label, keep, limit)
  check(first.values_at(:sha, :bytes, :id, :identity) == second.values_at(:sha, :bytes, :id, :identity), "#{label} unstable")
  second.merge(path:path)
rescue SystemCallError
  raise VError, "#{label} path missing"
end

def evidence_redaction_safe?(value)
  case value
  when Hash
    value.all? do |key, child|
      !FORBIDDEN_EVIDENCE_KEYS.include?(key.to_s.downcase.gsub(/[^a-z0-9]/, "")) && evidence_redaction_safe?(child)
    end
  when Array
    value.all? { |child| evidence_redaction_safe?(child) }
  when String
    !value.match?(FORBIDDEN_EVIDENCE_TEXT)
  else
    true
  end
end

def parse_json(data, label)
  value = JSON.parse(data, object_class:StrictObject, array_class:Array, create_additions:false)
  check(value.is_a?(Hash), "#{label} root invalid")
  value
rescue JSON::ParserError => error
  raise VError, "#{label} malformed JSON: #{error.message}"
end

def safe_text(value)
  value.is_a?(String) && !value.empty? && value.bytesize <= 256 && !value.match?(/[[:cntrl:]]/)
end

def validate_source(source, expected, label)
  keys(source, SOURCE_KEYS, label)
  check(source["schema_version"] == 1 && source["kind"] == "hepta-ui-source-binding", "#{label} schema invalid")
  check(source["head"] == expected.fetch(:head) && source["head_tree"] == expected.fetch(:tree) && source["source_fingerprint"] == expected.fetch(:fingerprint), "#{label} tuple invalid")
  check(source["worktree_clean"] == true && source["repository_worktree_clean"] == true, "#{label} dirty")
  source
end

def validate_manifest(file, spec, producer, source, run_id, label, reserved_ids)
  manifest = parse_json(file[:data], label)
  keys(manifest, %w[schema_version kind profile evidence_kind producer source_binding run_identifier_sha256 entries redaction], label)
  check(manifest["schema_version"] == 1 && manifest["kind"] == MANIFEST_KIND, "#{label} schema invalid")
  check(manifest["profile"] == spec.fetch(:stem) && manifest["evidence_kind"] == spec.fetch(:evidence), "#{label} profile invalid")
  check(manifest["producer"] == producer && manifest["run_identifier_sha256"] == run_id, "#{label} producer/run binding invalid")
  check(manifest["source_binding"] == source, "#{label} source binding invalid")
  entries = manifest["entries"]
  check(entries.is_a?(Array) && entries.length == spec.fetch(:roles).length, "#{label} evidence entries invalid")
  seen_roles = {}; seen_hashes = {}; seen_identities = reserved_ids.dup; evidence_files = []
  bundle_root = File.dirname(file.fetch(:path))
  entries.each_with_index do |entry, index|
    item_label = "#{label}.entries[#{index}]"
    keys(entry, %w[role path sha256 size_bytes media_type], item_label)
    role = entry["role"]
    check(spec.fetch(:roles).key?(role) && !seen_roles[role], "#{item_label} role invalid")
    check(entry["media_type"] == spec.fetch(:roles).fetch(role), "#{item_label} media type invalid")
    check(entry["sha256"].is_a?(String) && entry["sha256"].match?(HEX64) && !seen_hashes[entry["sha256"]], "#{item_label} digest invalid")
    check(entry["size_bytes"].is_a?(Integer) && entry["size_bytes"] > 0 && entry["size_bytes"] <= 53_687_091_200, "#{item_label} size invalid")
    path = entry["path"]
    check(path.is_a?(String) && path.start_with?(bundle_root + File::SEPARATOR), "#{item_label} outside evidence bundle")
    evidence = snapshot(path, "#{item_label} evidence", keep:entry["media_type"] == "application/json", limit:53_687_091_200)
    check(!seen_identities.include?(evidence[:id]), "#{item_label} identity overlaps another input")
    check(evidence[:sha] == entry["sha256"] && evidence[:bytes] == entry["size_bytes"], "#{item_label} bytes do not match manifest")
    redaction_safe = true
    if entry["media_type"] == "application/json"
      parsed = JSON.parse(evidence[:data], object_class:StrictObject, array_class:Array, create_additions:false)
      check(parsed.is_a?(Hash), "#{item_label} JSON root invalid")
      redaction_safe = evidence_redaction_safe?(parsed)
      check(redaction_safe, "#{item_label} contains unredacted secret material")
    end
    evidence_files << evidence.merge(role:role, media_type:entry["media_type"], redaction_safe:redaction_safe)
    seen_roles[role] = true
    seen_hashes[entry["sha256"]] = true
    seen_identities << evidence[:id]
  end
  check(seen_roles.keys.sort == spec.fetch(:roles).keys.sort, "#{label} required evidence roles missing")
  redaction = manifest["redaction"]
  keys(redaction, REDACTION_KEYS, "#{label}.redaction")
  check(redaction == {
    "credentials_redacted"=>true,
    "secrets_included"=>false,
    "tokens_included"=>false,
    "passwords_included"=>false,
    "raw_payloads_included"=>false
  }, "#{label} redaction invalid")
  {manifest:manifest, evidence_files:evidence_files}
rescue JSON::ParserError => error
  raise VError, "#{label} evidence JSON malformed: #{error.message}"
end

def rehash_verified_receipt(path, profile, spec)
  receipt_file = snapshot(path, "#{profile} verifier receipt", keep:true, limit:1_048_576)
  receipt = parse_json(receipt_file[:data], "#{profile} verifier receipt")
  expected_producer = "scripts/hepta-ui-#{spec.fetch(:stem)}-verifier-v1"
  check(receipt["schema_version"] == 1 && receipt["kind"] == spec.fetch(:output) && receipt["producer"] == expected_producer, "#{profile} verifier receipt identity invalid")
  check(receipt["status"] == "ready" && receipt["independent_promotion_verifier_ready"] == true, "#{profile} verifier receipt not ready")
  artifact = receipt["artifact"]
  keys(artifact, %w[path sha256 size_bytes kind evidence_kind manifest_valid entry_digests], "#{profile} verifier artifact")
  check(artifact["kind"] == MANIFEST_KIND && artifact["evidence_kind"] == spec.fetch(:evidence) && artifact["manifest_valid"] == true, "#{profile} verifier artifact invalid")
  manifest_file = snapshot(artifact["path"], "#{profile} evidence manifest", limit:1_048_576)
  check(manifest_file[:sha] == artifact["sha256"] && manifest_file[:bytes] == artifact["size_bytes"], "#{profile} evidence manifest bytes changed")
  entries = artifact["entry_digests"]
  check(entries.is_a?(Array) && entries.length == spec.fetch(:roles).length, "#{profile} evidence digest count invalid")
  bundle_root = File.dirname(manifest_file.fetch(:path)); seen_roles = {}; seen_hashes = {}; canonical = []
  entries.each_with_index do |entry, index|
    label = "#{profile} evidence digest[#{index}]"
    keys(entry, %w[role path sha256 size_bytes media_type content_verified redaction_scan_passed], label)
    role = entry["role"]
    check(spec.fetch(:roles).key?(role) && !seen_roles[role], "#{label} role invalid")
    check(entry["media_type"] == spec.fetch(:roles).fetch(role) && entry["content_verified"] == true && entry["redaction_scan_passed"] == true, "#{label} verification claim invalid")
    check(entry["sha256"].is_a?(String) && entry["sha256"].match?(HEX64) && !seen_hashes[entry["sha256"]], "#{label} digest invalid")
    check(entry["size_bytes"].is_a?(Integer) && entry["size_bytes"] > 0, "#{label} size invalid")
    check(entry["path"].is_a?(String) && entry["path"].start_with?(bundle_root + File::SEPARATOR), "#{label} outside evidence bundle")
    evidence = snapshot(entry["path"], "#{label} bytes", limit:53_687_091_200)
    check(evidence[:sha] == entry["sha256"] && evidence[:bytes] == entry["size_bytes"], "#{label} exact bytes changed")
    canonical << [role, entry["path"], evidence[:sha], evidence[:bytes], entry["media_type"]]
    seen_roles[role] = true; seen_hashes[entry["sha256"]] = true
  end
  check(seen_roles.keys.sort == spec.fetch(:roles).keys.sort, "#{profile} evidence roles missing")
  {
    "schema_version"=>1, "kind"=>"hepta-ui-promotion-evidence-rehash-v1", "status"=>"ready", "profile"=>profile,
    "receipt_sha256"=>receipt_file[:sha], "manifest_sha256"=>manifest_file[:sha], "entry_count"=>canonical.length,
    "entry_set_sha256"=>Digest::SHA256.hexdigest(JSON.generate(canonical.sort_by(&:first))), "nofollow_exact_bytes_verified"=>true
  }
end

def verify_detached_signature(receipt_file, signature_file, public_key_file, expected_key_sha, label)
  check(public_key_file[:sha] == expected_key_sha, "#{label} trusted public key hash invalid")
  check(!public_key_file[:data].include?("PRIVATE KEY"), "#{label} trusted key contains private material")
  key = OpenSSL::PKey::RSA.new(public_key_file[:data])
  check(key.public? && !key.private? && key.n.num_bits >= 2048, "#{label} trusted RSA public key invalid")
  check(key.verify(OpenSSL::Digest::SHA256.new, signature_file[:data], receipt_file[:data]), "#{label} detached signature invalid")
rescue OpenSSL::PKey::PKeyError, OpenSSL::PKey::RSAError, OpenSSL::OpenSSLError
  raise VError, "#{label} trusted RSA public key or detached signature invalid"
end

def validate_domain(spec, domain, profile, release_identity, release_team)
  expected = spec.values_at(:yes, :no, :hashes, :positive, :zero, :nonempty, :dynamic, :uuids).flatten + spec.fetch(:fixed).keys
  keys(domain, expected, "checks")
  spec.fetch(:yes).each { |key| check(domain[key] == true, "#{key} not true") }
  spec.fetch(:no).each { |key| check(domain[key] == false, "#{key} not false") }
  spec.fetch(:fixed).each { |key, value| check(domain[key] == value, "#{key} invalid") }
  spec.fetch(:hashes).each { |key| check(domain[key].is_a?(String) && domain[key].match?(HEX64), "#{key} invalid") }
  check(spec.fetch(:hashes).map { |key| domain[key] }.uniq.length == spec.fetch(:hashes).length, "hashed identifiers not unique")
  spec.fetch(:positive).each { |key| check(domain[key].is_a?(Integer) && domain[key] > 0, "#{key} invalid") }
  spec.fetch(:zero).each { |key| check(domain[key] == 0, "#{key} not zero") }
  spec.fetch(:nonempty).each { |key| check(safe_text(domain[key]), "#{key} invalid") }
  spec.fetch(:uuids).each { |key| check(domain[key].is_a?(String) && domain[key].match?(UUID), "#{key} invalid") }
  if profile == "release"
    check(domain["signing_identity"] == release_identity && domain["team_identifier"] == release_team, "signer binding invalid")
  end
  if profile == "accessibility"
    check(domain["labeled_actionable_control_count"] == domain["actionable_control_count"], "actionable labels incomplete")
  elsif profile == "matrix"
    check(domain["login_observed_unix_ms"] < domain["authenticated_workflow_observed_unix_ms"] && domain["authenticated_workflow_observed_unix_ms"] < domain["logout_observed_unix_ms"], "Matrix workflow time order invalid")
  elsif profile == "bridge"
    check(domain["request_expected_sequence"] == domain["response_sequence"], "bridge transport sequence mismatch")
  end
  domain
end

def option(opts, prefix, name)
  return opts.fetch(name) if prefix.empty?
  mapped = name.start_with?("expected-") ? "expected-#{prefix}#{name.delete_prefix("expected-")}" : "#{prefix}#{name}"
  opts.fetch(mapped)
end

def validate_attestation(profile, spec, opts, source_expected, now_ms, prefix:"")
  label = prefix.empty? ? profile : prefix.delete_suffix("-")
  producer = option(opts, prefix, "expected-producer")
  check(safe_text(producer), "#{label} expected producer invalid")
  expected_key_sha = option(opts, prefix, "expected-public-key-sha256")
  check(expected_key_sha.match?(HEX64), "#{label} trusted public key hash invalid")

  receipt_file = snapshot(option(opts, prefix, "receipt"), "#{label} attestation", keep:true, limit:1_048_576)
  artifact_file = snapshot(option(opts, prefix, "artifact"), "#{label} evidence manifest", keep:true, limit:1_048_576)
  signature_file = snapshot(option(opts, prefix, "signature"), "#{label} detached signature", keep:true, limit:16_384)
  public_key_file = snapshot(option(opts, prefix, "trusted-public-key"), "#{label} trusted public key", keep:true, limit:65_536)
  identities = [receipt_file, artifact_file, signature_file, public_key_file].map { |file| file[:id] }
  check(identities.uniq.length == identities.length, "#{label} input identities overlap")
  check(receipt_file[:bytes] > 0 && artifact_file[:bytes] > 0 && signature_file[:bytes] > 0 && public_key_file[:bytes] > 0, "#{label} input empty")
  verify_detached_signature(receipt_file, signature_file, public_key_file, expected_key_sha, label)

  receipt = parse_json(receipt_file[:data], "#{label} attestation")
  keys(receipt, %w[schema_version kind producer status source_binding source_stable_during_run attested_at_unix_ms expires_at_unix_ms run_identifier_sha256 artifact checks], "#{label} attestation")
  check(receipt["schema_version"] == 1 && receipt["kind"] == spec.fetch(:input) && receipt["producer"] == producer && receipt["status"] == "ready" && receipt["source_stable_during_run"] == true, "#{label} schema/producer/status invalid")
  source = validate_source(receipt["source_binding"], source_expected, "#{label} source_binding")
  run_id = receipt["run_identifier_sha256"]
  check(run_id.is_a?(String) && run_id.match?(HEX64), "#{label} run identifier invalid")
  attested_at = receipt["attested_at_unix_ms"]
  expires_at = receipt["expires_at_unix_ms"]
  check(attested_at.is_a?(Integer) && expires_at.is_a?(Integer), "#{label} timestamps invalid")
  check(attested_at <= now_ms + CLOCK_SKEW_MS && attested_at >= now_ms - spec.fetch(:max_age_ms), "#{label} attestation not fresh")
  check(expires_at > now_ms && expires_at > attested_at && expires_at - attested_at <= spec.fetch(:max_age_ms), "#{label} expiry invalid")

  artifact = receipt["artifact"]
  keys(artifact, %w[kind evidence_kind path sha256 size_bytes], "#{label} artifact")
  check(artifact["kind"] == MANIFEST_KIND && artifact["evidence_kind"] == spec.fetch(:evidence), "#{label} artifact kind invalid")
  check(artifact["path"] == option(opts, prefix, "artifact") && artifact["sha256"] == artifact_file[:sha] && artifact["size_bytes"] == artifact_file[:bytes], "#{label} artifact binding invalid")
  check(artifact["sha256"].is_a?(String) && artifact["sha256"].match?(HEX64), "#{label} artifact digest invalid")
  manifest_validation = validate_manifest(artifact_file, spec, producer, source, run_id, "#{label} evidence manifest", identities)
  domain = validate_domain(spec, receipt["checks"], profile, opts["expected-signing-identity"], opts["expected-team-id"])
  if profile == "matrix"
    check(domain["logout_observed_unix_ms"] <= attested_at, "Matrix attestation predates workflow")
  elsif profile == "bridge"
    check(domain["bridge_get_observed_unix_ms"] <= attested_at, "bridge attestation predates GET")
    check(domain["transport_run_identifier_sha256"] == run_id, "bridge transport run identifier mismatch")
  end

  {
    receipt:receipt, receipt_file:receipt_file, artifact_file:artifact_file, signature_file:signature_file,
    public_key_file:public_key_file, manifest:manifest_validation.fetch(:manifest), evidence_files:manifest_validation.fetch(:evidence_files), source:source, run_id:run_id, domain:domain,
    attested_at:attested_at, expires_at:expires_at, producer:producer, expected_key_sha:expected_key_sha
  }
end

begin
  command = ARGV.shift
  if command == "rehash"
    profile = ARGV.shift
    spec = SPECS[profile] or raise Usage, "rehash profile must be release, device, accessibility, matrix, or bridge"
    raise Usage, "rehash requires --receipt ABS" unless ARGV.length == 2 && ARGV[0] == "--receipt"
    puts JSON.generate(rehash_verified_receipt(ARGV[1], profile, spec))
    exit
  end
  profile = command
  spec = SPECS[profile] or raise Usage, "profile must be release, device, accessibility, matrix, or bridge"
  base = %w[receipt artifact signature trusted-public-key expected-public-key-sha256 expected-producer trust-policy-sha256 source-head source-tree source-fingerprint]
  allowed = base.dup
  allowed += %w[expected-signing-identity expected-team-id] if profile == "release"
  if profile == "bridge"
    allowed += %w[matrix-receipt matrix-artifact matrix-signature matrix-trusted-public-key expected-matrix-public-key-sha256 expected-matrix-producer]
  end
  if ARGV == ["--help"]
    puts "usage: verifier #{allowed.map { |name| "--#{name} VALUE" }.join(" ")}"
    puts "Read-only and fail-closed: verifies strict JSON manifests and RSA-SHA256 detached attestations; performs no signing, upload, network, device connection, or settings changes."
    exit
  end
  opts = {}
  until ARGV.empty?
    flag = ARGV.shift
    raise Usage, "invalid option" unless flag&.start_with?("--")
    name = flag.delete_prefix("--")
    raise Usage, "unknown or duplicate option" unless allowed.include?(name) && !opts.key?(name)
    value = ARGV.shift
    raise Usage, "missing option value" unless value && !value.empty?
    opts[name] = value
  end
  check(opts.keys.sort == allowed.sort, "all inputs and trust anchors must be explicit")
  check(opts["source-head"].match?(HEX40) && opts["source-tree"].match?(HEX40) && opts["source-fingerprint"].match?(HEX64), "expected source tuple invalid")
  check(opts["trust-policy-sha256"].match?(HEX64), "trust policy blob hash invalid")
  if profile == "release"
    check(safe_text(opts["expected-signing-identity"]), "expected signing identity invalid")
    check(opts["expected-team-id"].match?(/\A[A-Z0-9]{10}\z/), "expected team ID invalid")
  end
  source_expected = {head:opts["source-head"], tree:opts["source-tree"], fingerprint:opts["source-fingerprint"]}
  now_ms = (Time.now.to_r * 1000).to_i
  verified = validate_attestation(profile, spec, opts, source_expected, now_ms)

  parent = nil
  if profile == "bridge"
    parent = validate_attestation("matrix", SPECS.fetch("matrix"), opts, source_expected, now_ms, prefix:"matrix-")
    domain = verified.fetch(:domain)
    parent_domain = parent.fetch(:domain)
    check(domain["matrix_attestation_sha256"] == parent.fetch(:receipt_file).fetch(:sha), "bridge parent Matrix attestation hash mismatch")
    check(verified.fetch(:run_id) == parent.fetch(:run_id), "bridge/Matrix run identifier mismatch")
    check(domain["session_identifier_sha256"] == parent_domain["session_identifier_sha256"], "bridge/Matrix session mismatch")
    bridge_at = domain["bridge_get_observed_unix_ms"]
    check(parent_domain["login_observed_unix_ms"] < bridge_at && bridge_at < parent_domain["logout_observed_unix_ms"], "bridge GET outside authenticated Matrix interval")
    check(verified.fetch(:attested_at) >= bridge_at && verified.fetch(:attested_at) <= parent.fetch(:expires_at), "bridge attestation outside Matrix freshness window")
  end

  actions = {
    "signing_performed"=>false, "notarization_submission_performed"=>false,
    "artifact_upload_performed"=>false, "network_connection_performed"=>false,
    "real_device_contacted"=>false, "system_settings_changed"=>false
  }
  output = {
    "schema_version"=>1, "kind"=>spec.fetch(:output),
    "producer"=>"scripts/hepta-ui-#{spec.fetch(:stem)}-verifier-v1", "status"=>"ready",
    "source_binding"=>verified.fetch(:source), "source_stable_during_run"=>true,
    "trust_policy"=>{"sha256"=>opts.fetch("trust-policy-sha256"), "exact_head_blob_required"=>true},
    "independent_promotion_verifier_ready"=>true,
    "run_identifier_sha256"=>verified.fetch(:run_id),
    "temporal_binding"=>{
      "attested_at_unix_ms"=>verified.fetch(:attested_at), "expires_at_unix_ms"=>verified.fetch(:expires_at),
      "verified_at_unix_ms"=>now_ms, "freshness_verified"=>true
    },
    "input_receipt"=>{
      "path"=>opts["receipt"], "sha256"=>verified.fetch(:receipt_file).fetch(:sha), "producer"=>verified.fetch(:producer)
    },
    "attestation_signature"=>{
      "algorithm"=>"RSA-SHA256", "path"=>opts["signature"],
      "sha256"=>verified.fetch(:signature_file).fetch(:sha), "size_bytes"=>verified.fetch(:signature_file).fetch(:bytes),
      "trusted_public_key_path"=>opts["trusted-public-key"],
      "trusted_public_key_sha256"=>verified.fetch(:public_key_file).fetch(:sha),
      "trusted_public_key_size_bytes"=>verified.fetch(:public_key_file).fetch(:bytes),
      "expected_producer"=>verified.fetch(:producer), "signature_verified"=>true
    },
    "artifact"=>{
      "path"=>opts["artifact"], "sha256"=>verified.fetch(:artifact_file).fetch(:sha),
      "size_bytes"=>verified.fetch(:artifact_file).fetch(:bytes), "kind"=>MANIFEST_KIND,
      "evidence_kind"=>spec.fetch(:evidence), "manifest_valid"=>true,
      "entry_digests"=>verified.fetch(:evidence_files).map { |entry| {"role"=>entry[:role], "path"=>entry[:path], "sha256"=>entry[:sha], "size_bytes"=>entry[:bytes], "media_type"=>entry[:media_type], "content_verified"=>true, "redaction_scan_passed"=>entry[:redaction_safe]} }
    },
    "verified_checks"=>verified.fetch(:domain), "verifier_actions"=>actions,
    spec.fetch(:capability)=>true
  }
  if profile == "matrix"
    output["live_chain_binding"] = {
      "run_identifier_sha256"=>verified.fetch(:run_id),
      "session_identifier_sha256"=>verified.fetch(:domain).fetch("session_identifier_sha256"),
      "login_observed_unix_ms"=>verified.fetch(:domain).fetch("login_observed_unix_ms"),
      "authenticated_workflow_observed_unix_ms"=>verified.fetch(:domain).fetch("authenticated_workflow_observed_unix_ms"),
      "logout_observed_unix_ms"=>verified.fetch(:domain).fetch("logout_observed_unix_ms"),
      "sequence_verified"=>true
    }
  elsif profile == "bridge"
    output["live_chain_binding"] = {
      "run_identifier_sha256"=>verified.fetch(:run_id),
      "transport_run_identifier_sha256"=>verified.fetch(:domain).fetch("transport_run_identifier_sha256"),
      "session_identifier_sha256"=>verified.fetch(:domain).fetch("session_identifier_sha256"),
      "request_expected_sequence"=>verified.fetch(:domain).fetch("request_expected_sequence"),
      "response_sequence"=>verified.fetch(:domain).fetch("response_sequence"),
      "bridge_get_observed_unix_ms"=>verified.fetch(:domain).fetch("bridge_get_observed_unix_ms"),
      "matrix_attestation_path"=>opts["matrix-receipt"],
      "matrix_attestation_sha256"=>parent.fetch(:receipt_file).fetch(:sha),
      "matrix_signature_sha256"=>parent.fetch(:signature_file).fetch(:sha),
      "matrix_trusted_public_key_sha256"=>parent.fetch(:public_key_file).fetch(:sha),
      "matrix_evidence_manifest_sha256"=>parent.fetch(:artifact_file).fetch(:sha),
      "matrix_producer"=>parent.fetch(:producer), "parent_signature_verified"=>true,
      "session_match_verified"=>true, "run_match_verified"=>true, "sequence_verified"=>true
    }
  end
  output.merge!({"signed"=>true, "notarized"=>true, "stapled"=>true}) if profile == "release"
  puts JSON.generate(output)
rescue Usage => error
  warn "promotion verifier usage: #{error.message}"
  exit 64
rescue VError => error
  warn "promotion verifier rejected #{profile || "unknown"}: #{error.message}"
  exit 1
rescue StandardError
  warn "promotion verifier rejected input: internal fail-closed error"
  exit 1
end
RUBY

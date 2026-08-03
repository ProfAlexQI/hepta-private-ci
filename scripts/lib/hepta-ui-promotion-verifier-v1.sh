#!/bin/bash -p
set +x
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
PATH="/usr/bin:/bin:/usr/sbin:/sbin"; export PATH
[[ -x /usr/bin/ruby && ! -L /usr/bin/ruby ]] || { echo "promotion verifier: canonical Ruby unavailable" >&2; exit 2; }
exec /usr/bin/ruby - "$@" <<'RUBY'
require "json"; require "digest"; require "pathname"
class VError < StandardError; end
class Usage < StandardError; end
class StrictObject < Hash
  def []=(key, value); raise VError, "duplicate JSON key" if key?(key); super; end
end
SPECS = {
  "release" => {input:"hepta-ui-release-attestation-v1", output:"hepta-ui-release-receipt-v1", artifact:"macos_dmg", capability:"public_distribution_ready",
    yes:%w[developer_id_signed signature_valid hardened_runtime sealed_resources_valid notarized stapled stapler_valid gatekeeper_accepted public_distribution_authorized], no:%w[public_upload_performed], fixed:{"notary_status"=>"Accepted"}, hashes:[], positive:[], zero:[], nonempty:[], dynamic:%w[signing_identity team_identifier], uuids:%w[notarization_submission_id]},
  "device" => {input:"hepta-ui-device-lab-attestation-v1", output:"hepta-ui-device-lab-receipt-v1", artifact:"device_lab_evidence_bundle", capability:"real_device_lab_ready",
    yes:%w[ios_real_device android_real_device app_install_verified cold_launch_verified foreground_verified authenticated_workflow_verified background_resume_verified rotation_verified software_keyboard_verified safe_area_or_insets_verified rtl_verified text_scale_verified performance_budget_verified secure_credential_storage_verified crash_free], no:%w[simulators_or_emulators], fixed:{}, hashes:%w[ios_device_identifier_sha256 android_device_identifier_sha256], positive:[], zero:[], nonempty:%w[ios_model ios_os_version android_model android_os_version], dynamic:[], uuids:[]},
  "accessibility" => {input:"hepta-ui-accessibility-attestation-v1", output:"hepta-ui-accessibility-receipt-v1", artifact:"accessibility_evidence_bundle", capability:"accessibility_ready",
    yes:%w[voiceover_real_device talkback_real_device services_enabled_during_audit settings_baseline_captured settings_restored focus_order_verified all_actionable_controls_reachable roles_states_values_verified dynamic_updates_announced modal_focus_contained no_focus_trap text_scaling_verified rtl_verified contrast_verified reduced_motion_verified], no:[], fixed:{"voiceover_service"=>"VoiceOver","talkback_service"=>"TalkBack"}, hashes:%w[voiceover_device_identifier_sha256 talkback_device_identifier_sha256], positive:%w[semantic_node_count actionable_control_count labeled_actionable_control_count], zero:%w[unlabeled_actionable_control_count duplicate_actionable_label_count blocking_issue_count], nonempty:[], dynamic:[], uuids:[]}
}.freeze
def check(condition, message); raise VError, message unless condition; end
def keys(value, expected, label); check(value.is_a?(Hash) && value.keys.sort == expected.sort, "#{label} field set invalid"); end
def identity(stat); [stat.dev,stat.ino,stat.mode,stat.size,stat.mtime.to_i,stat.mtime.nsec,stat.ctime.to_i,stat.ctime.nsec]; end
def read_once(path, label, keep, limit)
  before=File.lstat(path); check(before.file? && !before.symlink?, "#{label} not a regular file"); check(!limit || before.size<=limit, "#{label} too large")
  digest=Digest::SHA256.new; data=keep ? String.new.b : nil; total=0
  File.open(path, File::RDONLY|File::NOFOLLOW) do |io|
    check(identity(io.stat)==identity(before), "#{label} changed before read")
    while chunk=io.read(1_048_576); total+=chunk.bytesize; check(!limit || total<=limit, "#{label} too large"); digest.update(chunk); data<<chunk if keep; end
    check(identity(io.stat)==identity(before), "#{label} changed during read")
  end
  check(identity(File.lstat(path))==identity(before), "#{label} changed after read")
  {sha:digest.hexdigest,bytes:total,data:data,id:[before.dev,before.ino],identity:identity(before)}
rescue SystemCallError
  raise VError, "#{label} unreadable or unsafe"
end
def snapshot(path, label, keep=false, limit=nil)
  check(path.is_a?(String) && path.start_with?("/") && !path.match?(/[[:cntrl:]]/), "#{label} path not explicit absolute")
  check(Pathname.new(path).cleanpath.to_s==path && File.realpath(path)==path, "#{label} path not canonical or contains symlink")
  first=read_once(path,label,keep,limit); second=read_once(path,label,keep,limit)
  check(first.values_at(:sha,:bytes,:id,:identity)==second.values_at(:sha,:bytes,:id,:identity), "#{label} unstable"); second
rescue SystemCallError
  raise VError, "#{label} path missing"
end
def safe_text(value); value.is_a?(String) && !value.empty? && value.bytesize<=256 && !value.match?(/[[:cntrl:]]/); end
begin
  profile=ARGV.shift; spec=SPECS[profile] or raise Usage, "profile must be release, device, or accessibility"
  if ARGV==["--help"]
    puts "usage: verifier --receipt ABS --artifact ABS --expected-producer ID --source-head HEX40 --source-tree HEX40 --source-fingerprint HEX64#{profile=="release" ? " --expected-signing-identity ID --expected-team-id TEAMID" : ""}"
    puts "Read-only and fail-closed: no signing, upload, network, device connection, or settings changes."
    exit
  end
  allowed=%w[receipt artifact expected-producer source-head source-tree source-fingerprint]; allowed+=%w[expected-signing-identity expected-team-id] if profile=="release"
  opts={}; until ARGV.empty?; flag=ARGV.shift; raise Usage, "invalid option" unless flag&.start_with?("--"); name=flag.delete_prefix("--"); raise Usage, "unknown or duplicate option" unless allowed.include?(name) && !opts.key?(name); value=ARGV.shift; raise Usage, "missing option value" unless value && !value.empty?; opts[name]=value; end
  check(opts.keys.sort==allowed.sort, "all inputs must be explicit")
  check(safe_text(opts["expected-producer"]), "expected producer invalid")
  check(opts["source-head"].match?(/\A[0-9a-f]{40}\z/) && opts["source-tree"].match?(/\A[0-9a-f]{40}\z/) && opts["source-fingerprint"].match?(/\A[0-9a-f]{64}\z/), "expected source tuple invalid")
  if profile=="release"; check(safe_text(opts["expected-signing-identity"]), "expected signing identity invalid"); check(opts["expected-team-id"].match?(/\A[A-Z0-9]{10}\z/), "expected team ID invalid"); end
  receipt_file=snapshot(opts["receipt"],"receipt",true,1_048_576); artifact_file=snapshot(opts["artifact"],"artifact"); check(receipt_file[:id]!=artifact_file[:id] && artifact_file[:bytes]>0, "receipt/artifact identity invalid")
  receipt=JSON.parse(receipt_file[:data], object_class:StrictObject, array_class:Array, create_additions:false)
  keys(receipt,%w[schema_version kind producer status source_binding source_stable_during_run artifact checks],"receipt")
  check(receipt["schema_version"]==1 && receipt["kind"]==spec[:input] && receipt["producer"]==opts["expected-producer"] && receipt["status"]=="ready" && receipt["source_stable_during_run"]==true,"schema/producer/status invalid")
  source=receipt["source_binding"]; keys(source,%w[schema_version kind head head_tree source_fingerprint worktree_clean repository_worktree_clean],"source_binding")
  check(source["schema_version"]==1 && source["kind"]=="hepta-ui-source-binding" && source["head"]==opts["source-head"] && source["head_tree"]==opts["source-tree"] && source["source_fingerprint"]==opts["source-fingerprint"] && source["worktree_clean"]==true && source["repository_worktree_clean"]==true,"source binding invalid")
  artifact=receipt["artifact"]; keys(artifact,%w[kind path sha256 size_bytes],"artifact")
  check(artifact["kind"]==spec[:artifact] && artifact["path"]==opts["artifact"] && artifact["sha256"].is_a?(String) && artifact["sha256"].match?(/\A[0-9a-f]{64}\z/) && artifact["sha256"]==artifact_file[:sha] && artifact["size_bytes"]==artifact_file[:bytes],"artifact binding invalid")
  domain=receipt["checks"]; expected=spec.values_at(:yes,:no,:hashes,:positive,:zero,:nonempty,:dynamic,:uuids).flatten+spec[:fixed].keys; keys(domain,expected,"checks")
  spec[:yes].each{|key| check(domain[key]==true,"#{key} not true")}; spec[:no].each{|key| check(domain[key]==false,"#{key} not false")}; spec[:fixed].each{|key,value| check(domain[key]==value,"#{key} invalid")}
  spec[:hashes].each{|key| check(domain[key].is_a?(String) && domain[key].match?(/\A[0-9a-f]{64}\z/),"#{key} invalid")}; check(spec[:hashes].map{|key| domain[key]}.uniq.length==spec[:hashes].length,"device identifiers not unique")
  spec[:positive].each{|key| check(domain[key].is_a?(Integer) && domain[key]>0,"#{key} invalid")}; spec[:zero].each{|key| check(domain[key]==0,"#{key} not zero")}; spec[:nonempty].each{|key| check(safe_text(domain[key]),"#{key} invalid")}; spec[:uuids].each{|key| check(domain[key].is_a?(String) && domain[key].match?(/\A[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}\z/),"#{key} invalid")}
  if profile=="release"; check(domain["signing_identity"]==opts["expected-signing-identity"] && domain["team_identifier"]==opts["expected-team-id"],"signer binding invalid"); end
  if profile=="accessibility"; check(domain["labeled_actionable_control_count"]==domain["actionable_control_count"],"actionable labels incomplete"); end
  actions={"signing_performed"=>false,"notarization_submission_performed"=>false,"artifact_upload_performed"=>false,"network_connection_performed"=>false,"real_device_contacted"=>false,"system_settings_changed"=>false}
  output={"schema_version"=>1,"kind"=>spec[:output],"producer"=>"scripts/hepta-ui-#{profile=="device" ? "device-lab" : profile}-verifier-v1","status"=>"ready","source_binding"=>source,"source_stable_during_run"=>true,"independent_promotion_verifier_ready"=>true,"input_receipt"=>{"path"=>opts["receipt"],"sha256"=>receipt_file[:sha],"producer"=>receipt["producer"]},"artifact"=>{"path"=>opts["artifact"],"sha256"=>artifact_file[:sha],"size_bytes"=>artifact_file[:bytes],"kind"=>artifact["kind"]},"verified_checks"=>domain,"verifier_actions"=>actions,spec[:capability]=>true}
  output.merge!({"signed"=>true,"notarized"=>true,"stapled"=>true}) if profile=="release"; puts JSON.generate(output)
rescue Usage => error
  warn "promotion verifier usage: #{error.message}"; exit 64
rescue VError, JSON::ParserError => error
  warn "promotion verifier rejected #{profile || "unknown"}: #{error.message}"; exit 1
rescue StandardError
  warn "promotion verifier rejected input: internal fail-closed error"; exit 1
end
RUBY

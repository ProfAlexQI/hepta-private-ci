hepta_mobile_cleanup_final_exit_code() {
  local original="$1" restore_ready="$2" readback_ready="$3"
  [[ "$original" =~ ^[0-9]+$ ]] && (( original <= 255 )) || return 64
  [[ "$restore_ready" == true || "$restore_ready" == false ]] || return 64
  [[ "$readback_ready" == true || "$readback_ready" == false ]] || return 64
  if [[ "$original" == 0 && ( "$restore_ready" != true || "$readback_ready" != true ) ]]; then
    printf '1\n'
  else
    printf '%s\n' "$original"
  fi
}

hepta_mobile_cleanup_failure_json() {
  local platform="$1" producer="$2" original="$3" restore_ready="$4" readback_ready="$5" final
  case "$platform" in ios_simulator|android_emulator) ;; *) return 64 ;; esac
  final="$(hepta_mobile_cleanup_final_exit_code "$original" "$restore_ready" "$readback_ready")" || return
  jq -n --arg platform "$platform" --arg producer "$producer" \
    --argjson original "$original" --argjson final "$final" \
    --argjson restore_ready "$restore_ready" --argjson readback_ready "$readback_ready" '
      {
        schema_version:1,kind:"hepta-native-mobile-lab-cleanup-failure-receipt",producer:$producer,
        platform:$platform,status:"not_ready",ready:false,original_exit_code:$original,final_exit_code:$final,
        cleanup:{attempted:true,restore_command_ready:$restore_ready,exact_readback_ready:$readback_ready,original_exit_code_preserved:($original != 0)},
        blockers:[
          if $restore_ready then empty else {code:($platform + "_state_restore_command_failed"),observed:false} end,
          if $readback_ready then empty else {code:($platform + "_state_restore_readback_mismatch"),observed:false} end
        ],
        claims:{state_restored:($restore_ready and $readback_ready)},
        local_device_state_mutation_performed:true,
        local_device_state_may_remain_mutated:(($restore_ready and $readback_ready) | not),
        external_side_effects_performed:false
      }
    '
}

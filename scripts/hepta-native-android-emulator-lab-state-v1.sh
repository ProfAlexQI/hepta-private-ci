hepta_android_restore_setting() {
  local adb="$1" serial="$2" namespace="$3" key="$4" value="$5" attempt readback
  if [[ "$value" == null ]]; then
    for attempt in 1 2 3; do
      "$adb" -s "$serial" shell settings delete "$namespace" "$key" >/dev/null || continue
      readback="$($adb -s "$serial" shell settings get "$namespace" "$key" | tr -d '\r')"
      [[ "$readback" == null ]] && return 0
    done
    return 1
  else
    "$adb" -s "$serial" shell settings put "$namespace" "$key" "$value" >/dev/null
    readback="$($adb -s "$serial" shell settings get "$namespace" "$key" | tr -d '\r')"
    [[ "$readback" == "$value" ]]
  fi
}

hepta_android_battery_state_json() {
  local adb="$1" serial="$2"
  "$adb" -s "$serial" shell dumpsys battery | tr -d '\r' | ruby -rjson -e '
    text = STDIN.read
    mapping = {
      "AC powered" => "ac", "USB powered" => "usb", "Wireless powered" => "wireless",
      "Dock powered" => "dock", "status" => "status", "health" => "health",
      "present" => "present", "level" => "level", "scale" => "scale",
      "temperature" => "temp", "Charge counter" => "counter"
    }
    state = {"updates_stopped" => text.include?("UPDATES STOPPED")}
    text.each_line do |line|
      key, value = line.strip.split(/:\s*/, 2)
      target = mapping[key]
      next unless target && value
      state[target] = value.match?(/\A-?\d+\z/) ? value.to_i : value == "true" ? true : value == "false" ? false : value
    end
    abort "incomplete battery state" unless (["updates_stopped"] + mapping.values).all? { |key| state.key?(key) }
    puts JSON.generate(state.sort.to_h)
  '
}

hepta_android_battery_restore_plan() {
  jq -er '
    if .updates_stopped == false then "reset"
    else error("frozen battery state is not exactly restorable")
    end
  ' <<<"$1"
}

hepta_android_restore_battery_state() {
  local adb="$1" serial="$2" state="$3" operation key value failed=false
  while IFS=$'\t' read -r operation key value; do
    if [[ "$operation" == reset ]]; then
      "$adb" -s "$serial" shell cmd battery reset </dev/null >/dev/null || failed=true
    else
      "$adb" -s "$serial" shell cmd battery set "$key" "$value" </dev/null >/dev/null || failed=true
    fi
  done < <(hepta_android_battery_restore_plan "$state")
  [[ "$failed" == false ]]
}

hepta_android_emulator_lab_state_snapshot() {
  local adb="$1" serial="$2"
  jq -n \
    --arg accelerometer "$($adb -s "$serial" shell settings get system accelerometer_rotation | tr -d '\r')" \
    --arg rotation "$($adb -s "$serial" shell settings get system user_rotation | tr -d '\r')" \
    --arg font "$($adb -s "$serial" shell settings get system font_scale | tr -d '\r')" \
    --arg rtl "$($adb -s "$serial" shell settings get global debug.force_rtl | tr -d '\r')" \
    --arg low_power "$($adb -s "$serial" shell settings get global low_power | tr -d '\r')" \
    --argjson battery "$(hepta_android_battery_state_json "$adb" "$serial")" \
    '{accelerometer_rotation:$accelerometer,user_rotation:$rotation,font_scale:$font,force_rtl:$rtl,low_power:$low_power,battery:$battery}'
}

hepta_android_emulator_lab_state_ready() {
  jq -e '
    (.accelerometer_rotation | test("^(null|0|1)$"))
    and (.user_rotation | test("^(null|[0-3])$"))
    and (.font_scale | test("^(null|[0-9]+([.][0-9]+)?)$"))
    and (.force_rtl | test("^(null|0|1)$"))
    and (.low_power | test("^[01]$"))
    and .battery.updates_stopped == false
    and (.battery | [.ac,.usb,.wireless,.dock,.present] | all(type == "boolean"))
    and (.battery | [.status,.health,.level,.scale,.temp,.counter] | all(type == "number"))
  ' >/dev/null <<<"$1"
}

hepta_android_emulator_lab_state_restore() {
  local adb="$1" serial="$2" state="$3" failed=false
  hepta_android_emulator_lab_state_ready "$state" || return 1
  hepta_android_restore_battery_state "$adb" "$serial" "$(jq -c '.battery' <<<"$state")" || failed=true
  "$adb" -s "$serial" shell cmd power set-mode "$(jq -r '.low_power' <<<"$state")" >/dev/null || failed=true
  hepta_android_restore_setting "$adb" "$serial" system font_scale "$(jq -r '.font_scale' <<<"$state")" || failed=true
  hepta_android_restore_setting "$adb" "$serial" global debug.force_rtl "$(jq -r '.force_rtl' <<<"$state")" || failed=true
  hepta_android_restore_setting "$adb" "$serial" system accelerometer_rotation "$(jq -r '.accelerometer_rotation' <<<"$state")" || failed=true
  hepta_android_restore_setting "$adb" "$serial" system user_rotation "$(jq -r '.user_rotation' <<<"$state")" || failed=true
  [[ "$failed" == false ]]
}

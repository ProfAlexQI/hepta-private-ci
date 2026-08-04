(async () => {
  const { spawn } = require("node:child_process");
  const fs = require("node:fs");
  const fsPromises = require("node:fs/promises");
  const os = require("node:os");
  const path = require("node:path");
  const { actualVisibilityFunctionSource } = require("./actual-visibility.cjs");
  const temperedGlassProbeSource = require("./density-probe/tempered-glass.fragment.cjs");

  const [chromeBin, baseUrl] = process.argv.slice(2);
  const viewports = [
    { name: "desktop", width: 1365, height: 900, visible: [".tg-conversation-rail", ".tg-thread-panel", ".tg-compose-wrap"], hidden: [".tg-room-panel"] },
    { name: "narrow", width: 768, height: 900, visible: [".tg-conversation-rail", ".tg-thread-panel", ".tg-compose-wrap"], hidden: [".tg-room-panel"] },
    { name: "mobile", width: 500, height: 844, visible: [".tg-thread-panel", ".tg-compose-wrap"], hidden: [".tg-conversation-rail", ".tg-room-panel"] },
    { name: "phone320", width: 320, height: 844, visible: [".tg-thread-panel", ".tg-compose-wrap"], hidden: [".tg-conversation-rail", ".tg-room-panel"] },
  ];

  const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

  async function waitFor(condition, timeoutMilliseconds, label) {
    const deadline = Date.now() + timeoutMilliseconds;
    while (Date.now() < deadline) {
      const value = condition();
      if (value) return value;
      await sleep(50);
    }
    throw new Error(`Timed out waiting for ${label}`);
  }

  async function inspectViewport(viewport) {
    const profileDirectory = await fsPromises.mkdtemp(path.join(os.tmpdir(), `hepta-control-density-${viewport.name}-`));
    const chrome = spawn(chromeBin, [
      "--headless=new",
      "--disable-gpu",
      "--force-device-scale-factor=1",
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--no-first-run",
      "--no-default-browser-check",
      "--hide-scrollbars",
      "--remote-debugging-port=0",
      `--user-data-dir=${path.join(profileDirectory, "profile")}`,
      `--window-size=${viewport.width},${viewport.height}`,
      "about:blank",
    ], { stdio: ["ignore", "ignore", "pipe"] });

    let browserWebSocketUrl = "";
    chrome.stderr.setEncoding("utf8");
    chrome.stderr.on("data", (chunk) => {
      const match = chunk.match(/DevTools listening on (ws:\/\/[^\s]+)/);
      if (match) browserWebSocketUrl = match[1];
    });

    try {
      browserWebSocketUrl = await waitFor(() => browserWebSocketUrl, 10000, "Chrome DevTools endpoint");
      const browserWebSocket = new URL(browserWebSocketUrl);
      const targets = await (await fetch(`http://${browserWebSocket.host}/json/list`)).json();
      const pageTarget = targets.find((target) => target.type === "page");
      if (!pageTarget?.webSocketDebuggerUrl) throw new Error(`Chrome page target unavailable for ${viewport.name}`);

      const webSocket = new WebSocket(pageTarget.webSocketDebuggerUrl);
      await new Promise((resolve, reject) => {
        webSocket.onopen = resolve;
        webSocket.onerror = reject;
      });

      let requestId = 0;
      const pending = new Map();
      webSocket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        const waiter = pending.get(message.id);
        if (!waiter) return;
        pending.delete(message.id);
        if (message.error) waiter.reject(new Error(JSON.stringify(message.error)));
        else waiter.resolve(message.result);
      };
      const send = (method, params = {}) => {
        const id = ++requestId;
        webSocket.send(JSON.stringify({ id, method, params }));
        return new Promise((resolve, reject) => pending.set(id, { resolve, reject }));
      };

      await send("Page.enable");
      await send("Runtime.enable");
      await send("Emulation.setDeviceMetricsOverride", {
        width: viewport.width,
        height: viewport.height,
        deviceScaleFactor: 1,
        mobile: false,
      });
      const navigation = await send("Page.navigate", { url: baseUrl });
      await sleep(900);

      const evaluation = await send("Runtime.evaluate", {
        expression: `(() => {
          const viewportName = ${JSON.stringify(viewport.name)};
          const expectedVisible = ${JSON.stringify(viewport.visible)};
          const expectedHidden = ${JSON.stringify(viewport.hidden)};
          const actualVisibility = ${actualVisibilityFunctionSource};
          ${temperedGlassProbeSource}
        })()`,
        returnByValue: true,
      });
      webSocket.close();
      if (evaluation.exceptionDetails) {
        const message = evaluation.exceptionDetails.exception?.description || evaluation.exceptionDetails.text || "runtime_evaluate_exception";
        return { name: viewport.name, viewport: `${viewport.width}x${viewport.height}`, status: "failed", errors: [String(message).slice(0, 240)] };
      }
      const value = evaluation.result?.value || { errors: ["runtime_evaluate_no_value"] };
      const errors = Array.isArray(value.errors) ? value.errors : ["runtime_evaluate_invalid_errors"];
      return {
        name: viewport.name,
        viewport: `${viewport.width}x${viewport.height}`,
        status: errors.length === 0 ? "ready" : "failed",
        navigation_error: navigation.errorText || null,
        ...value,
        errors,
      };
    } finally {
      if (chrome.exitCode === null) chrome.kill("SIGTERM");
      await Promise.race([
        new Promise((resolve) => chrome.once("exit", resolve)),
        sleep(1500).then(() => {
          if (chrome.exitCode === null) chrome.kill("SIGKILL");
        }),
      ]);
      await fsPromises.rm(profileDirectory, { recursive: true, force: true, maxRetries: 5, retryDelay: 100 });
    }
  }

  const results = [];
  for (const viewport of viewports) results.push(await inspectViewport(viewport));
  const failures = results.flatMap((result) => result.errors.map((error) => `${result.name}:${error}`));
  const all = (field) => results.every((result) => result[field] === true);
  const maximum = (field) => Math.max(...results.map((result) => result[field] || 0));

  const report = {
    schema_version: 2,
    gate: "control_ui_tempered_glass_density_qa",
    status: failures.length === 0 ? "ready" : "failed",
    control_ui_visual_density_qa_ready: failures.length === 0,
    viewport_count: results.length,
    phone320_ready: results.some((result) => result.name === "phone320" && result.status === "ready"),
    expected_visibility_ready: all("expected_visibility_ready"),
    browser_error_page_absent: all("browser_error_page_absent"),
    horizontal_overflow_free: all("horizontal_overflow_free"),
    stable_content_surface_ready: all("stable_content_surface_ready"),
    tempered_surface_budget_ready: all("tempered_surface_budget_ready"),
    visible_text_floor_ready: all("visible_text_floor_ready"),
    key_touch_controls_ready: all("key_touch_controls_ready"),
    mobile_single_topbar_ready: all("mobile_single_topbar_ready"),
    mobile_topbar_semantics_ready: all("mobile_topbar_semantics_ready"),
    mobile_single_bottom_action_layer_ready: all("mobile_single_bottom_action_layer_ready"),
    mobile_primary_actions_ready: all("mobile_primary_actions_ready"),
    narrow_shell_density_ready: all("narrow_shell_density_ready"),
    narrow_single_action_row_ready: all("narrow_single_action_row_ready"),
    maximum_visible_glass_surface_count: maximum("visible_glass_surface_count"),
    maximum_shadow_layer_count: maximum("maximum_shadow_layer_count"),
    maximum_gradient_layer_count: maximum("maximum_gradient_layer_count"),
    maximum_border_layer_count: maximum("maximum_border_layer_count"),
    visible_under_12px_count: results.reduce((total, result) => total + (result.visible_under_12px_count || 0), 0),
    results,
    failures,
  };

  fs.writeSync(1, `${JSON.stringify(report)}\n`);
  if (failures.length > 0) process.exit(1);
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});

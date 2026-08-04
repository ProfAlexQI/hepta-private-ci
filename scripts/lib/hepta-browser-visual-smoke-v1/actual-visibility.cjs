"use strict";

function inspectActualVisibility(node) {
  if (typeof HTMLElement === "undefined" || !(node instanceof HTMLElement)) {
    return {
      visible: false,
      display: "",
      visibility: "",
      opacity: "",
      rect_width: 0,
      rect_height: 0,
      client_rect_count: 0,
      hidden_attribute: false,
    };
  }
  const style = getComputedStyle(node);
  const rect = node.getBoundingClientRect();
  const clientRectCount = node.getClientRects().length;
  const visible = style.display !== "none"
    && style.visibility !== "hidden"
    && style.visibility !== "collapse"
    && Number.parseFloat(style.opacity || "1") > 0
    && rect.width > 0
    && rect.height > 0
    && clientRectCount > 0;
  return {
    visible,
    display: style.display,
    visibility: style.visibility,
    opacity: style.opacity,
    rect_width: rect.width,
    rect_height: rect.height,
    client_rect_count: clientRectCount,
    hidden_attribute: node.hidden,
  };
}

const actualVisibilityFunctionSource = `(${inspectActualVisibility.toString()})`;
const installActualVisibilitySource = `
Object.defineProperty(window, "__heptaActualVisibility", {
  configurable: false,
  value: ${actualVisibilityFunctionSource},
});
`;

module.exports = {
  actualVisibilityFunctionSource,
  installActualVisibilitySource,
};

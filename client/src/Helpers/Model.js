import { createRenderEffect } from "solid-js";

function Model(el, accessor) {
  const [s, set] = accessor();
  el.addEventListener("input", (e) => set(e.currentTarget.value.trim()));
  createRenderEffect(() => (el.value = s()));
}

export default Model;
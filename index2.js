import wasmInit from "./pkg/exports.js";
import * as wasm from "./pkg/exports.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const rustWasm = await wasmInit("./pkg/exports_bg.wasm");

  // Call the Add function export from wasm, save the result
  let result = wasm.call_me_from_javascript();

  console.log(result.get_v()); // Should output '72'
};
runWasm();
import { defineConfig } from "vite";
import preact from "@preact/preset-vite";
import wasmPack from "vite-plugin-wasm-pack";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [preact(), wasmPack("./lisp_rs")],
});

import { useMemo } from "preact/hooks";
import Terminal from "./Terminal";
import "./app.css";
import init from "../../target/wasm32-unknown-unknown/debug/lisp_rs.wasm?init";

export function App() {
  init({ help: { no: 0 } }).then((x) => console.log(x));

  const worker = useMemo(
    () => new Worker(new URL("./worker.ts", import.meta.url)),
    []
  );

  const onCommand = (input: number[]) => {
    worker.postMessage(
      input.map((charCode: number) => String.fromCharCode(charCode)).join("")
    );
  };

  return <Terminal onCommand={onCommand} worker={worker} />;
}

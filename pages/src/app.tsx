import { useMemo } from "preact/hooks";
import Terminal from "./Terminal";
import "./app.css";

export function App() {
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

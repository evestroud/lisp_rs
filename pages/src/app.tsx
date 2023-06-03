import { useCallback, useMemo } from "preact/hooks";
import Terminal from "./Terminal";
import "./app.css";
import useReader from "./useReader";

export function App() {
  const reader = useReader();

  const worker = useMemo(
    () => new Worker(new URL("./worker.ts", import.meta.url)),
    []
  );

  const onReadable = useCallback(
    (input: number[]) => {
      const message = input
        .map((charCode: number) => String.fromCharCode(charCode))
        .join("");

      let result: string | unknown = "";
      try {
        reader?.push(message);

        if (reader?.expression_complete()) {
          result = reader?.eval();
        }
      } catch (e) {
        result = e;
        reader?.clear_buffer();
      }

      const prompt = reader?.new_expression() ? "> " : ". ";
      worker.postMessage({ result, prompt });
    },
    [reader]
  );

  const onSignal = (_: string) => {
    reader?.clear_buffer();
    worker.postMessage({ result: " ", prompt: "> " });
  };

  return (
    <Terminal onReadable={onReadable} onSignal={onSignal} worker={worker} />
  );
}

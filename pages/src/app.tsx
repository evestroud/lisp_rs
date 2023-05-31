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

  const onCommand = useCallback(
    (input: number[]) => {
      const message = input
        .map((charCode: number) => String.fromCharCode(charCode))
        .join("");
      reader?.push(message);

      let result: string | unknown = "";
      if (reader?.expression_complete()) {
        try {
          result = reader?.eval();
        } catch (e) {
          result = e;
        }
      }

      const prompt =
        reader?.expression_complete() || reader?.new_expression() ? "> " : ". ";
      worker.postMessage({ result, prompt });
    },
    [reader]
  );

  return <Terminal onCommand={onCommand} worker={worker} />;
}

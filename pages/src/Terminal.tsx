import { useEffect, useRef } from "preact/hooks";
import { Terminal as _Terminal } from "xterm";
import { Readline } from "xterm-readline";
import { FitAddon } from "xterm-addon-fit";
import "../node_modules/xterm/css/xterm.css";
import "./Terminal.css";

interface TerminalProps {
  onReadable: (line: string) => void;
  onSignal: () => void;
  worker: Worker;
}

const Terminal = ({ onReadable, onSignal, worker }: TerminalProps) => {
  const divRef = useRef(null);
  const promptRef = useRef("> ");

  useEffect(() => {
    if (!divRef.current) throw new Error("div not found");

    const terminal = new _Terminal({
      cursorBlink: true,
    });
    terminal.open(divRef.current);
    const rl = new Readline();
    terminal.loadAddon(rl);
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    fitAddon.fit();
    terminal.focus();

    // rl.setCheckHandler TODO

    const readLine = () => {
      rl.read(promptRef.current).then(processLine);
    };

    const processLine = (input: string) => {
      onReadable(input);
    };

    terminal.attachCustomKeyEventHandler((event: KeyboardEvent) => {
      if (event.ctrlKey && event.key === "c") {
        onSignal();
      }
      if (event.key === "Tab") {
        console.log(event);
        // TODO figure out how to indent
        // event.shiftKey dedent?
      }
      return true;
    });

    worker.onmessage = ({ data: { result, prompt } }) => {
      if (result) {
        rl.println(result);
      }
      promptRef.current = prompt;
      setTimeout(readLine);
    };

    readLine();
    return () => terminal.dispose();
  }, [onReadable, onSignal, worker]);

  return <div id="terminal" ref={divRef} />;
};

export default Terminal;

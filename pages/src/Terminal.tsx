import { useEffect, useRef } from "preact/hooks";
import { Terminal as _Terminal } from "xterm";
import { openpty } from "xterm-pty";
import "../node_modules/xterm/css/xterm.css";

interface TerminalProps {
  onReadable: (line: number[]) => void;
  onSignal: (signal: string) => void;
  worker: Worker;
}

const Terminal = ({ onReadable, onSignal, worker }: TerminalProps) => {
  const divRef = useRef(null);

  useEffect(() => {
    const terminal = new _Terminal({
      cursorBlink: true,
    });
    terminal.open(divRef.current!);
    const { master, slave } = openpty();
    terminal.loadAddon(master);

    terminal.focus();
    terminal.write("> ");

    slave.onReadable(() => {
      const line = slave.read();
      onReadable(line);
    });

    slave.onSignal(onSignal);

    worker.onmessage = (event) => {
      slave.write(`${event.data}`);
    };

    return () => terminal.dispose();
  }, [onReadable, worker]);

  return <div ref={divRef} />;
};

export default Terminal;

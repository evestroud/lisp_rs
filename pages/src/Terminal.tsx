import { useEffect, useRef } from "preact/hooks";
import { Terminal as _Terminal } from "xterm";
import { openpty } from "xterm-pty";
import "../node_modules/xterm/css/xterm.css";

interface TerminalProps {
  onCommand: (line: number[]) => void;
  worker: Worker;
}

const Terminal = ({ onCommand, worker }: TerminalProps) => {
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
      const command = slave.read();
      onCommand(command);
    });

    worker.onmessage = (event) => {
      slave.write(`${event.data}`);
      terminal.write("> ");
    };

    return () => terminal.dispose();
  }, []);

  return <div ref={divRef} />;
};

export default Terminal;

import { useCallback, useEffect, useMemo, useState } from "preact/hooks";
import init, { Reader } from "lisp_rs";

const useReader = () => {
  const [reader, setReader] = useState<Reader>();

  useEffect(() => {
    init().then(() => {
      setReader(Reader.new());
    });
  }, []);

  return reader;
};

export default useReader;

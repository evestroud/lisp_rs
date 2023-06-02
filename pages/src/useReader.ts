import { useEffect, useState } from "preact/hooks";
import * as lisp_rs from "../../pkg/lisp_rs.js";

const useReader = () => {
  const [reader, setReader] = useState<lisp_rs.Reader>();

  useEffect(() => {
    lisp_rs.default().then(() => {
      setReader(lisp_rs.Reader.new());
    });
  }, []);

  return reader;
};

export default useReader;

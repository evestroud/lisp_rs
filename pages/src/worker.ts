onmessage = ({ data: { result, prompt } }) => {
  postMessage(result.length ? result + "\n" + prompt : "" + prompt);
};

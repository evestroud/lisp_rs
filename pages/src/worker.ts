onmessage = ({ data: { result, prompt } }) => {
  postMessage({ result, prompt });
};

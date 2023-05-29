onmessage = (event) => {
  console.log(`Worker recieved: ${event.data}`);
  postMessage(event.data);
};

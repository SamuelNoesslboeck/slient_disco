let s = new WebSocket("/mic");
let counter = 0;

s.addEventListener("message", (e) => {
  console.log(`RECEIVED: ${e.data}: ${counter}`);
  counter++;
});
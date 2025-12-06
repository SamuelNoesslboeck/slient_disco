const ws = new WebSocket("/mic");
ws.binaryType = "arraybuffer";

const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

// Suppose you have a “Start audio” button:
const btn = document.getElementById("startAudioButton");
btn.addEventListener("click", async () => {
  if (audioCtx.state === "suspended") {
    await audioCtx.resume();
  }
});

function playFloat32AudioChunk(arrayBuffer, sampleRate = 44100) {
  const floatSamples = new Float32Array(arrayBuffer);
  const numChannels = 2;
  const frameCount = floatSamples.length / numChannels;

  const audioBuffer = audioCtx.createBuffer(
    numChannels,
    frameCount,
    sampleRate
  );

  // Separate interleaved data into each channel
  const channelDataLeft  = audioBuffer.getChannelData(0);
  const channelDataRight = audioBuffer.getChannelData(1);

  for (let i = 0; i < frameCount; i++) {
    channelDataLeft[i]  = floatSamples[i * 2];
    channelDataRight[i] = floatSamples[i * 2 + 1];
  }

  const source = audioCtx.createBufferSource();
  source.buffer = audioBuffer;
  source.connect(audioCtx.destination);
  source.start();
}

ws.addEventListener("message", (event) => {
  if (event.data instanceof ArrayBuffer) {
    const arrayBuffer = event.data;
    playFloat32AudioChunk(arrayBuffer);
  } else {
    console.log("Received non-binary data:", event.data);
  }
})
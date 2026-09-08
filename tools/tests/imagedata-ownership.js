// Run in a NativeScript Apple runtime. Rejects if explicit GC is unavailable.
// Pixel views must survive collection of their ImageData wrapper, and collecting
// both must release each native allocation only once (including V8 worker GC).
export async function checkImageDataOwnership(ImageData) {
  if (typeof global.gc !== 'function') throw new Error('NativeScript global.gc is required');
  for (let round = 0; round < 8; round++) {
    const retained = [];
    for (let i = 0; i < 512; i++) {
      const image = new ImageData(4, 4);
      const bytes = image.data;
      bytes.set([i % 256, round, 127, 255]);
      if (i % 64 === 0) retained.push({ bytes, expected: i % 256 });
    }
    global.gc();
    await new Promise(resolve => setTimeout(resolve, 25));
    for (const { bytes, expected } of retained) {
      if (bytes[0] !== expected || bytes[1] !== round || bytes[2] !== 127 || bytes[3] !== 255)
        throw new Error('ImageData view changed after wrapper collection');
      bytes[0] = 255;
      if (bytes[0] !== 255) throw new Error('Retained ImageData view is not writable');
    }
  }
  global.gc();
  await new Promise(resolve => setTimeout(resolve, 100));
  return { allocations: 4096, retainedViews: 64 };
}

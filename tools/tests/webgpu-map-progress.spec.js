import { describe, it, expect } from 'vitest';
import { GPU, GPUBufferUsage, GPUMapMode } from '@nativescript/canvas';

// Run inside a native runtime, with no WebGPU canvas or requestAnimationFrame.
describe('WebGPU mapping progress', () => {
  it('maps compute output, remaps, and rejects an invalid range', async () => {
    const adapter = await new GPU().requestAdapter();
    expect(adapter).toBeTruthy();
    const device = await adapter.requestDevice();
    const output = device.createBuffer({ size: 16, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
    const readback = device.createBuffer({ size: 16, usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST });
    const module = device.createShaderModule({ code: '@group(0) @binding(0) var<storage, read_write> values: array<u32>; @compute @workgroup_size(1) fn main() { values[0] = 42u; }' });
    const pipeline = device.createComputePipeline({ layout: 'auto', compute: { module, entryPoint: 'main' } });
    const bindGroup = device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: output } }] });
    const encoder = device.createCommandEncoder();
    const pass = encoder.beginComputePass();
    pass.setPipeline(pipeline);
    pass.setBindGroup(0, bindGroup);
    pass.dispatchWorkgroups(1);
    pass.end();
    encoder.copyBufferToBuffer(output, 0, readback, 0, 16);
    device.queue.submit([encoder.finish()]);
    await readback.mapAsync(GPUMapMode.READ);
    expect(new Uint32Array(readback.getMappedRange())[0]).toBe(42);
    readback.unmap();
    // A second map also progresses without submitting or rendering another frame.
    await readback.mapAsync(GPUMapMode.READ);
    expect(new Uint32Array(readback.getMappedRange())[0]).toBe(42);
    readback.unmap();
    await expect(readback.mapAsync(GPUMapMode.READ, 1, 4)).rejects.toThrow();
    readback.destroy();
    output.destroy();
    device.destroy();
  });
});

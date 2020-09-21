package com.github.triniwiz.canvas;


import android.graphics.Bitmap;
import android.opengl.GLES20;
import android.opengl.GLES30;
import android.os.Build;

import androidx.annotation.RequiresApi;

import java.nio.ByteBuffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.nio.LongBuffer;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Map;
import java.util.concurrent.CountDownLatch;

import static android.os.Build.VERSION_CODES.JELLY_BEAN_MR2;

/**
 * Created by triniwiz on 5/1/20
 */
@RequiresApi(JELLY_BEAN_MR2)
public class WebGL2RenderingContext extends WebGLRenderingContext {

    native void nativeFlipInPlace3D(byte[] storage, int width, int height, int depth);

    native void nativeTexImage3DAsset(int target, int level, int internalformat, int width,int height, int depth, int border, int format, int image_type, long asset, boolean flipY);

    native void nativeTexSubImage3DAsset(int target, int level, int xoffset, int yoffset, int zoffset, int width, int height, int depth, int format, int type, long asset, boolean flipY);
    public WebGL2RenderingContext(CanvasView canvas) {
        super(canvas);
    }

    public WebGL2RenderingContext(CanvasView canvas, Map<String, Object> attrs) {
        super(canvas, attrs);
    }

    public void beginQuery(final int target, final int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBeginQuery(target, query);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void beginTransformFeedback(final int primitiveMode) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBeginTransformFeedback(primitiveMode);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void bindBufferBase(final int target, final int index, final int buffer) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {

                GLES30.glBindBufferBase(target, index, buffer);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void bindBufferRange(final int target, final int index, final int buffer, final int offset, final int size) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBindBufferRange(target, index, buffer, offset, size);
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void bindSampler(final int unit, final int sampler) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBindSampler(unit, sampler);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void bindTransformFeedback(final int target, final int transformFeedback) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBindTransformFeedback(target, transformFeedback);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void bindVertexArray(final int vertexArray) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBindVertexArray(vertexArray);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void blitFramebuffer(final int srcX0, final int srcY0, final int srcX1, final int srcY1,
                                final int dstX0, final int dstY0, final int dstX1, final int dstY1,
                                final int mask, final int filter) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glBlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void clearBufferfv(final int buffer, final int drawbuffer, final float[] values) {
        final CountDownLatch lock = new CountDownLatch(1);

        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glClearBufferfv(buffer, drawbuffer, FloatBuffer.wrap(values));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void clearBufferiv(final int buffer, final int drawbuffer, final int[] values) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glClearBufferiv(buffer, drawbuffer, IntBuffer.wrap(values));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void clearBufferuiv(final int buffer, final int drawbuffer, final int[] values) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glClearBufferuiv(buffer, drawbuffer, IntBuffer.wrap(values));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void clearBufferfi(final int buffer, final int drawbuffer, final float depth, final int stencil) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glClearBufferfi(buffer, drawbuffer, depth, stencil);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public int clientWaitSync(final long sync, final int flags, final long timeout) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] value = new int[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glClientWaitSync(sync, flags, timeout);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public void compressedTexSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int imageSize, final int offset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, offset);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void compressedTexSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final byte[] srcData, final int srcOffset, int srcLengthOverride) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                ByteBuffer buffer = ByteBuffer.wrap(srcData);
                buffer.position(srcOffset);
                GLES30.glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, SIZE_OF_BYTE * srcData.length, buffer);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public void copyBufferSubData(final int readTarget, final int writeTarget, final int readOffset, final int writeOffset, final int size) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glCopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void copyTexSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int x, final int y, final int width, final int height) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
              clearIfComposited();
                GLES30.glCopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }

    public int createQuery() {
        final CountDownLatch lock = new CountDownLatch(1);
        final IntBuffer query = IntBuffer.allocate(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenQueries(1, query);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return query.get(0);
    }

    public int createSampler() {
        final CountDownLatch lock = new CountDownLatch(1);
        final IntBuffer sampler = IntBuffer.allocate(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenSamplers(1, sampler);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return sampler.get(0);
    }

    public int createVertexArray() {
        final CountDownLatch lock = new CountDownLatch(1);
        final IntBuffer array = IntBuffer.allocate(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenVertexArrays(1, array);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return array.get(0);
    }

    public int createTransformFeedback() {
        final CountDownLatch lock = new CountDownLatch(1);
        final IntBuffer id = IntBuffer.allocate(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glGenTransformFeedbacks(1, id);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return id.get(0);
    }

    public void deleteQuery(final int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                int[] id = {query};
                GLES30.glDeleteQueries(1, id, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void deleteSampler(final int sampler) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                int[] id = {sampler};
                GLES30.glDeleteQueries(1, id, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void deleteSync(final int sync) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glDeleteSync(sync);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void deleteTransformFeedback(final int transformFeedback) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                int[] feedback = {transformFeedback};
                GLES30.glDeleteTransformFeedbacks(1, feedback, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void deleteVertexArray(final int vertexArray) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                int[] array = {vertexArray};
                GLES30.glDeleteVertexArrays(1, array, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void drawArraysInstanced(final int mode, final int first, final int count, final int instanceCount) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
              clearIfComposited();
                GLES30.glDrawArraysInstanced(mode, first, count, instanceCount);
                updateCanvas();;
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void drawElementsInstanced(final int mode, final int count, final int type, final int offset, final int instanceCount) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
              clearIfComposited();
                GLES30.glDrawElementsInstanced(mode, count, type, offset, instanceCount);
                updateCanvas();;
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void drawRangeElements(final int mode, final int start, final int end, final int count, final int type, final int offset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
              clearIfComposited();
                GLES30.glDrawRangeElements(mode, start, end, count, type, offset);
                updateCanvas();
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

  public void drawBuffers(final int[] buffers) {
    final CountDownLatch lock = new CountDownLatch(1);
    runOnGLThread(new Runnable() {
      @Override
      public void run() {
        GLES30.glDrawBuffers(buffers.length, IntBuffer.wrap(buffers));
        lock.countDown();
      }
    });
    try {
      lock.await();
    } catch (InterruptedException ignored) {
    }
  }

    public void endQuery(final int target) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glEndQuery(target);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void endTransformFeedback() {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glEndTransformFeedback();
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void fenceSync(final int condition, final int flags) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glFenceSync(condition, flags);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void framebufferTextureLayer(final int target, final int attachment, final int texture, final int level, final int layer) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glFramebufferTextureLayer(target, attachment, texture, level, layer);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public String getActiveUniformBlockName(final int program, final int uniformBlockIndex) {
        final CountDownLatch lock = new CountDownLatch(1);
        final String[] value = new String[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                IntBuffer maxNameLength = IntBuffer.allocate(1);
                GLES30.glGetProgramiv(program, GLES30.GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH, maxNameLength);
                byte[] name = new byte[maxNameLength.get(0)];
                IntBuffer length = IntBuffer.allocate(1);
                GLES30.glGetActiveUniformBlockName(program, uniformBlockIndex, length, ByteBuffer.wrap(name));
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
                    value[0] = new String(name, StandardCharsets.UTF_8);
                } else {
                    value[0] = new String(name, Charset.forName("UTF-8"));
                }
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getActiveUniformBlockParameter(final int program, final int uniformBlockIndex, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                switch (pname) {
                    case GLES30.GL_UNIFORM_BLOCK_BINDING:
                    case GLES30.GL_UNIFORM_BLOCK_DATA_SIZE:
                    case GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS:
                        IntBuffer intValue = IntBuffer.allocate(1);
                        GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, intValue);
                        value[0] = intValue.get(0);
                        break;
                    case GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES:
                        IntBuffer uniformCount = IntBuffer.allocate(1);
                        GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS, uniformCount);
                        int[] indices = new int[uniformCount.get(0)];
                        GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, IntBuffer.wrap(indices));
                        value[0] = indices;
                        break;
                    case GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:
                    case GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
                        IntBuffer boolValue = IntBuffer.allocate(1);
                        GLES30.glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, boolValue);
                        value[0] = boolValue.get(0) == GLES30.GL_TRUE;
                        break;
                    default:
                        value[0] = null;
                        break;
                }
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    enum ReturnType {
        EnumType,
        UnsignedIntType,
        IntType,
        BoolType
    }

    public Object getActiveUniforms(final int program, final int[] uniformIndices, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                ReturnType returnType;
                switch (pname) {
                    case GLES30.GL_UNIFORM_TYPE:
                        returnType = ReturnType.EnumType;
                        break;
                    case GLES30.GL_UNIFORM_SIZE:
                        returnType = ReturnType.UnsignedIntType;
                        break;
                    case GLES30.GL_UNIFORM_BLOCK_INDEX:
                    case GLES30.GL_UNIFORM_OFFSET:
                    case GLES30.GL_UNIFORM_ARRAY_STRIDE:
                    case GLES30.GL_UNIFORM_MATRIX_STRIDE:
                        returnType = ReturnType.IntType;
                        break;
                    case GLES30.GL_UNIFORM_IS_ROW_MAJOR:
                        returnType = ReturnType.BoolType;
                        break;
                    default:
                        value[0] = null;
                        lock.countDown();
                        return;
                }
                IntBuffer activeUniforms = IntBuffer.allocate(1);
                GLES30.glGetProgramiv(program, GLES30.GL_ACTIVE_UNIFORMS,
                        activeUniforms);


                IntBuffer activeUniformsUnsigned = activeUniforms;
                int size = uniformIndices.length;
                for (int i = 0; i < size; i++) {
                    if (i >= activeUniformsUnsigned.get(0)) {
                        value[0] = null;
                        lock.countDown();
                        return;
                    }
                }

                int[] indices = uniformIndices;
                int[] result = new int[size];
                GLES30.glGetActiveUniformsiv(program, uniformIndices.length,
                        IntBuffer.wrap(indices), pname, IntBuffer.wrap(result));

                switch (returnType) {
                    case IntType:
                    case EnumType:
                    case UnsignedIntType:
                        value[0] = result;
                        break;
                    case BoolType:
                        value[0] = fromGLint(result);
                        break;
                    default:
                        value[0] = null;
                        break;
                }
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    boolean[] fromGLint(int[] value) {
        boolean[] array = new boolean[value.length];
        for (int i = 0; i < value.length; i++) {
            array[i] = (value[i] == GLES30.GL_TRUE);
        }
        return array;
    }

    public void getBufferSubData(final int target, int srcByteOffset, final byte[] dstData, final int dstOffset, final int length) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                if (length == 0) {

                }

                int size = dstData.length * SIZE_OF_BYTE;
                int typeSize = SIZE_OF_BYTE;
                int byteLength = 0;
                if (length > 0) {
                    // type size is at most 8, so no overflow.
                    byteLength = length * typeSize;
                }
                int byteOffset = 0;
                if (dstOffset > 0) {
                    // type size is at most 8, so no overflow.
                    byteOffset = dstOffset * typeSize;
                }
                int total = byteOffset;
                total += byteLength;
                if (total > size) {
                    return;
                }
                if (byteLength == 0) {
                    byteLength = size - byteOffset;
                }

                GLES30.glBufferSubData(target, byteOffset, byteLength, ByteBuffer.wrap(dstData));
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public int getFragDataLocation(final int program, final String name) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] value = new int[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glGetFragDataLocation(program, name);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }


    public Object getIndexedParameter(final int target, final int index) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] val = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                IndexedParameter binding = new IndexedParameter();
                switch (target) {
                    case GLES30.GL_UNIFORM_BUFFER_BINDING:
                    case GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_BINDING:
                        IntBuffer newTarget = IntBuffer.allocate(1);
                        GLES30.glGetIntegerv(target, newTarget);
                        // NO BINDING RETURN
                        if (newTarget.get(0) == 0) {
                            val[0] = null;
                            break;
                        }
                        IntBuffer buffer = IntBuffer.allocate(1);
                        GLES30.glGetIntegeri_v(newTarget.get(0), index, buffer);
                        binding.bufferValue = buffer.get(0);
                        binding.isBuffer = true;
                        val[0] = binding;
                        break;
                    case GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:
                    case GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_START:
                    case GLES30.GL_UNIFORM_BUFFER_SIZE:
                    case GLES30.GL_UNIFORM_BUFFER_START:
                        LongBuffer value = LongBuffer.allocate(1);
                        GLES30.glGetInteger64i_v(target, index, value);
                        binding.isBuffer = false;
                        binding.value = value.get(0);
                        val[0] = binding;
                        break;
                    default:
                        val[0] = null;
                        break;
                }
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return val[0];
    }

    public Object getInternalformatParameter(final int target, final int internalformat, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                switch (internalformat) {
                    // Renderbuffer doesn't support unsized internal formats,
                    // though GL_RGB and GL_RGBA are color-renderable.
                    case GLES30.GL_RGB:
                    case GLES30.GL_RGBA:
                        // Multisampling is not supported for signed and unsigned integer internal
                        // formats.
                    case GLES30.GL_R8UI:
                    case GLES30.GL_R8I:
                    case GLES30.GL_R16UI:
                    case GLES30.GL_R16I:
                    case GLES30.GL_R32UI:
                    case GLES30.GL_R32I:
                    case GLES30.GL_RG8UI:
                    case GLES30.GL_RG8I:
                    case GLES30.GL_RG16UI:
                    case GLES30.GL_RG16I:
                    case GLES30.GL_RG32UI:
                    case GLES30.GL_RG32I:
                    case GLES30.GL_RGBA8UI:
                    case GLES30.GL_RGBA8I:
                    case GLES30.GL_RGB10_A2UI:
                    case GLES30.GL_RGBA16UI:
                    case GLES30.GL_RGBA16I:
                    case GLES30.GL_RGBA32UI:
                    case GLES30.GL_RGBA32I:
                        value[0] = new int[0];
                        lock.countDown();
                        return;
                    case GLES30.GL_R8:
                    case GLES30.GL_RG8:
                    case GLES30.GL_RGB565:
                    case GLES30.GL_RGBA8:
                    case GLES30.GL_SRGB8_ALPHA8:
                    case GLES30.GL_RGB5_A1:
                    case GLES30.GL_RGBA4:
                    case GLES30.GL_RGB10_A2:
                    case GLES30.GL_DEPTH_COMPONENT16:
                    case GLES30.GL_DEPTH_COMPONENT24:
                    case GLES30.GL_DEPTH_COMPONENT32F:
                    case GLES30.GL_DEPTH24_STENCIL8:
                    case GLES30.GL_DEPTH32F_STENCIL8:
                    case GLES30.GL_STENCIL_INDEX8:
                        break;
                    case GLES30.GL_R16F:
                    case GLES30.GL_RG16F:
                    case GLES30.GL_R32F:
                    case GLES30.GL_RG32F:
                    case GLES30.GL_RGBA32F:
                    case GLES30.GL_R11F_G11F_B10F:
                        break;
                    default:
                        value[0] = null;
                        lock.countDown();
                        return;
                }

                if (pname == GLES30.GL_SAMPLES) {
                    IntBuffer length = IntBuffer.allocate(1);
                    GLES30.glGetInternalformativ(target, internalformat,
                            GLES30.GL_NUM_SAMPLE_COUNTS, 1, length);
                    if (length.get(0) <= 0) {
                        value[0] = new int[0];
                        lock.countDown();
                        return;
                    }
                    int[] values = new int[length.get(0)];
                    GLES30.glGetInternalformativ(target, internalformat, pname, length.get(0), IntBuffer.wrap(values));
                    value[0] = values;
                } else {
                    value[0] = null;
                }
                lock.countDown();


            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getQuery(final int target, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                if (pname == GLES30.GL_CURRENT_QUERY) {
                    IntBuffer params = IntBuffer.allocate(1);
                    GLES30.glGetQueryiv(target, pname, params);
                    value[0] = params.get(0);
                } else {
                    value[0] = null;
                }
                lock.countDown();

            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getQueryParameter(final int query, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                final IntBuffer params = IntBuffer.allocate(1);
                GLES30.glGetQueryObjectuiv(query, pname, params);
                switch (pname) {
                    case GLES30.GL_QUERY_RESULT:
                        value[0] = params.get(0) == GLES30.GL_TRUE;
                        break;
                    case GLES30.GL_QUERY_RESULT_AVAILABLE:
                        value[0] = params;
                        break;
                    default:
                        value[0] = null;
                        break;
                }
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getSamplerParameter(final int sampler, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                switch (pname) {
                    case TEXTURE_MAX_LOD:
                    case TEXTURE_MIN_LOD:
                        FloatBuffer floatValue = FloatBuffer.allocate(1);
                        GLES30.glGetSamplerParameterfv(sampler, pname, floatValue);
                        value[0] = floatValue.get(0);
                        break;
                    case TEXTURE_COMPARE_FUNC:
                    case TEXTURE_COMPARE_MODE:
                    case TEXTURE_MAG_FILTER:
                    case TEXTURE_MIN_FILTER:
                    case TEXTURE_WRAP_R:
                    case TEXTURE_WRAP_S:
                    case TEXTURE_WRAP_T:
                        IntBuffer intValue = IntBuffer.allocate(1);
                        GLES30.glGetSamplerParameteriv(sampler, pname, intValue);
                        value[0] = intValue.get(0);
                        break;
                    default:
                        value[0] = null;
                        break;
                }
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getSyncParameter(final int sync, final int pname) {
        final CountDownLatch lock = new CountDownLatch(1);
        final Object[] value = new Object[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                switch (pname) {
                    case GLES30.GL_OBJECT_TYPE:
                    case GLES30.GL_SYNC_STATUS:
                    case GLES30.GL_SYNC_CONDITION:
                    case GLES30.GL_SYNC_FLAGS:
                        IntBuffer values = IntBuffer.allocate(1);
                        IntBuffer length = IntBuffer.allocate(1);
                        GLES30.glGetSynciv(sync, pname, 1, length, values);
                        value[0] = values.get(0);
                        break;
                    default:
                        value[0] = null;
                        break;
                }
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public Object getTransformFeedbackVarying(final int program, final int index) {
        final CountDownLatch lock = new CountDownLatch(1);
        final WebGLActiveInfo[] info = new WebGLActiveInfo[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                IntBuffer maxIndex = IntBuffer.allocate(1);

                GLES30.glGetProgramiv(program, GLES30.GL_TRANSFORM_FEEDBACK_VARYINGS, maxIndex);
                if (index >= maxIndex.get(0)) {
                    info[0] = null;
                    return;
                }

                int[] maxNameLength = new int[1];
                GLES30.glGetProgramiv(program, GLES30.GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
                        maxNameLength, 0);
                if (maxNameLength[0] <= 0) {
                    info[0] = null;
                    return;
                }
                byte[] name = new byte[maxNameLength[0]];
                int[] length = new int[1];
                int[] size = new int[1];
                int[] type = new int[1];
                GLES30.glGetTransformFeedbackVarying(program, index, maxNameLength[0], length, 0, size, 0, type, 0, name, 0);
                if (length[0] == 0 || size[0] == 0 || type[0] == 0) {
                    info[0] = null;
                    return;
                }

                name = Arrays.copyOfRange(name, 0, length[0]);
                String nameValue = new String(name);
                info[0] = new WebGLActiveInfo(nameValue, size[0], type[0]);
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return info[0];
    }

    public int getUniformBlockIndex(final int program, final String uniformBlockName) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[] value = new int[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glGetUniformBlockIndex(program, uniformBlockName);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }


    public int[] getUniformIndices(final int program, final String[] uniformNames) {
        final CountDownLatch lock = new CountDownLatch(1);
        final int[][] value = new int[1][];

        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                int[] indices = new int[uniformNames.length];
                GLES30.glGetUniformIndices(program, uniformNames, IntBuffer.wrap(indices));
                value[0] = indices;
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public void invalidateFramebuffer(final int target, final int[] attachments) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glInvalidateFramebuffer(target, attachments.length, IntBuffer.wrap(attachments));
                lock.countDown();
            }
        });

        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void invalidateSubFramebuffer(final int target, final int[] attachments, final int x, final int y, final int width, final int height) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glInvalidateSubFramebuffer(target, attachments.length, IntBuffer.wrap(attachments), x, y, width, height);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public boolean isQuery(final int query) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsQuery(query);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public boolean isSampler(final int sampler) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsSampler(sampler);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public boolean isSync(final int sync) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsSync(sync);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public boolean isTransformFeedback(final int transformFeedback) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsTransformFeedback(transformFeedback);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public boolean isVertexArray(final int vertexArray) {
        final CountDownLatch lock = new CountDownLatch(1);
        final boolean[] value = new boolean[1];
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                value[0] = GLES30.glIsVertexArray(vertexArray);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
        return value[0];
    }

    public void pauseTransformFeedback() {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glPauseTransformFeedback();
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void readBuffer(final int src) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glReadBuffer(src);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void renderbufferStorageMultisample(final int target, final int samples, final int internalFormat, final int width, final int height) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glRenderbufferStorageMultisample(target, samples, internalFormat, width, height);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void resumeTransformFeedback() {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glResumeTransformFeedback();
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void samplerParameteri(final int sampler, final int pname, final int param) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glSamplerParameteri(sampler, pname, param);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void samplerParameterf(final int sampler, final int pname, final float param) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glSamplerParameterf(sampler, pname, param);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texImage3D(final int target, final int level, final int internalformat, final int width, final int height, final int depth, final int border, final int format, final int type, final int offset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, offset);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texImage3D(final int target, final int level, final int internalformat, final int width, final int height, final int depth, final int border, final int format, final int type, final byte[] source) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {

                if (flipYWebGL) {
                    nativeFlipInPlace3D(source, bytesPerPixel(type,format) * width, height, depth);
                }
                ByteBuffer pixels = ByteBuffer.wrap(source);
                GLES30.glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, pixels);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public void texImage3D(final int target, final int level, final int internalformat, final int width, final int height, final int depth, final int border, final int format, final int type, final Bitmap source) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                ByteBuffer buffer = bufferFromBitmap(source, bytesPerPixel(type, format) * source.getWidth() ,flipYWebGL);
                GLES30.glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, buffer);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texImage3D(final int target, final int level, final int internalformat, final int width, final int height, final int depth, final int border, final int format, final int type, final ImageAsset asset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                nativeTexImage3DAsset(target, level, internalformat, asset.getWidth(), asset.getHeight(),depth,0,format, type, asset.nativeImageAsset, flipYWebGL);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public void texStorage2D(final int target, final int levels, final int internalformat, final int width, final int height) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glTexStorage2D(target, levels, internalformat, width, height);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texStorage3D(final int target, final int levels, final int internalformat, final int width, final int height, final int depth) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glTexStorage3D(target, levels, internalformat, width, height, depth);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int type, final int offset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, offset);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int type, final Bitmap srcData) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                ByteBuffer storage = bufferFromBitmap(srcData, bytesPerPixel(type, format) * srcData.getWidth(), flipYWebGL);
                GLES30.glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, storage);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public void texSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int type, final ImageAsset asset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                nativeTexSubImage3DAsset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, asset.nativeImageAsset,flipYWebGL);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }


    public void texSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int type, final byte[] srcData) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, srcData, 0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void texSubImage3D(final int target, final int level, final int xoffset, final int yoffset, final int zoffset, final int width, final int height, final int depth, final int format, final int type, final byte[] srcData, final int srcOffset) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                if (flipYWebGL) {
                    nativeFlipInPlace3D(srcData,bytesPerPixel(type, format ) * width, height, depth);
                }
                ByteBuffer pixels = ByteBuffer.wrap(srcData);
                pixels.position(srcOffset);
                GLES30.glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void transformFeedbackVaryings(final int program, final String[] varyings, final int bufferMode) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glTransformFeedbackVaryings(program, varyings, bufferMode);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform1ui(final int location, final int v0) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform1ui(location, v0);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform2ui(final int location, final int v0, final int v1) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform2ui(location, v0, v1);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform3ui(final int location, final int v0, final int v1, final int v2) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform3ui(location, v0, v1, v2);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform4ui(final int location, final int v0, final int v1, final int v2, final int v3) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform4ui(location, v0, v1, v2, v3);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform1uiv(final int location, final int[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform1uiv(location, data.length, IntBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform2uiv(final int location, final int[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform2uiv(location, data.length, IntBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform3uiv(final int location, final int[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform3uiv(location, data.length, IntBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniform4uiv(final int location, final int[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniform4uiv(location, data.length, IntBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformBlockBinding(final int program, final int uniformBlockIndex, final int uniformBlockBinding) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix3x2fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix3x2fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix4x2fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix4x2fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix2x3fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix2x3fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix4x3fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix4x3fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix2x4fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix2x4fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void uniformMatrix3x4fv(final int location, final boolean transpose, final float[] data) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glUniformMatrix3x4fv(location, data.length, transpose, FloatBuffer.wrap(data));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void vertexAttribDivisor(final int index, final int divisor) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glVertexAttribDivisor(index, divisor);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void vertexAttribI4i(final int index, final int v0, final int v1, final int v2, final int v3) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glVertexAttribI4i(index, v0, v1, v2, v3);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void vertexAttribI4ui(final int index, final int v0, final int v1, final int v2, final int v3) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glVertexAttribI4ui(index, v0, v1, v2, v3);
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void vertexAttribI4iv(final int index, final int[] value) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glVertexAttribI4iv(index, IntBuffer.wrap(value));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    public void vertexAttribI4uiv(final int index, final int[] value) {
        final CountDownLatch lock = new CountDownLatch(1);
        runOnGLThread(new Runnable() {
            @Override
            public void run() {
                GLES30.glVertexAttribI4uiv(index, IntBuffer.wrap(value));
                lock.countDown();
            }
        });
        try {
            lock.await();
        } catch (InterruptedException ignored) {
        }
    }

    /* Getting GL parameter information */
    public final int READ_BUFFER = GLES30.GL_READ_BUFFER;

    public final int UNPACK_ROW_LENGTH = GLES30.GL_UNPACK_ROW_LENGTH;

    public final int UNPACK_SKIP_ROWS = GLES30.GL_UNPACK_SKIP_ROWS;

    public final int UNPACK_SKIP_PIXELS = GLES30.GL_UNPACK_SKIP_PIXELS;

    public final int PACK_ROW_LENGTH = GLES30.GL_PACK_ROW_LENGTH;

    public final int PACK_SKIP_ROWS = GLES30.GL_PACK_SKIP_ROWS;

    public final int PACK_SKIP_PIXELS = GLES30.GL_PACK_SKIP_PIXELS;

    public final int TEXTURE_BINDING_3D = GLES30.GL_TEXTURE_BINDING_3D;

    public final int UNPACK_SKIP_IMAGES = GLES30.GL_UNPACK_SKIP_IMAGES;

    public final int UNPACK_IMAGE_HEIGHT = GLES30.GL_UNPACK_IMAGE_HEIGHT;

    public final int MAX_3D_TEXTURE_SIZE = GLES30.GL_MAX_3D_TEXTURE_SIZE;

    public final int MAX_ELEMENTS_VERTICES = GLES30.GL_MAX_ELEMENTS_VERTICES;

    public final int MAX_ELEMENTS_INDICES = GLES30.GL_MAX_ELEMENTS_INDICES;

    public final int MAX_TEXTURE_LOD_BIAS = GLES30.GL_MAX_TEXTURE_LOD_BIAS;

    public final int MAX_FRAGMENT_UNIFORM_COMPONENTS = GLES30.GL_MAX_FRAGMENT_UNIFORM_COMPONENTS;

    public final int MAX_VERTEX_UNIFORM_COMPONENTS = GLES30.GL_MAX_VERTEX_UNIFORM_COMPONENTS;

    public final int MAX_ARRAY_TEXTURE_LAYERS = GLES30.GL_MAX_ARRAY_TEXTURE_LAYERS;

    public final int MIN_PROGRAM_TEXEL_OFFSET = GLES30.GL_MIN_PROGRAM_TEXEL_OFFSET;

    public final int MAX_PROGRAM_TEXEL_OFFSET = GLES30.GL_MAX_PROGRAM_TEXEL_OFFSET;

    public final int MAX_VARYING_COMPONENTS = GLES30.GL_MAX_VARYING_COMPONENTS;

    public final int FRAGMENT_SHADER_DERIVATIVE_HINT = GLES30.GL_FRAGMENT_SHADER_DERIVATIVE_HINT;

    public final int RASTERIZER_DISCARD = GLES30.GL_RASTERIZER_DISCARD;

    public final int VERTEX_ARRAY_BINDING = GLES30.GL_VERTEX_ARRAY_BINDING;

    public final int MAX_VERTEX_OUTPUT_COMPONENTS = GLES30.GL_MAX_VERTEX_OUTPUT_COMPONENTS;

    public final int MAX_FRAGMENT_INPUT_COMPONENTS = GLES30.GL_MAX_FRAGMENT_INPUT_COMPONENTS;

    public final int MAX_SERVER_WAIT_TIMEOUT = GLES30.GL_MAX_SERVER_WAIT_TIMEOUT;

    public final int MAX_ELEMENT_INDEX = GLES30.GL_MAX_ELEMENT_INDEX;
    /* Getting GL parameter information */


    /* Textures */

    public final int RED = GLES30.GL_RED;

    public final int RGB8 = GLES30.GL_RGB8;

    public final int RGBA8 = GLES30.GL_RGBA8;

    public final int RGB10_A2 = GLES30.GL_RGB10_A2;

    public final int TEXTURE_3D = GLES30.GL_TEXTURE_3D;

    public final int TEXTURE_WRAP_R = GLES30.GL_TEXTURE_WRAP_R;

    public final int TEXTURE_MIN_LOD = GLES30.GL_TEXTURE_MIN_LOD;

    public final int TEXTURE_MAX_LOD = GLES30.GL_TEXTURE_MAX_LOD;

    public final int TEXTURE_BASE_LEVEL = GLES30.GL_TEXTURE_BASE_LEVEL;

    public final int TEXTURE_MAX_LEVEL = GLES30.GL_TEXTURE_MAX_LEVEL;

    public final int TEXTURE_COMPARE_MODE = GLES30.GL_TEXTURE_COMPARE_MODE;

    public final int TEXTURE_COMPARE_FUNC = GLES30.GL_TEXTURE_COMPARE_FUNC;

    public final int SRGB = GLES30.GL_SRGB;

    public final int SRGB8 = GLES30.GL_SRGB8;

    public final int SRGB8_ALPHA8 = GLES30.GL_SRGB8_ALPHA8;

    public final int COMPARE_REF_TO_TEXTURE = GLES30.GL_COMPARE_REF_TO_TEXTURE;

    public final int RGBA32F = GLES30.GL_RGBA32F;

    public final int RGB32F = GLES30.GL_RGB32F;

    public final int RGBA16F = GLES30.GL_RGBA16F;

    public final int RGB16F = GLES30.GL_RGB16F;

    public final int TEXTURE_2D_ARRAY = GLES30.GL_TEXTURE_2D_ARRAY;

    public final int TEXTURE_BINDING_2D_ARRAY = GLES30.GL_TEXTURE_BINDING_2D_ARRAY;

    public final int R11F_G11F_B10F = GLES30.GL_R11F_G11F_B10F;

    public final int RGB9_E5 = GLES30.GL_RGB9_E5;

    public final int RGBA32UI = GLES30.GL_RGBA32UI;

    public final int RGB32UI = GLES30.GL_RGB32UI;

    public final int RGBA16UI = GLES30.GL_RGBA16UI;

    public final int RGB16UI = GLES30.GL_RGB16UI;

    public final int RGBA8UI = GLES30.GL_RGBA8UI;

    public final int RGB8UI = GLES30.GL_RGB8UI;

    public final int RGBA32I = GLES30.GL_RGBA32I;

    public final int RGB32I = GLES30.GL_RGB32I;

    public final int RGBA16I = GLES30.GL_RGBA16I;

    public final int RGB16I = GLES30.GL_RGB16I;

    public final int RGBA8I = GLES30.GL_RGBA8I;

    public final int RGB8I = GLES30.GL_RGB8I;

    public final int RED_INTEGER = GLES30.GL_RED_INTEGER;

    public final int RGB_INTEGER = GLES30.GL_RGB_INTEGER;

    public final int RGBA_INTEGER = GLES30.GL_RGBA_INTEGER;

    public final int R8 = GLES30.GL_R8;

    public final int RG8 = GLES30.GL_RG8;

    public final int R16F = GLES30.GL_R16F;

    public final int R32F = GLES30.GL_R32F;

    public final int RG16F = GLES30.GL_RG16F;

    public final int RG32F = GLES30.GL_RG32F;

    public final int R8I = GLES30.GL_R8I;

    public final int R8UI = GLES30.GL_R8UI;

    public final int R16I = GLES30.GL_R16I;

    public final int R16UI = GLES30.GL_R16UI;

    public final int R32I = GLES30.GL_R32I;

    public final int R32UI = GLES30.GL_R32UI;

    public final int RG8I = GLES30.GL_RG8I;

    public final int RG8UI = GLES30.GL_RG8UI;

    public final int RG16I = GLES30.GL_RG16I;

    public final int RG16UI = GLES30.GL_RG16UI;

    public final int RG32I = GLES30.GL_RG32I;

    public final int RG32UI = GLES30.GL_RG32UI;

    public final int R8_SNORM = GLES30.GL_R8_SNORM;

    public final int RG8_SNORM = GLES30.GL_RG8_SNORM;

    public final int RGB8_SNORM = GLES30.GL_RGB8_SNORM;

    public final int RGBA8_SNORM = GLES30.GL_RGBA8_SNORM;

    public final int RGB10_A2UI = GLES30.GL_RGB10_A2UI;

    public final int TEXTURE_IMMUTABLE_FORMAT = GLES30.GL_TEXTURE_IMMUTABLE_FORMAT;

    public final int TEXTURE_IMMUTABLE_LEVELS = GLES30.GL_TEXTURE_IMMUTABLE_LEVELS;

    /* Textures */


    /* Pixel types */

    public final int UNSIGNED_INT_2_10_10_10_REV = GLES30.GL_UNSIGNED_INT_2_10_10_10_REV;

    public final int UNSIGNED_INT_10F_11F_11F_REV = GLES30.GL_UNSIGNED_INT_10F_11F_11F_REV;

    public final int UNSIGNED_INT_5_9_9_9_REV = GLES30.GL_UNSIGNED_INT_5_9_9_9_REV;

    public final int FLOAT_32_UNSIGNED_INT_24_8_REV = GLES30.GL_FLOAT_32_UNSIGNED_INT_24_8_REV;

    public final int UNSIGNED_INT_24_8 = GLES30.GL_UNSIGNED_INT_24_8;

    public final int HALF_FLOAT = GLES30.GL_HALF_FLOAT;

    public final int RG = GLES30.GL_RG;

    public final int RG_INTEGER = GLES30.GL_RG_INTEGER;

    public final int INT_2_10_10_10_REV = GLES30.GL_INT_2_10_10_10_REV;

    /* Pixel types */



    /* Queries */

    public final int QUERY_RESULT_AVAILABLE = GLES30.GL_QUERY_RESULT_AVAILABLE;

    public final int QUERY_RESULT = GLES30.GL_QUERY_RESULT;

    public final int CURRENT_QUERY = GLES30.GL_CURRENT_QUERY;

    public final int ANY_SAMPLES_PASSED = GLES30.GL_ANY_SAMPLES_PASSED;

    public final int ANY_SAMPLES_PASSED_CONSERVATIVE = GLES30.GL_ANY_SAMPLES_PASSED_CONSERVATIVE;

    /* Queries */


    /* Draw buffers */

    public final int MAX_DRAW_BUFFERS = GLES30.GL_MAX_DRAW_BUFFERS;
    public final int DRAW_BUFFER0 = GLES30.GL_DRAW_BUFFER0;
    public final int DRAW_BUFFER1 = GLES30.GL_DRAW_BUFFER1;
    public final int DRAW_BUFFER2 = GLES30.GL_DRAW_BUFFER2;
    public final int DRAW_BUFFER3 = GLES30.GL_DRAW_BUFFER3;
    public final int DRAW_BUFFER4 = GLES30.GL_DRAW_BUFFER4;
    public final int DRAW_BUFFER5 = GLES30.GL_DRAW_BUFFER5;
    public final int DRAW_BUFFER6 = GLES30.GL_DRAW_BUFFER6;
    public final int DRAW_BUFFER7 = GLES30.GL_DRAW_BUFFER7;
    public final int DRAW_BUFFER8 = GLES30.GL_DRAW_BUFFER8;
    public final int DRAW_BUFFER9 = GLES30.GL_DRAW_BUFFER9;
    public final int DRAW_BUFFER10 = GLES30.GL_DRAW_BUFFER10;
    public final int DRAW_BUFFER11 = GLES30.GL_DRAW_BUFFER11;
    public final int DRAW_BUFFER12 = GLES30.GL_DRAW_BUFFER12;
    public final int DRAW_BUFFER13 = GLES30.GL_DRAW_BUFFER13;
    public final int DRAW_BUFFER14 = GLES30.GL_DRAW_BUFFER14;
    public final int DRAW_BUFFER15 = GLES30.GL_DRAW_BUFFER15;
    public final int MAX_COLOR_ATTACHMENTS = GLES30.GL_MAX_COLOR_ATTACHMENTS;

    public final int COLOR_ATTACHMENT1 = GLES30.GL_COLOR_ATTACHMENT1;

    public final int COLOR_ATTACHMENT2 = GLES30.GL_COLOR_ATTACHMENT2;

    public final int COLOR_ATTACHMENT3 = GLES30.GL_COLOR_ATTACHMENT3;

    public final int COLOR_ATTACHMENT4 = GLES30.GL_COLOR_ATTACHMENT4;

    public final int COLOR_ATTACHMENT5 = GLES30.GL_COLOR_ATTACHMENT5;

    public final int COLOR_ATTACHMENT6 = GLES30.GL_COLOR_ATTACHMENT6;

    public final int COLOR_ATTACHMENT7 = GLES30.GL_COLOR_ATTACHMENT7;

    public final int COLOR_ATTACHMENT8 = GLES30.GL_COLOR_ATTACHMENT8;

    public final int COLOR_ATTACHMENT9 = GLES30.GL_COLOR_ATTACHMENT9;

    public final int COLOR_ATTACHMENT10 = GLES30.GL_COLOR_ATTACHMENT10;

    public final int COLOR_ATTACHMENT11 = GLES30.GL_COLOR_ATTACHMENT11;

    public final int COLOR_ATTACHMENT12 = GLES30.GL_COLOR_ATTACHMENT12;

    public final int COLOR_ATTACHMENT13 = GLES30.GL_COLOR_ATTACHMENT13;

    public final int COLOR_ATTACHMENT14 = GLES30.GL_COLOR_ATTACHMENT14;

    public final int COLOR_ATTACHMENT15 = GLES30.GL_COLOR_ATTACHMENT15;

    /* Draw buffers */

    /* Samplers */

    public final int SAMPLER_3D = GLES30.GL_SAMPLER_3D;

    public final int SAMPLER_2D_SHADOW = GLES30.GL_SAMPLER_2D_SHADOW;

    public final int SAMPLER_2D_ARRAY = GLES30.GL_SAMPLER_2D_ARRAY;

    public final int SAMPLER_2D_ARRAY_SHADOW = GLES30.GL_SAMPLER_2D_ARRAY_SHADOW;

    public final int SAMPLER_CUBE_SHADOW = GLES30.GL_SAMPLER_CUBE_SHADOW;

    public final int INT_SAMPLER_2D = GLES30.GL_INT_SAMPLER_2D;

    public final int INT_SAMPLER_3D = GLES30.GL_INT_SAMPLER_3D;

    public final int INT_SAMPLER_CUBE = GLES30.GL_INT_SAMPLER_CUBE;

    public final int INT_SAMPLER_2D_ARRAY = GLES30.GL_INT_SAMPLER_2D_ARRAY;

    public final int UNSIGNED_INT_SAMPLER_2D = GLES30.GL_UNSIGNED_INT_SAMPLER_2D;

    public final int UNSIGNED_INT_SAMPLER_3D = GLES30.GL_UNSIGNED_INT_SAMPLER_3D;

    public final int UNSIGNED_INT_SAMPLER_CUBE = GLES30.GL_UNSIGNED_INT_SAMPLER_CUBE;

    public final int UNSIGNED_INT_SAMPLER_2D_ARRAY = GLES30.GL_UNSIGNED_INT_SAMPLER_2D_ARRAY;

    public final int MAX_SAMPLES = GLES30.GL_MAX_SAMPLES;

    public final int SAMPLER_BINDING = GLES30.GL_SAMPLER_BINDING;

    /* Samplers */


    /* Buffers */

    public final int PIXEL_PACK_BUFFER = GLES30.GL_PIXEL_PACK_BUFFER;

    public final int PIXEL_UNPACK_BUFFER = GLES30.GL_PIXEL_UNPACK_BUFFER;

    public final int PIXEL_PACK_BUFFER_BINDING = GLES30.GL_PIXEL_PACK_BUFFER_BINDING;

    public final int PIXEL_UNPACK_BUFFER_BINDING = GLES30.GL_PIXEL_UNPACK_BUFFER_BINDING;

    public final int COPY_READ_BUFFER = GLES30.GL_COPY_READ_BUFFER;

    public final int COPY_WRITE_BUFFER = GLES30.GL_COPY_WRITE_BUFFER;

    public final int COPY_READ_BUFFER_BINDING = GLES30.GL_COPY_READ_BUFFER_BINDING;

    public final int COPY_WRITE_BUFFER_BINDING = GLES30.GL_COPY_WRITE_BUFFER_BINDING;

    /* Buffers */


    /* Data types */

    public final int FLOAT_MAT2x3 = GLES30.GL_FLOAT_MAT2x3;

    public final int FLOAT_MAT2x4 = GLES30.GL_FLOAT_MAT2x4;

    public final int FLOAT_MAT3x2 = GLES30.GL_FLOAT_MAT3x2;

    public final int FLOAT_MAT3x4 = GLES30.GL_FLOAT_MAT3x4;

    public final int FLOAT_MAT4x2 = GLES30.GL_FLOAT_MAT4x2;

    public final int FLOAT_MAT4x3 = GLES30.GL_FLOAT_MAT4x3;

    public final int UNSIGNED_INT_VEC2 = GLES30.GL_UNSIGNED_INT_VEC2;

    public final int UNSIGNED_INT_VEC3 = GLES30.GL_UNSIGNED_INT_VEC3;
    public final int UNSIGNED_INT_VEC4 = GLES30.GL_UNSIGNED_INT_VEC4;
    public final int UNSIGNED_NORMALIZED = GLES30.GL_UNSIGNED_NORMALIZED;
    public final int SIGNED_NORMALIZED = GLES30.GL_SIGNED_NORMALIZED;


    /* Data types */

    /* Vertex attributes */
    public final int VERTEX_ATTRIB_ARRAY_INTEGER = GLES30.GL_VERTEX_ATTRIB_ARRAY_INTEGER;

    public final int VERTEX_ATTRIB_ARRAY_DIVISOR = GLES30.GL_VERTEX_ATTRIB_ARRAY_DIVISOR;

    /* Vertex attributes */


    /* Transform feedback */

    public final int TRANSFORM_FEEDBACK_BUFFER_MODE = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_MODE;
    public final int MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS = GLES30.GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS;
    public final int TRANSFORM_FEEDBACK_VARYINGS = GLES30.GL_TRANSFORM_FEEDBACK_VARYINGS;

    public final int TRANSFORM_FEEDBACK_BUFFER_START = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_START;

    public final int TRANSFORM_FEEDBACK_BUFFER_SIZE = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_SIZE;

    public final int TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN = GLES30.GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN;

    public final int MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS = GLES30.GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS;

    public final int MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS = GLES30.GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS;

    public final int INTERLEAVED_ATTRIBS = GLES30.GL_INTERLEAVED_ATTRIBS;

    public final int SEPARATE_ATTRIBS = GLES30.GL_SEPARATE_ATTRIBS;

    public final int TRANSFORM_FEEDBACK_BUFFER = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER;

    public final int TRANSFORM_FEEDBACK_BUFFER_BINDING = GLES30.GL_TRANSFORM_FEEDBACK_BUFFER_BINDING;

    public final int TRANSFORM_FEEDBACK = GLES30.GL_TRANSFORM_FEEDBACK;

    public final int TRANSFORM_FEEDBACK_PAUSED = GLES30.GL_TRANSFORM_FEEDBACK_PAUSED;

    public final int TRANSFORM_FEEDBACK_ACTIVE = GLES30.GL_TRANSFORM_FEEDBACK_ACTIVE;

    public final int TRANSFORM_FEEDBACK_BINDING = GLES30.GL_TRANSFORM_FEEDBACK_BINDING;

    /* Transform feedback */

    /* Framebuffers and renderbuffers */

    public final int FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING = GLES30.GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING;
    public final int FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE;

    public final int FRAMEBUFFER_ATTACHMENT_RED_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE;
    public final int FRAMEBUFFER_ATTACHMENT_GREEN_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE;
    public final int FRAMEBUFFER_ATTACHMENT_BLUE_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE;
    public final int FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE;

    public final int FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE;
    public final int FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE = GLES30.GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE;

    public final int FRAMEBUFFER_DEFAULT = GLES30.GL_FRAMEBUFFER_DEFAULT;
    public final int DEPTH_STENCIL_ATTACHMENT = GLES30.GL_DEPTH_STENCIL_ATTACHMENT;
    public final int DEPTH_STENCIL = GLES30.GL_DEPTH_STENCIL;
    public final int DEPTH24_STENCIL8 = GLES30.GL_DEPTH24_STENCIL8;

    public final int DRAW_FRAMEBUFFER_BINDING = GLES30.GL_DRAW_FRAMEBUFFER_BINDING;

    public final int READ_FRAMEBUFFER = GLES30.GL_READ_FRAMEBUFFER;

    public final int DRAW_FRAMEBUFFER = GLES30.GL_DRAW_FRAMEBUFFER;

    public final int READ_FRAMEBUFFER_BINDING = GLES30.GL_READ_FRAMEBUFFER_BINDING;

    public final int RENDERBUFFER_SAMPLES = GLES30.GL_RENDERBUFFER_SAMPLES;

    public final int FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER = GLES30.GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER;

    public  final int FRAMEBUFFER_INCOMPLETE_MULTISAMPLE = GLES30.GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE;

    /* Framebuffers and renderbuffers */


    /* Uniforms */

    public final int UNIFORM_BUFFER = GLES30.GL_UNIFORM_BUFFER;
    public final int UNIFORM_BUFFER_BINDING = GLES30.GL_UNIFORM_BUFFER_BINDING;

    public final int UNIFORM_BUFFER_START = GLES30.GL_UNIFORM_BUFFER_START;

    public final int UNIFORM_BUFFER_SIZE = GLES30.GL_UNIFORM_BUFFER_SIZE;

    public final int MAX_VERTEX_UNIFORM_BLOCKS = GLES30.GL_MAX_VERTEX_UNIFORM_BLOCKS;

    public final int MAX_FRAGMENT_UNIFORM_BLOCKS = GLES30.GL_MAX_FRAGMENT_UNIFORM_BLOCKS;

    public final int MAX_COMBINED_UNIFORM_BLOCKS = GLES30.GL_MAX_COMBINED_UNIFORM_BLOCKS;

    public final int MAX_UNIFORM_BUFFER_BINDINGS = GLES30.GL_MAX_UNIFORM_BUFFER_BINDINGS;

    public final int MAX_UNIFORM_BLOCK_SIZE = GLES30.GL_MAX_UNIFORM_BLOCK_SIZE;

    public final int MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS = GLES30.GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS;

    public final int MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS = GLES30.GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS;

    public final int UNIFORM_BUFFER_OFFSET_ALIGNMENT = GLES30.GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT;

    public final int ACTIVE_UNIFORM_BLOCKS = GLES30.GL_ACTIVE_UNIFORM_BLOCKS;

    public final int UNIFORM_TYPE = GLES30.GL_UNIFORM_TYPE;

    public final int UNIFORM_SIZE = GLES30.GL_UNIFORM_SIZE;

    public final int UNIFORM_BLOCK_INDEX = GLES30.GL_UNIFORM_BLOCK_INDEX;

    public final int UNIFORM_OFFSET = GLES30.GL_UNIFORM_OFFSET;

    public final int UNIFORM_ARRAY_STRIDE = GLES30.GL_UNIFORM_ARRAY_STRIDE;

    public final int UNIFORM_MATRIX_STRIDE = GLES30.GL_UNIFORM_MATRIX_STRIDE;

    public final int UNIFORM_IS_ROW_MAJOR = GLES30.GL_UNIFORM_IS_ROW_MAJOR;

    public final int UNIFORM_BLOCK_BINDING = GLES30.GL_UNIFORM_BLOCK_BINDING;

    public final int UNIFORM_BLOCK_DATA_SIZE = GLES30.GL_UNIFORM_BLOCK_DATA_SIZE;

    public final int UNIFORM_BLOCK_ACTIVE_UNIFORMS = GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS;

    public final int UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES = GLES30.GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES;

    public final int UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER = GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER;

    public final int UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER = GLES30.GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER;


    /* Uniforms */

    /* Sync objects */

    public final int OBJECT_TYPE = GLES30.GL_OBJECT_TYPE;

    public final int SYNC_CONDITION = GLES30.GL_SYNC_CONDITION;

    public final int SYNC_STATUS = GLES30.GL_SYNC_STATUS;

    public final int SYNC_FLAGS = GLES30.GL_SYNC_FLAGS;

    public final int SYNC_FENCE = GLES30.GL_SYNC_FENCE;

    public final int SYNC_GPU_COMMANDS_COMPLETE = GLES30.GL_SYNC_GPU_COMMANDS_COMPLETE;

    public final int UNSIGNALED = GLES30.GL_UNSIGNALED;

    public final int SIGNALED = GLES30.GL_SIGNALED;

    public final int ALREADY_SIGNALED = GLES30.GL_ALREADY_SIGNALED;

    public final int TIMEOUT_EXPIRED = GLES30.GL_TIMEOUT_EXPIRED;


    public final int CONDITION_SATISFIED = GLES30.GL_CONDITION_SATISFIED;

    public final int WAIT_FAILED = GLES30.GL_WAIT_FAILED;

    public final int SYNC_FLUSH_COMMANDS_BIT = GLES30.GL_SYNC_FLUSH_COMMANDS_BIT;

    /* Sync objects */

    /* Miscellaneous constants */

    public final int COLOR = GLES30.GL_COLOR;

    public final int DEPTH = GLES30.GL_DEPTH;

    public final int STENCIL = GLES30.GL_STENCIL;

    public final int MIN = GLES30.GL_MIN;

    public final int MAX = GLES30.GL_MAX;

    public final int DEPTH_COMPONENT24 = GLES30.GL_DEPTH_COMPONENT24;

    public final int STREAM_READ = GLES30.GL_STREAM_READ;

    public final int STREAM_COPY = GLES30.GL_STREAM_COPY;

    public final int STATIC_READ = GLES30.GL_STATIC_READ;

    public final int STATIC_COPY = GLES30.GL_STATIC_COPY;

    public final int DYNAMIC_READ = GLES30.GL_DYNAMIC_READ;

    public final int DYNAMIC_COPY = GLES30.GL_DYNAMIC_COPY;

    public final int DEPTH_COMPONENT32F = GLES30.GL_DEPTH_COMPONENT32F;

    public final int DEPTH32F_STENCIL8 = GLES30.GL_DEPTH32F_STENCIL8;

    public final int INVALID_INDEX = GLES30.GL_INVALID_INDEX;

    public final long TIMEOUT_IGNORED = GLES30.GL_TIMEOUT_IGNORED;

    public final int MAX_CLIENT_WAIT_TIMEOUT_WEBGL = 0x9247;

    /* Miscellaneous constants */
}

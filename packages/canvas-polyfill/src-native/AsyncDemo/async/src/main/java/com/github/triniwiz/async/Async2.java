package com.github.triniwiz.async;

import android.os.Handler;
import android.os.HandlerThread;

import androidx.annotation.Nullable;

import okhttp3.*;
import okhttp3.internal.http2.ErrorCode;
import okhttp3.internal.http2.StreamResetException;
import okio.*;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.net.SocketTimeoutException;
import java.nio.ByteBuffer;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.Iterator;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.io.FileOutputStream;
import java.io.OutputStream;
import java.util.HashMap;
import java.util.Map;

public class Async2 {
	private static ExecutorService executor = Executors.newCachedThreadPool();
	private static Handler handler;
	static final Map<String, String> b64Extensions = new HashMap<>();

	static {
		b64Extensions.put("/", "jpg");
		b64Extensions.put("i", "png");
		b64Extensions.put("R", "gif");
		b64Extensions.put("U", "webp");

		HandlerThread handlerThread = new HandlerThread("TNSAsyncHttp", android.os.Process.THREAD_PRIORITY_BACKGROUND);
		handlerThread.start();
		handler = new Handler(handlerThread.getLooper());
	}


	public static class Base64 {
		static String b64WithoutPrefix(String b64) {
			return b64.split(",")[1];
		}

		static String getMIMEForBase64String(String b64) throws Exception {
			String input = b64;
			if (b64.contains(",")) {
				input = b64WithoutPrefix(b64);
			}
			char first = input.charAt(0);
			String mime = b64Extensions.get(String.valueOf(first));
			if (mime == null) {
				throw new Exception("Unknown Base64 MIME type: " + b64);
			}
			return mime;
		}

		static String getUUID() {
			return java.util.UUID.randomUUID().toString();
		}

		public interface Callback {
			void success(Object response);

			void error(Object error, String message);
		}

		public static void base64ToFile(final String base64, final String path, final Callback callback) {
			executor.submit(new Runnable() {
				@Override
				public void run() {
					String MIME;
					try {
						MIME = getMIMEForBase64String(base64);
					} catch (Exception e) {
						callback.error(e, e.getMessage());
						return;
					}
					String localUri = path + getUUID() + "-b64image." + MIME;
					File file = new File(localUri);
					try {
						FileOutputStream os = new FileOutputStream(file);
						os.write(android.util.Base64.decode(
							base64,
							android.util.Base64.DEFAULT
						));
						os.flush();
						os.close();
						callback.success(file.getAbsolutePath());
					} catch (FileNotFoundException e) {
						callback.error(e, e.getMessage());
					} catch (IOException e) {
						callback.error(e, e.getMessage());
					}
				}
			});
		}
	}

	public static void runInBackground(final Runnable runnable) {
		executor.submit(new Runnable() {
			@Override
			public void run() {
				Thread thread = new Thread(runnable);
				thread.start();
			}
		});
	}

	static final class ByteArrayOutputStream2 extends ByteArrayOutputStream {
		public ByteArrayOutputStream2() {
			super();
		}

		public ByteArrayOutputStream2(int size) {
			super(size);
		}

		/**
		 * Returns the internal buffer of this ByteArrayOutputStream, without copying.
		 */
		public synchronized byte[] buf() {
			return this.buf;
		}
	}

	public static class Http {
		private static ConcurrentHashMap<String, CallOptions> callMap = new ConcurrentHashMap<>();
		private static ConcurrentHashMap<String, DownloadCallOptions> downloadCallMap = new ConcurrentHashMap<>();
		private static ArrayList<String> cancelList = new ArrayList<>();

		interface ProgressListener {
			void onProgress(long loaded, long total);
		}

		public static class Result {
			public Object content;
			public String url;
			public ArrayList<KeyValuePair> headers;
			public String contentText;

			Result() {
			}
		}

		public static class FileResult {
			public String filePath;
			public String url;
			public ArrayList<KeyValuePair> headers;

			FileResult() {
			}
		}


		static class ProgressRequestBody extends RequestBody {
			RequestBody body;
			ProgressListener listener;

			ProgressRequestBody(final RequestBody body, final ProgressListener listener) {
				this.body = body;
				this.listener = listener;
			}

			@Nullable
			@Override
			public MediaType contentType() {
				return body.contentType();
			}

			@Override
			public long contentLength() throws IOException {
				return body.contentLength();
			}

			@Override
			public void writeTo(BufferedSink sink) throws IOException {
				BufferedSink bufferedSink;
				bufferedSink = Okio.buffer(forwardingSink(sink));
				body.writeTo(bufferedSink);
				bufferedSink.close();
			}

			ForwardingSink forwardingSink(BufferedSink sink) {
				long contentLength = -1;
				try {
					contentLength = contentLength();
				} catch (IOException ignored) {

				}
				final long length = contentLength;
				return new ForwardingSink(sink) {
					long totalBytesWrite = 0;

					@Override
					public void write(Buffer source, long byteCount) throws IOException {
						super.write(source, byteCount);
						if (listener != null) {
							totalBytesWrite += byteCount != -1 ? byteCount : 0;
							if (length < 0) {
								listener.onProgress(totalBytesWrite, length);
							}
							if (byteCount >= 0) {
								listener.onProgress(totalBytesWrite, length);
							}
						}
					}
				};
			}
		}

		static class ProgressResponseBody extends ResponseBody {
			ResponseBody body;
			ProgressListener listener;
			BufferedSource bufferedSource;
			Headers headers;

			ProgressResponseBody(ResponseBody body, ProgressListener listener) {
				this.body = body;
				this.listener = listener;
			}

			ProgressResponseBody(ResponseBody body, Headers headers, ProgressListener listener) {
				this.body = body;
				this.listener = listener;
				this.headers = headers;
			}

			@Nullable
			@Override
			public MediaType contentType() {
				return body.contentType();
			}

			@Override
			public long contentLength() {
				return body.contentLength();
			}

			@Override
			public BufferedSource source() {
				if (bufferedSource == null) {
					bufferedSource = Okio.buffer(source(body.source()));
				}
				return bufferedSource;
			}

			private Source source(Source source) {
				return new ForwardingSource(body.source()) {
					long totalBytesRead = 0;
					long length = contentLength();

					@Override
					public long read(Buffer sink, long byteCount) throws IOException {
						long bytesRead = super.read(sink, byteCount);
						if (listener != null) {
							totalBytesRead += bytesRead != -1 ? bytesRead : 0;

							if (length < 0) {
								listener.onProgress(totalBytesRead, length);
							}
							if (bytesRead >= 0) {
								listener.onProgress(totalBytesRead, length);
							}
						}
						return bytesRead;
					}
				};
			}
		}

		public interface Callback {
			void onComplete(Object result);

			void onError(String error, Exception e);

			void onProgress(boolean lengthComputable, long loaded, long total);

			void onHeaders(ArrayList<Http.KeyValuePair> headers, int status);

			void onCancel(Object result);

			void onLoading();

			void onTimeout();
		}

		static class HttpEventListener extends EventListener {
			@Override
			public void requestHeadersEnd(Call call, Request request) {
				super.requestHeadersEnd(call, request);
			}

			@Override
			public void responseHeadersEnd(Call call, Response response) {
				for (String key : callMap.keySet()) {
					CallOptions options = callMap.get(key);
					if (options != null && options.call.equals(call)) {
						Headers responseHeaders = response.headers();
						ArrayList<KeyValuePair> headers = new ArrayList<>();
						for (String value : responseHeaders.names()) {
							headers.add(new KeyValuePair(value, responseHeaders.get(value)));
						}
						options.callback.onHeaders(headers, response.code());
					}
				}
				super.responseHeadersEnd(call, response);
			}

			@Override
			public void responseBodyStart(Call call) {
				String method = call.request().method();
				if (!method.equals("POST") && !method.equals("PUT")) {
					for (String key : callMap.keySet()) {
						CallOptions options = callMap.get(key);
						if (options != null && options.call.equals(call)) {
							options.callback.onLoading();
						}
					}
				}
				super.responseBodyStart(call);
			}

			@Override
			public void responseBodyEnd(Call call, long byteCount) {
				super.responseBodyEnd(call, byteCount);
			}

			@Override
			public void requestBodyStart(Call call) {
				String method = call.request().method();
				if (method.equals("POST") || method.equals("PUT")) {
					for (String key : callMap.keySet()) {
						CallOptions options = callMap.get(key);
						if (options != null && options.call.equals(call)) {
							options.callback.onLoading();
						}
					}
				}
				super.requestBodyStart(call);
			}

			@Override
			public void requestBodyEnd(Call call, long byteCount) {
				super.requestBodyEnd(call, byteCount);
			}
		}

		private static boolean isTextType(@Nullable String contentType) {
			boolean isTextType = false;
			if (contentType != null) {
				String[] textTypes = {
					"text/plain",
					"application/xml",
					"application/rss+xml",
					"text/html",
					"text/xml"};
				for (String type : textTypes) {
					isTextType = contentType.contains(type);
					if (isTextType) {
						break;
					}
				}
			}
			return isTextType;
		}

		public static String makeRequest(final RequestOptions options, final Http.Callback callback) {
			UUID uuid = UUID.randomUUID();
			final String id = uuid.toString();
			Async2.executor.submit(new Runnable() {
				@Override
				public void run() {
					OkHttpClient.Builder builder = new OkHttpClient.Builder();
					builder.eventListener(new HttpEventListener());
					builder.followRedirects(!options.dontFollowRedirects);
					if (options.timeout > -1) {
						builder.connectTimeout(options.timeout, TimeUnit.MILLISECONDS);
						builder.readTimeout(options.timeout, TimeUnit.MILLISECONDS);
						builder.writeTimeout(options.timeout, TimeUnit.MILLISECONDS);
					}
					if (options.username != null && options.password != null) {
						builder.authenticator(new Authenticator() {
							@Nullable
							@Override
							public Request authenticate(Route route, Response response) throws IOException {
								if (response.request().header("Authorization") != null) {
									return null;
								}
								return response.request().newBuilder()
									.header("Authorization", Credentials.basic(options.username, options.password))
									.build();
							}
						});
					}
					Request.Builder request = new Request.Builder();
					request.url(options.url);
					String contentType = "text/html";
					if (options.headers != null) {
						for (KeyValuePair pair : options.headers) {
							request.addHeader(pair.key, pair.value);
							if (pair.key.equals("Content-Type")) {
								contentType = pair.value;
							}
						}
					}
					RequestBody body = null;
					boolean isPostPutOrPatch = options.method.equals("POST") || options.method.equals("PUT") || options.method.equals("PATCH");
					if (isPostPutOrPatch) {
						if (options.content instanceof File) {

						} else if (options.content instanceof String) {
							if (contentType.equals("application/x-www-form-urlencoded")) {
								JSONTokener tokener = new JSONTokener((String) options.content);
								Object value = null;
								try {
									value = tokener.nextValue();
								} catch (JSONException e) {
									e.printStackTrace();
								}
								if (value instanceof JSONObject) {
									FormBody.Builder formBody = null;
									if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.KITKAT) {
										formBody = new FormBody.Builder(StandardCharsets.UTF_8);
									} else {
										formBody = new FormBody.Builder(java.nio.charset.Charset.forName("UTF-8"));
									}

									for (Iterator<String> it = ((JSONObject) value).keys(); it.hasNext(); ) {
										String key = it.next();
										formBody.addEncoded(key, String.valueOf(((JSONObject) value).opt(key)));
									}
									body = new ProgressRequestBody(formBody.build(), new ProgressListener() {
										@Override
										public void onProgress(long loaded, long total) {
											callback.onProgress(total > -1, loaded, total);
										}
									});
								} else {
									body = new ProgressRequestBody(RequestBody.create(MediaType.parse(contentType), (String) options.content), new ProgressListener() {
										@Override
										public void onProgress(long loaded, long total) {
											callback.onProgress(total > -1, loaded, total);
										}
									});
								}

							} else {
								body = new ProgressRequestBody(RequestBody.create(MediaType.parse(contentType), (String) options.content), new ProgressListener() {
									@Override
									public void onProgress(long loaded, long total) {
										callback.onProgress(total > -1, loaded, total);
									}
								});
							}

						} else if (options.content instanceof JSONObject || options.content instanceof JSONArray) {
							if (contentType.equals("application/x-www-form-urlencoded")) {
								if (options.content instanceof JSONObject) {
									FormBody.Builder formBody = null;
									if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.KITKAT) {
										formBody = new FormBody.Builder(StandardCharsets.UTF_8);
									} else {
										formBody = new FormBody.Builder(java.nio.charset.Charset.forName("UTF-8"));
									}

									for (Iterator<String> it = (((JSONObject) options.content).keys()); it.hasNext(); ) {
										String key = it.next();
										formBody.addEncoded(key, String.valueOf(((JSONObject) options.content).opt(key)));
									}
									body = new ProgressRequestBody(formBody.build(), new ProgressListener() {
										@Override
										public void onProgress(long loaded, long total) {
											callback.onProgress(total > -1, loaded, total);
										}
									});
								} else {
									body = new ProgressRequestBody(RequestBody.create(MediaType.parse(contentType), options.content.toString()), new ProgressListener() {
										@Override
										public void onProgress(long loaded, long total) {
											callback.onProgress(total > -1, loaded, total);
										}
									});
								}

							} else {
								body = new ProgressRequestBody(RequestBody.create(MediaType.parse(contentType), options.content.toString()), new ProgressListener() {
									@Override
									public void onProgress(long loaded, long total) {
										callback.onProgress(total > -1, loaded, total);
									}
								});
							}

						} else {
							body = RequestBody.create(null, "");
						}
					}

					request.method(options.method, body);
					OkHttpClient client = builder.build();
					Call call = client.newCall(request.build());
					call.enqueue(new okhttp3.Callback() {
						ByteArrayOutputStream2 stream;

						@Override
						public void onFailure(Call call, IOException e) {
							if (call.isCanceled()) {
								Result result = new Result();
								if (stream != null) {
									result.content = ByteBuffer.wrap(stream.buf());
									result.url = call.request().url().toString();
									callback.onCancel(result);
								} else {
									result.content = ByteBuffer.allocate(0);
									result.url = call.request().url().toString();
									callback.onCancel(result);
								}
							} else if (e instanceof SocketTimeoutException) {
								callback.onTimeout();
							} else {
								callback.onError(e.getMessage(), e);
							}
							callMap.remove(id);
						}

						@Override
						public void onResponse(Call call, Response response) throws IOException {
							ProgressResponseBody responseBody = new ProgressResponseBody(response.body(), new ProgressListener() {
								@Override
								public void onProgress(long loaded, long total) {
									if (!options.method.equals("POST") && !options.method.equals("PUT")) {
										callback.onProgress(total > -1, loaded, total);
									}
								}
							});
							String contentType = response.header("Content-Type");
							if (contentType == null) {
								contentType = response.header("content-type");
							}
							String acceptHeader;

							if (contentType == null) {
								acceptHeader = response.header("Accept");
								if (acceptHeader == null) {
									acceptHeader = response.header("accept");
								}
							} else {
								acceptHeader = contentType;
							}
							String returnType = "text/plain";
							if (acceptHeader != null) {
								String[] acceptValues = acceptHeader.split(",");
								ArrayList<String> quality = new ArrayList<>();
								ArrayList<String> defaultQuality = new ArrayList<>();
								ArrayList<String> customQuality = new ArrayList<>();
								for (String value : acceptValues) {
									if (value.contains(";q=")) {
										customQuality.add(value);
									} else {
										defaultQuality.add(value);
									}
								}
								Collections.sort(customQuality, new QualitySort());
								quality.addAll(defaultQuality);
								quality.addAll(customQuality);
								returnType = quality.get(0);
							}
							BufferedSource source = responseBody.source();
							stream = new ByteArrayOutputStream2();
							Sink sink = Okio.sink(stream);
							Result result = new Result();
							result.contentText = "";
							result.url = response.request().url().toString();
							result.headers = new ArrayList<>();
							try {
								source.readAll(sink);
								if (isTextType(returnType)) {
									result.headers.add(new KeyValuePair("Content-Type", returnType));
									result.content = stream.toString();
									result.contentText = (String) result.content;
									callback.onComplete(result);
								} else if (returnType.contains("application/json")) {
									String returnValue = stream.toString();
									JSONTokener tokener = new JSONTokener(returnValue);
									Object value = tokener.nextValue();
									if (value instanceof JSONObject || value instanceof JSONArray) {
										result.headers.add(new KeyValuePair("Content-Type", returnType));
										result.content = value;
										result.contentText = returnValue;
									} else {
										result.headers.add(new KeyValuePair("Content-Type", "text/plain"));
										result.content = returnValue;
										result.contentText = returnValue;
									}
									callback.onComplete(result);
								} else {
									result.headers.add(new KeyValuePair("Content-Type", "application/octet-stream"));
									result.content = ByteBuffer.wrap(stream.buf());
									callback.onComplete(result);
								}
							} catch (StreamResetException e) {
								if (e.errorCode == ErrorCode.CANCEL) {
									if (isTextType(returnType)) {
										result.content = stream.toString();
										callback.onCancel(result);
									} else if (returnType.contains("application/json")) {
										String returnValue = stream.toString();
										JSONTokener tokener = new JSONTokener(returnValue);
										try {
											Object value = tokener.nextValue();
											if (value instanceof JSONObject || value instanceof JSONArray) {
												result.headers.add(new KeyValuePair("Content-Type", returnType));
												result.content = value;
												result.contentText = returnValue;
											} else {
												result.headers.add(new KeyValuePair("Content-Type", "text/plain"));
												result.content = returnValue;
												result.contentText = returnValue;
											}
											callback.onCancel(result);
										} catch (JSONException e1) {
											callback.onError(e1.getMessage(), e1);
										}
									} else {
										result.headers.add(new KeyValuePair("Content-Type", "application/octet-stream"));
										result.content = ByteBuffer.wrap(stream.buf());
										callback.onCancel(result);
									}
								} else {
									callback.onError(e.getMessage(), e);
								}

							} catch (SocketTimeoutException e) {
								callback.onTimeout();
							} catch (Exception e) {
								callback.onError(e.getMessage(), e);
							}
							responseBody.close();
							callMap.remove(id);
						}
					});
					callMap.put(id, new CallOptions(call, options, callback));
					for (String cancelId : cancelList) {
						if (id.equals(cancelId)) {
							call.cancel();
							callMap.remove(cancelId);
							cancelList.remove(cancelId);
						}
					}
				}
			});
			return id;
		}

		public static String getFileRequest(final DownloadRequestOptions options, final Http.Callback callback) {
			UUID uuid = UUID.randomUUID();
			final String id = uuid.toString();
			Async2.executor.submit(new Runnable() {
				@Override
				public void run() {
					OkHttpClient.Builder builder = new OkHttpClient.Builder();
					builder.eventListener(new HttpEventListener());
					builder.followRedirects(!options.dontFollowRedirects);
					if (options.timeout > -1) {
						builder.connectTimeout(options.timeout, TimeUnit.MILLISECONDS);
						builder.readTimeout(options.timeout, TimeUnit.MILLISECONDS);
						builder.writeTimeout(options.timeout, TimeUnit.MILLISECONDS);
					}
					if (options.username != null && options.password != null) {
						builder.authenticator(new Authenticator() {
							@Nullable
							@Override
							public Request authenticate(Route route, Response response) throws IOException {
								if (response.request().header("Authorization") != null) {
									return null;
								}
								return response.request().newBuilder()
									.header("Authorization", Credentials.basic(options.username, options.password))
									.build();
							}
						});
					}
					Request.Builder request = new Request.Builder();
					request.url(options.url);
					OkHttpClient client = builder.build();
					Call call = client.newCall(request.build());
					call.enqueue(new okhttp3.Callback() {
						@Override
						public void onFailure(Call call, IOException e) {
							if (call.isCanceled()) {
								FileResult result = new FileResult();
								callback.onCancel(result);
							} else if (e instanceof SocketTimeoutException) {
								callback.onTimeout();
							} else {
								callback.onError(e.getMessage(), e);
							}
							downloadCallMap.remove(id);
						}

						@Override
						public void onResponse(Call call, Response response) throws IOException {
							ProgressResponseBody responseBody = new ProgressResponseBody(response.body(), response.headers(), new ProgressListener() {
								@Override
								public void onProgress(long loaded, long total) {
									callback.onProgress(total > -1, loaded, total);
								}
							});

							BufferedSource bufferedSource = responseBody.source();
							File file = new File(options.filePath);
							BufferedSink sink = null;
							try {
								sink = Okio.buffer(Okio.sink(file));
								sink.writeAll(bufferedSource);
								FileResult result = new FileResult();
								result.url = response.request().url().toString();
								result.headers = new ArrayList<>();
								result.filePath = file.getAbsolutePath();
								sink.close();
								callback.onComplete(result);
							} catch (StreamResetException e) {
								if (e.errorCode == ErrorCode.CANCEL) {
									FileResult result = new FileResult();
									callback.onCancel(result);
								} else {
									callback.onError(e.getMessage(), e);
								}

							} catch (SocketTimeoutException e) {
								callback.onTimeout();
							} catch (Exception e) {
								callback.onError(e.getMessage(), e);
							} finally {
								if (sink != null) {
									try {
										sink.close();
									} catch (IOException e) {
									}
								}
								if (bufferedSource != null) {
									try {
										bufferedSource.close();
									} catch (IOException e) {
									}
								}
								responseBody.close();
								downloadCallMap.remove(id);
							}
						}
					});
					downloadCallMap.put(id, new DownloadCallOptions(call, options, callback));
					for (String cancelId : cancelList) {
						if (id.equals(cancelId)) {
							call.cancel();
							downloadCallMap.remove(cancelId);
							cancelList.remove(cancelId);
						}
					}
				}
			});
			return id;
		}

		public static void cancelRequest(final String id) {
			if (id != null) {
				Async2.executor.submit(new Runnable() {
					@Override
					public void run() {
						CallOptions pair = callMap.get(id);
						DownloadCallOptions downloadPair = downloadCallMap.get(id);

						if (pair != null) {
							pair.call.cancel();
						}

						if (downloadPair != null) {
							downloadPair.call.cancel();
						}

						if (pair == null && downloadPair == null) {
							cancelList.add(id);
						}
					}
				});
			}
		}

		static class CallOptions {
			public Call call;
			public Callback callback;
			public RequestOptions options;

			CallOptions(Call call, RequestOptions options, Callback callback) {
				this.call = call;
				this.options = options;
				this.callback = callback;
			}
		}

		static class DownloadCallOptions {
			public Call call;
			public Callback callback;
			public DownloadRequestOptions options;

			DownloadCallOptions(Call call, DownloadRequestOptions options, Callback callback) {
				this.call = call;
				this.options = options;
				this.callback = callback;
			}
		}

		public static class KeyValuePair {
			public String key;
			public String value;

			public KeyValuePair(String key, String value) {
				this.key = key;
				this.value = value;
			}
		}

		public static class RequestOptions {
			public String url;
			public String method;
			public ArrayList<KeyValuePair> headers;
			public Object content;
			public int timeout = -1;
			public boolean dontFollowRedirects = false;
			public String username;
			public String password;
		}

		public static class DownloadRequestOptions {
			public String url;
			public String filePath;
			public ArrayList<KeyValuePair> headers;
			public int timeout = -1;
			public boolean dontFollowRedirects = false;
			public String username;
			public String password;
		}
	}


	static class QualitySort implements Comparator<String> {
		@Override
		public int compare(String a, String b) {
			float a_quality = Float.valueOf(a.substring(a.indexOf(";q=")).replace(";q=", ""));
			float b_quality = Float.valueOf(b.substring(b.indexOf(";q=")).replace(";q=", ""));
			return (int) (b_quality - a_quality);
		}
	}

	public static class FileManager {
		public interface Callback {
			void onError(String error, Exception e);

			void onComplete(@Nullable Object object);
		}

		public static class Options {
			public boolean asStream = false;

			public Options() {
			}
		}

		public static void writeFile(final byte[] bytes, final String path, final Callback callback) {
			Async2.executor.submit(new Runnable() {
				@Override
				public void run() {
					File file = new File(path);
					FileOutputStream stream = null;
					try {
						stream = new FileOutputStream(file);
						stream.write(bytes);
						callback.onComplete(null);
					} catch (FileNotFoundException e) {
						callback.onError(e.getMessage(), e);
					} catch (IOException e) {
						callback.onError(e.getMessage(), e);
					} finally {
						if (stream != null) {
							try {
								stream.flush();
								stream.close();
							} catch (IOException e) {
							}
						}
					}
				}
			});
		}

		public static void readFile(final String path, final @Nullable Options options, final Callback callback) {
			Async2.executor.submit(new Runnable() {
				@Override
				public void run() {
					File file = new File(path);
					FileInputStream stream = null;
					try {
						stream = new FileInputStream(file);
						int count;
						byte[] buffer = new byte[1024];
						ByteArrayOutputStream2 outputStream = new ByteArrayOutputStream2(stream.available());

						while (true) {
							count = stream.read(buffer);
							if (count <= 0)
								break;
							outputStream.write(buffer, 0, count);
						}

						callback.onComplete(outputStream.buf());
					} catch (FileNotFoundException e) {
						callback.onError(e.getMessage(), e);
					} catch (IOException e) {
						callback.onError(e.getMessage(), e);
					} finally {
						try {
							if (stream != null) {
								stream.close();
							}
						} catch (IOException ignored) {
						}
					}
				}
			});
		}

	}
}

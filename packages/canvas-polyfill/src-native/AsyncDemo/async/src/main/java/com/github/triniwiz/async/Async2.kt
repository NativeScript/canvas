package com.github.triniwiz.async

import android.os.Build
import android.os.Handler
import okhttp3.*
import okhttp3.internal.http2.ErrorCode
import okhttp3.internal.http2.StreamResetException
import okio.*
import org.json.JSONArray
import org.json.JSONException
import org.json.JSONObject
import org.json.JSONTokener
import java.io.*
import java.io.IOException
import java.net.SocketTimeoutException
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit


public class Async2 {

	private var handler: Handler? = null


	class Base64 {
		companion object {
			@JvmStatic
			fun b64WithoutPrefix(b64: String): String {
				return b64.split(",").toTypedArray()[1]
			}

			@JvmStatic
			@Throws(Exception::class)
			fun getMIMEForBase64String(b64: String): String {
				var input = b64
				if (b64.contains(",")) {
					input = b64WithoutPrefix(b64)
				}
				val first = input[0]
				return b64Extensions[first.toString()]
					?: throw Exception("Unknown Base64 MIME type: $b64")
			}

			@JvmStatic
			internal val uUID: String
				get() = UUID.randomUUID().toString()

			@JvmStatic
			fun base64ToFile(base64: String, path: String, callback: Callback) {
				executor.execute {
					val MIME: String
					MIME = try {
						getMIMEForBase64String(base64)
					} catch (e: Exception) {
						callback.error(e, e.message)
						return@execute
					}

					val localUri = "$path$uUID-b64image.$MIME"
					val file = File(localUri)
					var os: FileOutputStream? = null
					var isError = false
					try {
						os = FileOutputStream(file)
						os.write(
							android.util.Base64.decode(
								base64,
								android.util.Base64.DEFAULT
							)
						)
					} catch (e: FileNotFoundException) {
						isError = true
						callback.error(e, e.message)
					} catch (e: IOException) {
						isError = true
						callback.error(e, e.message)
					} finally {
						try {
							os?.flush()
							os?.close()
						} catch (e: java.lang.Exception) {

						}

						if (!isError) {
							callback.success(file.absolutePath)
						}
					}
				}
			}

		}

		interface Callback {
			fun success(response: Any?)
			fun error(error: Any?, message: String?)
		}
	}

	internal class ByteArrayOutputStream2 : ByteArrayOutputStream {
		constructor() : super()
		constructor(size: Int) : super(size)

		/**
		 * Returns the internal buffer of this ByteArrayOutputStream, without copying.
		 */
		@Synchronized
		fun buf(): ByteArray {
			return buf
		}
	}

	class Http {
		companion object {
			private val callMap = ConcurrentHashMap<String, CallOptions>()
			private val downloadCallMap = ConcurrentHashMap<String, DownloadCallOptions>()
			private val cancelList = ArrayList<String>()

			internal fun isTextType(contentType: String?): Boolean {
				var isTextType = false
				if (contentType != null) {
					val textTypes = arrayOf(
						"text/plain",
						"application/xml",
						"application/rss+xml",
						"text/html",
						"text/xml",
						"image/svg+xml"
					)
					for (type in textTypes) {
						isTextType = contentType.contains(type)
						if (isTextType) {
							break
						}
					}
				}
				return isTextType
			}


			@JvmStatic
			@JvmOverloads
			fun decodeBuffer(stream: ByteBuffer, encoding: String? = null): String {
				stream.rewind()
				if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
					val ret = when (encoding) {
						"UTF-8" -> {
							StandardCharsets.UTF_8.decode(stream).toString()
						}
						"US-ASCII" -> {
							StandardCharsets.US_ASCII.decode(stream).toString()
						}
						else -> StandardCharsets.ISO_8859_1.decode(stream).toString()
					}
					stream.rewind()
					return ret
				} else {
					val encoding = encoding?.let {
						when (it) {
							"UTF-8", "US-ASCII", "ISO-8859-1" -> {
								Charset.forName(it)
							}
							else -> null
						}
					}

					return if (stream.isDirect) {
						val buf = ByteArray(stream.remaining())
						stream.get(buf)
						stream.rewind()
						encoding?.let {
							String(buf, it)
						} ?: String(buf)
					} else {
						val buf = stream.array()
						encoding?.let {
							String(buf, it)
						} ?: String(buf)
					}
				}
			}

			@JvmStatic
			fun makeRequest(options: RequestOptions, callback: Callback): String {
				val uuid = UUID.randomUUID()
				val id = uuid.toString()
				executor.execute {
					val builder = OkHttpClient.Builder()
					val listener = HttpEventListener()
					builder.eventListener(listener)
					builder.followRedirects(!options.dontFollowRedirects)
					if (options.timeout > -1) {
						builder.connectTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
						builder.readTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
						builder.writeTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
					}

					if (options.username != null && options.password != null) {
						builder.authenticator { route, response ->
							if (response.request().header("Authorization") != null) {
								null
							} else response.request().newBuilder()
								.header("Authorization", Credentials.basic(options.username, options.password))
								.build()
						}
					}
					val request = Request.Builder()
					request.url(options.url)
					var contentType = "text/html"
					if (options.headers != null) {
						for (pair in options.headers!!) {
							pair.value?.let {
								request.addHeader(pair.key, it)
								if (pair.key == "Content-Type") {
									contentType = it
								}
							}
						}
					}
					var body: RequestBody? = null
					val isPostPutOrPatch =
						options.method == "POST" || options.method == "PUT" || options.method == "PATCH"
					if (isPostPutOrPatch) {
						if (options.content is File) {
						} else if (options.content is String) {
							if (contentType == "application/x-www-form-urlencoded") {
								val tokener = JSONTokener(options.content as String?)
								var value: Any? = null
								try {
									value = tokener.nextValue()
								} catch (e: JSONException) {
									e.printStackTrace()
								}
								if (value is JSONObject) {
									val formBody = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
										FormBody.Builder(StandardCharsets.UTF_8)
									} else {
										FormBody.Builder(Charset.forName("UTF-8"))
									}
									val it = value.keys()
									while (it.hasNext()) {
										val key = it.next()
										formBody.addEncoded(key, value.opt(key).toString())
									}
									body = ProgressRequestBody(formBody.build(), object : ProgressListener {
										override fun onProgress(loaded: Long, total: Long) {
											callback.onProgress(total > -1, loaded, total)
										}
									})
								} else {
									body = ProgressRequestBody(
										RequestBody.create(
											MediaType.parse(contentType),
											options.content as String?
										), object : ProgressListener {
											override fun onProgress(loaded: Long, total: Long) {
												callback.onProgress(total > -1, loaded, total)
											}
										})
								}
							} else {
								body = ProgressRequestBody(
									RequestBody.create(
										MediaType.parse(contentType),
										options.content as String?
									), object : ProgressListener {
										override fun onProgress(loaded: Long, total: Long) {
											callback.onProgress(total > -1, loaded, total)
										}
									})
							}
						} else if (options.content is JSONObject || options.content is JSONArray) {
							if (contentType == "application/x-www-form-urlencoded") {
								if (options.content is JSONObject) {
									var formBody: FormBody.Builder?
									formBody = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
										FormBody.Builder(StandardCharsets.UTF_8)
									} else {
										FormBody.Builder(Charset.forName("UTF-8"))
									}
									val it = (options.content as JSONObject?)!!.keys()
									while (it.hasNext()) {
										val key = it.next()
										formBody.addEncoded(key, (options.content as JSONObject?)!!.opt(key).toString())
									}
									body = ProgressRequestBody(formBody.build(), object : ProgressListener {
										override fun onProgress(loaded: Long, total: Long) {
											callback.onProgress(total > -1, loaded, total)
										}
									})
								} else {
									body = ProgressRequestBody(
										RequestBody.create(
											MediaType.parse(contentType),
											options.content.toString()
										), object : ProgressListener {
											override fun onProgress(loaded: Long, total: Long) {
												callback.onProgress(total > -1, loaded, total)
											}
										})
								}
							} else {
								body = ProgressRequestBody(
									RequestBody.create(
										MediaType.parse(contentType),
										options.content.toString()
									), object : ProgressListener {
										override fun onProgress(loaded: Long, total: Long) {
											callback.onProgress(total > -1, loaded, total)
										}
									})
							}
						} else {
							body = RequestBody.create(null, "")
						}
					}
					request.method(options.method, body)
					val client = builder.build()
					val call = client.newCall(request.build())
					call.enqueue(object : okhttp3.Callback {
						var stream: ByteBuffer? = null
						override fun onFailure(call: Call, e: IOException) {
							if (call.isCanceled) {
								val result = Result()
								if (stream != null) {
									result.content = stream
									result.url = call.request().url().toString()
									callback.onCancel(result)
								} else {
									result.content = ByteBuffer.allocate(0)
									result.url = call.request().url().toString()
									callback.onCancel(result)
								}
							} else if (e is SocketTimeoutException) {
								callback.onTimeout()
							} else {
								callback.onError(e.message, e)
							}
							callMap.remove(id)
						}

						@Throws(IOException::class)
						override fun onResponse(call: Call, response: Response) {
							val responseBody = ProgressResponseBody(response.body(), object : ProgressListener {
								override fun onProgress(loaded: Long, total: Long) {
									if (options.method != "POST" && options.method != "PUT") {
										callback.onProgress(total > -1, loaded, total)
									}
								}
							})
							var contentType = response.header("Content-Type")
							if (contentType == null) {
								contentType = response.header("content-type")
							}
							var acceptHeader: String?
							if (contentType == null) {
								acceptHeader = response.header("Accept")
								if (acceptHeader == null) {
									acceptHeader = response.header("accept")
								}
							} else {
								acceptHeader = contentType
							}
							var returnType = "text/plain"
							if (acceptHeader != null) {
								val acceptValues = acceptHeader.split(",").toTypedArray()
								val quality = ArrayList<String>()
								val defaultQuality = ArrayList<String>()
								val customQuality = ArrayList<String>()
								for (value in acceptValues) {
									if (value.contains(";q=")) {
										customQuality.add(value)
									} else {
										defaultQuality.add(value)
									}
								}
								Collections.sort(customQuality, QualitySort())
								quality.addAll(defaultQuality)
								quality.addAll(customQuality)
								returnType = quality[0]
							}
							val source = responseBody.source()

							val result = Result()
							result.contentText = ""
							result.url = response.request().url().toString()
							result.headers = ArrayList()


							var contentLength = responseBody.contentLength()

//							val headers = response.headers()
//							headers.names().forEach {
//								result.headers?.add(KeyValuePair(it, headers.get(it)))
//							}
							try {

								if (contentLength == -1L) {
									val array = source.readByteArray()
									stream = ByteBuffer
										.allocateDirect(array.size)
										.order(ByteOrder.nativeOrder())
									stream?.put(array)
									stream?.rewind()
								} else {
									stream = ByteBuffer
										.allocateDirect(responseBody.contentLength().toInt())
										.order(ByteOrder.nativeOrder())

									source.read(stream!!)
									stream?.rewind()
								}

								if (isTextType(returnType)) {
//									result.headers!!.add(KeyValuePair("Content-Type", returnType))
									result.content = decodeBuffer(stream!!)
									result.contentText = result.content as String?
									callback.onComplete(result)
								} else if (returnType.contains("application/json")) {
									val returnValue = decodeBuffer(stream!!)
									val tokener = JSONTokener(returnValue)
									val value = tokener.nextValue()
									if (value is JSONObject || value is JSONArray) {
//										result.headers!!.add(KeyValuePair("Content-Type", returnType))
										result.content = value
										result.contentText = returnValue
									} else {
//										result.headers!!.add(KeyValuePair("Content-Type", "text/plain"))
										result.content = returnValue
										result.contentText = returnValue
									}
									callback.onComplete(result)
								} else {
//									result.headers!!.add(KeyValuePair("Content-Type", "application/octet-stream"))
									result.content = stream
									callback.onComplete(result)
								}
							} catch (e: StreamResetException) {
								if (e.errorCode == ErrorCode.CANCEL) {
									if (isTextType(returnType)) {
										result.content = decodeBuffer(stream!!)
										callback.onCancel(result)
									} else if (returnType.contains("application/json")) {
										val returnValue = decodeBuffer(stream!!)
										val tokener = JSONTokener(returnValue)
										try {
											val value = tokener.nextValue()
											if (value is JSONObject || value is JSONArray) {
//												result.headers!!.add(KeyValuePair("Content-Type", returnType))
												result.content = value
												result.contentText = returnValue
											} else {
//												result.headers!!.add(KeyValuePair("Content-Type", "text/plain"))
												result.content = returnValue
												result.contentText = returnValue
											}
											callback.onCancel(result)
										} catch (e1: JSONException) {
											callback.onError(e1.message, e1)
										}
									} else {
//										result.headers!!.add(KeyValuePair("Content-Type", "application/octet-stream"))
										result.content = stream
										callback.onCancel(result)
									}
								} else {
									callback.onError(e.message, e)
								}
							} catch (e: SocketTimeoutException) {
								callback.onTimeout()
							} catch (e: Exception) {
								callback.onError(e.message, e)
							}
							responseBody.close()
							callMap.remove(id)
						}
					})
					callMap[id] = CallOptions(call, options, callback)
					for (cancelId in cancelList) {
						if (id == cancelId) {
							call.cancel()
							callMap.remove(cancelId)
							cancelList.remove(cancelId)
						}
					}
				}
				return id
			}

			@JvmStatic
			fun getFileRequest(options: DownloadRequestOptions, callback: Callback): String {
				val uuid = UUID.randomUUID()
				val id = uuid.toString()
				executor.execute {
					val builder = OkHttpClient.Builder()
					builder.eventListener(HttpEventListener())
					builder.followRedirects(!options.dontFollowRedirects)
					if (options.timeout > -1) {
						builder.connectTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
						builder.readTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
						builder.writeTimeout(options.timeout.toLong(), TimeUnit.MILLISECONDS)
					}
					if (options.username != null && options.password != null) {
						builder.authenticator { route, response ->
							if (response.request().header("Authorization") != null) {
								null
							} else response.request().newBuilder()
								.header("Authorization", Credentials.basic(options.username, options.password))
								.build()
						}
					}
					val request = Request.Builder()
					request.url(options.url)
					val client = builder.build()
					val call = client.newCall(request.build())
					call.enqueue(object : okhttp3.Callback {
						override fun onFailure(call: Call, e: IOException) {
							if (call.isCanceled) {
								val result = FileResult()
								callback.onCancel(result)
							} else if (e is SocketTimeoutException) {
								callback.onTimeout()
							} else {
								callback.onError(e.message, e)
							}
							downloadCallMap.remove(id)
						}

						@Throws(IOException::class)
						override fun onResponse(call: Call, response: Response) {
							val responseBody = ProgressResponseBody(
								response.body(),
								response.headers(),
								object : ProgressListener {
									override fun onProgress(loaded: Long, total: Long) {
										callback.onProgress(total > -1, loaded, total)
									}
								})
							val bufferedSource = responseBody.source()
							val file = File(options.filePath)
							var sink: BufferedSink? = null
							try {
								sink = file.sink().buffer()
								sink.writeAll(bufferedSource)
								val result = FileResult()
								result.url = response.request().url().toString()
								result.headers = ArrayList()
								result.filePath = file.absolutePath
								sink.close()
								callback.onComplete(result)
							} catch (e: StreamResetException) {
								if (e.errorCode == ErrorCode.CANCEL) {
									val result = FileResult()
									callback.onCancel(result)
								} else {
									callback.onError(e.message, e)
								}
							} catch (e: SocketTimeoutException) {
								callback.onTimeout()
							} catch (e: Exception) {
								callback.onError(e.message, e)
							} finally {
								if (sink != null) {
									try {
										sink.close()
									} catch (e: IOException) {
									}
								}
								if (bufferedSource != null) {
									try {
										bufferedSource.close()
									} catch (e: IOException) {
									}
								}
								responseBody.close()
								downloadCallMap.remove(id)
							}
						}
					})
					downloadCallMap[id] = DownloadCallOptions(call, options, callback)
					for (cancelId in cancelList) {
						if (id == cancelId) {
							call.cancel()
							downloadCallMap.remove(cancelId)
							cancelList.remove(cancelId)
						}
					}
				}
				return id
			}

			@JvmStatic
			fun cancelRequest(id: String?) {
				if (id != null) {
					executor.execute {
						val pair = callMap[id]
						val downloadPair = downloadCallMap[id]
						pair?.call?.cancel()
						downloadPair?.call?.cancel()
						if (pair == null && downloadPair == null) {
							cancelList.add(id)
						}
					}
				}
			}
		}

		internal interface ProgressListener {
			fun onProgress(loaded: Long, total: Long)
		}

		class Result internal constructor() {
			var content: Any? = null
			var url: String? = null
			var headers: ArrayList<KeyValuePair>? = null
			var contentText: String? = null
		}

		class FileResult internal constructor() {
			var filePath: String? = null
			var url: String? = null
			var headers: ArrayList<KeyValuePair>? = null
		}

		internal class ProgressRequestBody(var body: RequestBody, var listener: ProgressListener?) :
			RequestBody() {
			override fun contentType(): MediaType? {
				return body.contentType()
			}

			@Throws(IOException::class)
			override fun contentLength(): Long {
				return body.contentLength()
			}

			@Throws(IOException::class)
			override fun writeTo(sink: BufferedSink) {
				val bufferedSink: BufferedSink = forwardingSink(sink).buffer()
				body.writeTo(bufferedSink)
				bufferedSink.close()
			}

			fun forwardingSink(sink: BufferedSink?): ForwardingSink {
				var contentLength: Long = -1
				try {
					contentLength = contentLength()
				} catch (ignored: IOException) {
				}
				val length = contentLength
				return object : ForwardingSink(sink!!) {
					var totalBytesWrite: Long = 0

					@Throws(IOException::class)
					override fun write(source: Buffer, byteCount: Long) {
						super.write(source, byteCount)
						if (listener != null) {
							totalBytesWrite += if (byteCount != -1L) byteCount else 0
							if (length < 0) {
								listener!!.onProgress(totalBytesWrite, length)
							}
							if (byteCount >= 0) {
								listener!!.onProgress(totalBytesWrite, length)
							}
						}
					}
				}
			}
		}

		internal class ProgressResponseBody : ResponseBody {
			var body: ResponseBody?
			var listener: ProgressListener?
			var bufferedSource: BufferedSource? = null
			var headers: Headers? = null

			constructor(body: ResponseBody?, listener: ProgressListener?) {
				this.body = body
				this.listener = listener
			}

			constructor(body: ResponseBody?, headers: Headers?, listener: ProgressListener?) {
				this.body = body
				this.listener = listener
				this.headers = headers
			}

			override fun contentType(): MediaType? {
				return body!!.contentType()
			}

			override fun contentLength(): Long {
				return body!!.contentLength()
			}

			override fun source(): BufferedSource {
				if (bufferedSource == null) {
					bufferedSource = source(body!!.source()).buffer()
				}
				return bufferedSource!!
			}

			private fun source(source: Source): Source {
				return object : ForwardingSource(body!!.source()) {
					var totalBytesRead: Long = 0
					var length = contentLength()

					@Throws(IOException::class)
					override fun read(sink: Buffer, byteCount: Long): Long {
						val bytesRead = super.read(sink, byteCount)
						if (listener != null) {
							totalBytesRead += if (bytesRead != -1L) bytesRead else 0
							if (length < 0) {
								listener!!.onProgress(totalBytesRead, length)
							}
							if (bytesRead >= 0) {
								listener!!.onProgress(totalBytesRead, length)
							}
						}
						return bytesRead
					}
				}
			}
		}

		interface Callback {
			fun onComplete(result: Any?)
			fun onError(error: String?, e: Exception?)
			fun onProgress(lengthComputable: Boolean, loaded: Long, total: Long)
			fun onHeaders(headers: ArrayList<KeyValuePair>?, status: Int)
			fun onCancel(result: Any?)
			fun onLoading()
			fun onTimeout()
		}

		internal class HttpEventListener : okhttp3.EventListener() {
			override fun requestHeadersEnd(call: Call, request: Request) {
				super.requestHeadersEnd(call, request)
			}

			override fun responseHeadersEnd(call: Call, response: Response) {
				for (key in callMap.keys) {
					val options = callMap[key]
					if (options != null && options.call == call) {
						val responseHeaders = response.headers()
						val headers = ArrayList<KeyValuePair>()
						for (value in responseHeaders.names()) {
							headers.add(KeyValuePair(value, responseHeaders[value]))
						}
						options.callback.onHeaders(headers, response.code())
					}
				}
				super.responseHeadersEnd(call, response)
			}

			override fun responseBodyStart(call: Call) {
				val method = call.request().method()
				if (method != "POST" && method != "PUT") {
					for (key in callMap.keys) {
						val options = callMap[key]
						if (options != null && options.call == call) {
							options.callback.onLoading()
						}
					}
				}
				super.responseBodyStart(call)
			}

			override fun responseBodyEnd(call: Call, byteCount: Long) {
				super.responseBodyEnd(call, byteCount)
			}

			override fun requestBodyStart(call: Call) {
				val method = call.request().method()
				if (method == "POST" || method == "PUT") {
					for (key in callMap.keys) {
						val options = callMap[key]
						if (options != null && options.call == call) {
							options.callback.onLoading()
						}
					}
				}
				super.requestBodyStart(call)
			}

			override fun requestBodyEnd(call: Call, byteCount: Long) {
				super.requestBodyEnd(call, byteCount)
			}
		}

		internal class CallOptions(var call: Call, var options: RequestOptions, var callback: Callback)
		internal class DownloadCallOptions(
			var call: Call,
			var options: DownloadRequestOptions,
			var callback: Callback
		)

		class KeyValuePair(var key: String, var value: String?)
		class RequestOptions {
			var url: String? = null
			var method: String? = null
			var headers: ArrayList<KeyValuePair>? = null
			var content: Any? = null
			var timeout = -1
			var dontFollowRedirects = false
			var username: String? = null
			var password: String? = null
		}

		class DownloadRequestOptions {
			var url: String? = null
			var filePath: String? = null
			var headers: ArrayList<KeyValuePair>? = null
			var timeout = -1
			var dontFollowRedirects = false
			var username: String? = null
			var password: String? = null
		}
	}

	internal class QualitySort : Comparator<String> {
		override fun compare(a: String, b: String): Int {
			val a_quality = java.lang.Float.valueOf(a.substring(a.indexOf(";q=")).replace(";q=", ""))
			val b_quality = java.lang.Float.valueOf(b.substring(b.indexOf(";q=")).replace(";q=", ""))
			return (b_quality - a_quality).toInt()
		}
	}

	class FileManager {

		companion object {
			@JvmStatic
			fun writeFile(bytes: ByteArray?, path: String?, callback: Callback) {
				executor.execute {
					val file = File(path)
					var stream: FileOutputStream? = null
					var isError = false
					try {
						stream = FileOutputStream(file)
						stream.write(bytes)
					} catch (e: FileNotFoundException) {
						isError = true
						callback.onError(e.message, e)
					} catch (e: IOException) {
						isError = true
						callback.onError(e.message, e)
					} finally {
						if (stream != null) {
							try {
								stream.flush()
								stream.close()
							} catch (e: IOException) {
							}
						}
						if (!isError) {
							callback.onComplete(true)
						}
					}
				}
			}

			@JvmStatic
			fun writeFile(bytes: ByteBuffer?, path: String?, callback: Callback) {
				executor.execute {
					val file = File(path)
					var stream: FileOutputStream? = null
					var isError = false
					try {
						stream = FileOutputStream(file)
						val channel = stream.channel
						channel.write(bytes)
					} catch (e: FileNotFoundException) {
						isError = true
						callback.onError(e.message, e)
					} catch (e: IOException) {
						isError = true
						callback.onError(e.message, e)
					} finally {
						if (stream != null) {
							try {
								stream.flush()
								stream.close()
							} catch (e: IOException) {
							}
						}
						if (!isError) {
							callback.onComplete(true)
						}
					}
				}
			}

			@JvmStatic
			fun readFile(path: String?, options: Options?, callback: Callback) {
				executor.execute {
					val file = File(path)
					var stream: FileInputStream? = null
					var outputStream: ByteArrayOutputStream2? = null
					var isError = false
					try {
						stream = FileInputStream(file)
						var count: Int
						val buffer = ByteArray(DEFAULT_BUFFER_SIZE)
						outputStream = ByteArrayOutputStream2(stream.available())
						while (true) {
							count = stream.read(buffer)
							if (count <= 0) break
							outputStream.write(buffer, 0, count)
						}
					} catch (e: FileNotFoundException) {
						isError = true
						callback.onError(e.message, e)
						isError = true
					} catch (e: IOException) {
						callback.onError(e.message, e)
					} finally {
						try {
							stream?.close()

						} catch (ignored: IOException) {
						}
						if (!isError) {
							callback.onComplete(outputStream!!.buf())
						}
					}
				}
			}

			@JvmStatic
			fun readFileBuffer(path: String?, options: Options?, callback: Callback) {
				executor.execute {
					val file = File(path)
					var stream: FileInputStream? = null
					var buffer: ByteBuffer? = null
					var isError = false
					try {
						stream = FileInputStream(file)
						val channel = stream.channel
						buffer = ByteBuffer.allocateDirect(stream.available())
						channel.read(buffer)
					} catch (e: FileNotFoundException) {
						isError = true
						callback.onError(e.message, e)
						isError = true
					} catch (e: IOException) {
						callback.onError(e.message, e)
					} finally {
						try {
							stream?.close()

						} catch (ignored: IOException) {
						}
						if (!isError) {
							callback.onComplete(buffer)
						}
					}
				}
			}
		}

		interface Callback {
			fun onError(error: String?, e: Exception?)
			fun onComplete(result: Any?)
		}

		class Options {
			var asStream = false
		}
	}

	companion object {
		@JvmStatic
		private val executor = Executors.newCachedThreadPool()

		@JvmStatic
		val b64Extensions: MutableMap<String, String> = HashMap()

		@JvmStatic
		fun runInBackground(runnable: Runnable?) {
			executor.submit {
				runnable?.run()
			}
		}

		init {
			b64Extensions["/"] = "jpg"
			b64Extensions["i"] = "png"
			b64Extensions["R"] = "gif"
			b64Extensions["U"] = "webp"
		}
	}
}

// swift-tools-version:5.9
//
// AudioContextNative.xcframework as a SwiftPM binary target, referenced by local path from this
// plugin's nativescript.config.ts. In the repo it points at a framework downloaded or
// built into this directory (see README).
// npm_release.yml uploads the framework to the GitHub release and rewrites the target
// to its url + checksum, so the npm package ships only this file.
import PackageDescription

let package = Package(
    name: "AudioContextNativeBinary",
    platforms: [.iOS(.v12), .macCatalyst(.v14), .tvOS(.v13), .visionOS(.v1)],
    products: [
        .library(name: "AudioContextNative", targets: ["AudioContextNative"]),
    ],
    targets: [
        .binaryTarget(name: "AudioContextNative", path: "AudioContextNative.xcframework"),
    ]
)

// swift-tools-version:5.9
//
// NOTE: NativeScript plugin consumers do NOT fetch this package from the
// repository. The @nativescript/canvas and @nativescript/audio-context npm
// packages ship this same header-only shim at platforms/ios/NativeScriptV8
// and reference it via a local `path` SPM package in nativescript.config.ts.
//
// That is deliberate: SwiftPM clones a repository's ENTIRE git history just
// to read Package.swift, and this repo's history is several GB — a
// repositoryURL reference here means a 20+ minute "Fetching" on every first
// build. A binaryTarget would not help either; the clone happens before the
// target type is even known. Keep plugin references path-based.
//
// This manifest remains only for direct SwiftPM consumption of the repo
// (expect a very slow first fetch) and for the in-repo demo apps.
import PackageDescription

let package = Package(
    name: "CanvasNative",
    platforms: [.iOS(.v12), .visionOS(.v1)],
    products: [
        .library(name: "NativeScriptV8", targets: ["NativeScriptV8"]),
    ],
    targets: [
        .target(
            name: "NativeScriptV8",
            path: "nativescript-v8",
            sources: ["Sources"],
            publicHeadersPath: "Headers"
        ),
    ]
)

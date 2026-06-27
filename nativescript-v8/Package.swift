// swift-tools-version:5.9
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
            path: ".",
            sources: ["Sources"],
            publicHeadersPath: "Headers"
        ),
    ]
)

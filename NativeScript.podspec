Pod::Spec.new do |s|
    s.name         = "NativeScript"
    s.version      = '1.0.0'
    s.summary      = "V8 + NativeScript runtime headers for canvas and audio-context"
    s.license      = "MIT"

    s.authors      = "Osei Fortune"
    s.homepage     = "https://github.com/NativeScript/canvas"
    s.platforms    = { :ios => "12.0", :visionos => "1.0" }

    # Both canvas and audio-context can depend on this pod from the same repo:
    #   pod 'NativeScript', :git => 'https://github.com/NativeScript/canvas.git', :tag => 'v1.0.0'
    s.source       = { :git => "https://github.com/NativeScript/canvas.git", :tag => "v#{s.version}" }
    s.cocoapods_version = ">= 1.10.1"

    # Headers-only — the NativeScript.xcframework binary is provided by the
    # NativeScript runtime at app build time, not by this pod.
    s.source_files        = "nativescript-v8/Headers/**/*.h"
    s.header_mappings_dir = "nativescript-v8/Headers"
    s.xcconfig = { 'HEADER_SEARCH_PATHS' => '$(inherited) "${PODS_ROOT}/Headers/Public/NativeScript"' }
end

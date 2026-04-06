Pod::Spec.new do |s|
    s.name         = "NativeScript"
    s.version      = '1.0.0'
    s.summary      = "description"
    s.license      = "MIT"

    s.authors      = "Ammar Ahmed"
    s.homepage     = "https://github.com/ammarahm-ed/nativescript-v8-module"
    s.platforms    = { :ios => "12.0" }
    s.source       = { :git => "https://github.com/ammarahm-ed/nativescript-v8-module.git", :tag => "v1.0.0" }
    s.cocoapods_version = ">= 1.10.1"
    s.vendored_frameworks = "NativeScript.xcframework"

    s.source_files = "Headers/**/*.h"
    s.header_mappings_dir = "Headers"
    s.xcconfig = { 'HEADER_SEARCH_PATHS' => '$(inherited) "${PODS_ROOT}/Headers/Public/NativeScript"' }
end

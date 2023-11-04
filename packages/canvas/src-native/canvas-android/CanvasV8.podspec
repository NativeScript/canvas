Pod::Spec.new do |s|
    s.name         = "CanvasV8"
    s.version      = '1.0.0'
    s.summary      = "description"
    s.license      = "MIT"

    s.authors      = "Osei Fortune"
    s.homepage     = "https://github.com/NativeScript/canvas"
    s.platforms    = { :ios => "12.4" }
    s.source       = { :git => "https://github.com/NativeScript/canvas.git", :tag => "v1.0.0" }
    s.cocoapods_version      = ">= 1.10.1"
    s.vendored_frameworks = "NativeScript.xcframework", "CanvasNative.xcframework"
    s.source_files = 'canvas/src/main/cpp/**/**/*.{h,cpp,cc} canvas/src/main/cpp/rust/*.{h,cc}'
    s.xcconfig = { 'HEADER_SEARCH_PATHS' => '"${PODS_SRC_ROOT}/canvas/src/main/**/**/*.{h}"' }
    s.preserve_path = '"${PODS_SRC_ROOT}/canvas/src/main/cpp/*"'
  end
 
  
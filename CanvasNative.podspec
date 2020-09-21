Pod::Spec.new do |s|

    s.name         = "CanvasNative"

    s.version      = "0.9.0"

    s.summary      = "A Canvas library"

    s.homepage     = "https://github.com/nativescript/tree/master/canvas/src-native/canvas-ios"


    s.license      = { :type => "MIT", :file => "LICENSE" }


    s.author       = { "Osei Fortune" => "fortune.osei@yahoo.com" }

    s.platform     = :ios, "9.0"

    s.source       = { :git => "https://github.com/nativescript/native-ios.git", :tag => "#{s.version}" }

    s.source_files  = 'packages/canvas/src-native/canvas-ios/CanvasNative/**/*.{swift,m,mm,h,c}'
    s.preserve_paths = 'libcanvasnative.a'
    s.pod_target_xcconfig = {
'USER_HEADER_SEARCH_PATHS' => '"${PODS_ROOT}/CanvasNative/include" "${PODS_ROOT}/CanvasNative/CanvasNative/include"',
'HEADER_SEARCH_PATHS' => '"${PODS_ROOT}/CanvasNative/include" "${PODS_ROOT}/CanvasNative/CanvasNative/include"',
'CLANG_ALLOW_NON_MODULAR_INCLUDES_IN_FRAMEWORK_MODULES' => 'YES',
'ENABLE_BITCODE' => 'NO',
'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' ,
 'GCC_PREPROCESSOR_DEFINITIONS' => 'GLES_SILENCE_DEPRECATION=1'
 }
 s.user_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
    s.swift_versions = ['4.0','4.2', '5.0']
    s.vendored_libraries = 'packages/canvas/src-native/canvas-ios/CanvasNative/libs/*.a'
    s.public_header_files = 'packages/canvas/src-native/canvas-ios/CanvasNative/include/*.h'
    s.libraries = 'c++'
  end

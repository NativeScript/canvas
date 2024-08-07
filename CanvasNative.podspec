Pod::Spec.new do |s|

    s.name         = "CanvasNative"

    s.version      = "2.0.0"

    s.summary      = "A Canvas library"

    s.homepage     = "https://github.com/nativescript/canvas"


    s.license      = { :type => "MIT", :file => "LICENSE" }

    s.author       = { "Osei Fortune" => "fortune.osei@yahoo.com" }

    s.platform     = :ios, "13.0"

    s.source       = { :git => "https://github.com/nativescript/canvas.git", :tag => "#{s.version}" }

    s.pod_target_xcconfig = {
			'FRAMEWORK_SEARCH_PATHS' => '$(inherited) "${PODS_ROOT}/dist"',
			'ENABLE_BITCODE' => 'NO'
 }
    s.swift_versions = ['4.0','4.2', '5.0']
    s.vendored_frameworks = 'packages/canvas/src-native/canvas-ios/dist/CanvasNative.xcframework'
    s.preserve_paths = 'packages/canvas/src-native/canvas-ios/dist/CanvasNative.xcframework'
  end

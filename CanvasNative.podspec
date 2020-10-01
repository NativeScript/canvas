Pod::Spec.new do |s|

    s.name         = "CanvasNative"

    s.version      = "0.9.3"

    s.summary      = "A Canvas library"

    s.homepage     = "https://github.com/nativescript/canvas"


    s.license      = { :type => "MIT", :file => "LICENSE" }

    s.author       = { "Osei Fortune" => "fortune.osei@yahoo.com" }

    s.platform     = :ios, "11.0"

    s.source       = { :git => "https://github.com/nativescript/canvas.git", :tag => "#{s.version}" }

    s.pod_target_xcconfig = {
			'FRAMEWORK_SEARCH_PATHS' => '$(inherited) "${PODS_ROOT}/CanvasNative/frameworks"',
			'ENABLE_BITCODE' => 'YES',
 }
    s.swift_versions = ['4.0','4.2', '5.0']
    s.vendored_frameworks = 'packages/canvas/src-native/canvas-ios/CanvasNative/Dist/CanvasNative.xcframework'
    s.preserve_paths = 'CanvasNative.xcframework'
  end

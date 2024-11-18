import UIKit
import CanvasNative
import GLKit
@available(iOS 13.0, *)
class ViewController: UIViewController {
    func contextReady() {
        print("ready")
    }
    
    var canvas: NSCCanvas?
    var imageView: UIImageView?
    let PI: Float = .pi
    let TWO_PI: Float = .pi * 2
    var tapped = 0
    
    override init(nibName nibNameOrNil: String?, bundle nibBundleOrNil: Bundle?) {
        super.init(nibName: nibNameOrNil, bundle: nibBundleOrNil)
    }
    
    required init?(coder: NSCoder) {
        super.init(coder: coder)
    }
    
    
    @objc func linkTriggered(displaylink: CADisplayLink) {}
        
    
   // var svg: TNSSVG?
    override func viewDidLoad() {
       // super.viewDidLoad()
        canvas = NSCCanvas(frame: .zero)
        NSCCanvas.forceGL = false
        view.addSubview(canvas!)
        canvas?.frame = view.frame
        view.backgroundColor = .white
        
        let emoji = NSCFontFace(family: "emoji")
        
        emoji.load { error in
            print("emoji Font load error \(error ?? "") \(emoji.font.debugDescription)")
                }
                
        
        
        NSCFontFace.loadFromRemote(url: "https://fonts.googleapis.com/css2?family=Noto+Serif+TC:wght@200..900&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swa") { fonts, error in
            if let fonts = fonts {
                for font in fonts {
                    NSCFontFaceSet.instance.add(font)
                }
                print(NSCFontFaceSet.instance.size)
                NSCFontFaceSet.instance.load("italic bold 16px 'Noto Serif TC'", nil){f,err in
                    print("NSCFontFaceSet: load",f, err)
                }
            }
           
        }
        
        /*
        NSCFontFace.loadFromRemote(url: "https://fonts.googleapis.com/css2?family=Monsieur+La+Doulaise&family=Noto+Serif+TC:wght@200..900&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap") { fonts, error in
            
            if let fonts = fonts {
                for font in fonts {
                    NSCFontFaceSet.instance.add(font)
                    font.load({ error in
                        print("Font Failed to load", error, "font: ?" , font.font)
                    })
                }
            }
           
        }
        */
        
        
//        let fangsong = NSCFontFace(family: "fangsong")
//        fangsong.weight = 2000
//        fangsong.load { error in
//            print("Math Font load error \(error) \(fangsong.font)")
//        }
//        
        /*
        
        let math = NSCFontFace(family: "math")
        math.load { error in
            print("Math Font load error \(error)")
        }
        
        let ff = NSCFontFace(family: "serif")
        NSCFontFaceSet.instance.add(ff)
        ff.load { error in
            print("Serif Font load error \(error)")
        }
        
        */
        
        
//        let c = NSCCanvas(frame: .zero)
//        
//        let cc = c.create2DContext(
//                                     true,
//                                     true,
//                                     true,
//                                     false,
//                                     0,
//                                     true,
//                                     false,
//                                     false,
//                                     false,
//                                     false,
//                                     -16777216
//         )
//        
            
      //  canvas?.forceLayout(view.bounds.width, view.bounds.height)
       // canvas = NSCCanvas(frame: view.bounds)
        //view.addSubview(canvas!)
//        canvas?.frame = view.frame
//        canvas?.layoutIfNeeded()
//       let ctx = canvas!.create2DContext(
//                                    true,
//                                    true,
//                                    true,
//                                    false,
//                                    0,
//                                    true,
//                                    false,
//                                    false,
//                                    false,
//                                    false,
//                                    -16777216
//        )
        
        
       // canvas?.context2DConic(ctx)
        
        
//        let ss = canvas?.snapshot(false)
//        
//        let v = UIImageView(image: ss)
//        view.addSubview(v)
//        v.frame = CGRect(origin: CGPoint(x: v.frame.origin.x, y: 400), size: v.frame.size)
        
      //  canvas?.context2DTest(ctx)
     
        
      
//        if(data != nil){
//            print(String(data: data!, encoding: .utf8))
//        }
        
        /*var start = CACurrentMediaTime()
        print(canvas?.context2DTestToDataURL(ctx))
        print("done", CACurrentMediaTime() - start)
        
        
     
        
        start = CACurrentMediaTime()
        let data = glView.snapshot.pngData()?.base64EncodedData()
       
        print("done", CACurrentMediaTime() - start)
        */
//        print(String(data: data!, encoding: .utf8))
        
    //    print(canvas?.context2DTestToDataURL(ctx))
        // Do any additional setup after loading the view.
       //canvas1.setListener(self)
        
       // canvas1?.ignorePixelScaling = true
       // canvas1.backgroundColor = .white
       
    
    }
    
    
    func drawAll() {
 
        
    }
    
   
}

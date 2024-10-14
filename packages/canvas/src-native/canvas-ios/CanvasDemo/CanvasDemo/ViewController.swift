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
       let ctx = canvas!.create2DContext(
                                    true,
                                    true,
                                    true,
                                    false,
                                    0,
                                    true,
                                    false,
                                    false,
                                    false,
                                    false,
                                    -16777216
        )
        
        
        canvas?.context2DConic(ctx)
        
        
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

import UIKit
import CanvasNative
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
   // var svg: TNSSVG?
    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .white
        canvas = NSCCanvas(frame: view.bounds)
        view.addSubview(canvas!)

        let ctx = canvas!.create2DContext(
                                    true,
                                    true,
                                    true,
                                    false,
                                    "default",
                                    true,
                                    false,
                                    false,
                                    false,
                                    false,
                                    -16777216
        )
        

        canvas?.context2DTest(ctx)
        // Do any additional setup after loading the view.
       //canvas1.setListener(self)
        
       // canvas1?.ignorePixelScaling = true
       // canvas1.backgroundColor = .white
       
    
    }
    
    
    func drawAll() {
 
        
    }
    
   
}

import UIKit
import CanvasNative
@available(iOS 13.0, *)
class ViewController: UIViewController {
    func contextReady() {
        print("ready")
    }
    
   // @IBOutlet weak var canvas1: NSCCanvas!
   // var canvas2: NSCCanvas?
    var imageView: UIImageView?
    let PI: Float = .pi
    let TWO_PI: Float = .pi * 2
    var tapped = 0
   // var svg: TNSSVG?
    override func viewDidLoad() {
        super.viewDidLoad()
        print(CanvasHelpers.self)
        let scale = Int(UIScreen.main.scale)
        // Do any additional setup after loading the view.
       //canvas1.setListener(self)
        
       // canvas1?.ignorePixelScaling = true
       // canvas1.backgroundColor = .white
       
    
    }
    
    
    func drawAll() {
 
        
    }
    
   
}

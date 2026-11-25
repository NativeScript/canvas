//! WinUI 3 (`Microsoft.UI.Xaml`) bindings for the drawable host, which the `windows` crate does not
//! cover. Declared from the Windows App SDK 1.6 metadata: each interface's IID and its vtable up to
//! the last slot used.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::ffi::c_void;

use ::windows::Foundation::{Point, Rect};
use ::windows::Storage::Streams::IBuffer;
use windows_collections::IVector;
use windows_core::imp::{define_interface, interface_hierarchy, FactoryCache, IGenericFactory};
use windows_core::{
  IInspectable, IInspectable_Vtbl, IUnknown, Interface, Result, Type, GUID, HRESULT,
};
use windows_future::{IAsyncAction, IAsyncOperation};

type Raw = *mut c_void;

/// `Microsoft.UI.Xaml.Media.Matrix`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix {
  pub M11: f64,
  pub M12: f64,
  pub M21: f64,
  pub M22: f64,
  pub OffsetX: f64,
  pub OffsetY: f64,
}

/// `HorizontalAlignment.Left` / `VerticalAlignment.Top`.
pub const ALIGN_START: i32 = 0;

define_interface!(
  IUIElement,
  IUIElement_Vtbl,
  0xc3c01020_320c_5cf6_9d24_d396bbfa4d8b
);
impl windows_core::RuntimeType for IUIElement {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIElement_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 4],
  SetOpacity: unsafe extern "system" fn(Raw, f64) -> HRESULT, // 5
  _6: usize,
  SetClip: unsafe extern "system" fn(Raw, Raw) -> HRESULT, // 7
  _8: usize,
  SetRenderTransform: unsafe extern "system" fn(Raw, Raw) -> HRESULT, // 9
  _10: [usize; 7],
  SetIsHitTestVisible: unsafe extern "system" fn(Raw, bool) -> HRESULT, // 17
  _18: [usize; 3],
  UseLayoutRounding: unsafe extern "system" fn(Raw, *mut bool) -> HRESULT, // 21
  SetUseLayoutRounding: unsafe extern "system" fn(Raw, bool) -> HRESULT,   // 22
  _23: [usize; 81],
  XamlRoot: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT, // 104
  _105: [usize; 3],
  RasterizationScale: unsafe extern "system" fn(Raw, *mut f64) -> HRESULT, // 108
  SetRasterizationScale: unsafe extern "system" fn(Raw, f64) -> HRESULT,   // 109
  _110: [usize; 15],
  KeyUp: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT, // 125
  RemoveKeyUp: unsafe extern "system" fn(Raw, i64) -> HRESULT,
  KeyDown: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT,
  RemoveKeyDown: unsafe extern "system" fn(Raw, i64) -> HRESULT,
  _129: [usize; 18],
  PointerPressed: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT, // 147
  RemovePointerPressed: unsafe extern "system" fn(Raw, i64) -> HRESULT,
  PointerMoved: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT,
  RemovePointerMoved: unsafe extern "system" fn(Raw, i64) -> HRESULT,
  PointerReleased: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT,
  RemovePointerReleased: unsafe extern "system" fn(Raw, i64) -> HRESULT, // 152
  _153: [usize; 8],
  PointerWheelChanged: unsafe extern "system" fn(Raw, Raw, *mut i64) -> HRESULT, // 161
  RemovePointerWheelChanged: unsafe extern "system" fn(Raw, i64) -> HRESULT,
  _163: [usize; 47],
  AddHandler: unsafe extern "system" fn(Raw, Raw, Raw, bool) -> HRESULT, // 210
  RemoveHandler: unsafe extern "system" fn(Raw, Raw, Raw) -> HRESULT,
  TransformToVisual: unsafe extern "system" fn(Raw, Raw, *mut Raw) -> HRESULT, // 212
}

define_interface!(
  IUIElementStatics,
  IUIElementStatics_Vtbl,
  0xd2921d87_3584_5e22_8a3a_c2c78dab4f6e
);
#[repr(C)]
pub struct IUIElementStatics_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 2],
  PointerEnteredEvent: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT, // 3
  _4: usize,
  PointerMovedEvent: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT, // 5
}

define_interface!(
  IPointerRoutedEventArgs,
  IPointerRoutedEventArgs_Vtbl,
  0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c
);
#[repr(C)]
pub struct IPointerRoutedEventArgs_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 5],
  GetCurrentPoint: unsafe extern "system" fn(Raw, Raw, *mut Raw) -> HRESULT, // 6
}

// Microsoft.UI.Input.IPointerPoint
define_interface!(
  IPointerPoint,
  IPointerPoint_Vtbl,
  0x0d430ee6_252c_59a4_b2a2_d44264dc6a40
);
#[repr(C)]
pub struct IPointerPoint_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 4],
  Position: unsafe extern "system" fn(Raw, *mut Point) -> HRESULT, // 5
}

define_interface!(
  IFrameworkElement,
  IFrameworkElement_Vtbl,
  0xfe08f13d_dc6a_5495_ad44_c2d8d21863b0
);
#[repr(C)]
pub struct IFrameworkElement_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 7],
  ActualWidth: unsafe extern "system" fn(Raw, *mut f64) -> HRESULT, // 8
  ActualHeight: unsafe extern "system" fn(Raw, *mut f64) -> HRESULT,
  _10: usize,
  SetWidth: unsafe extern "system" fn(Raw, f64) -> HRESULT, // 11
  _12: [usize; 11],
  SetHorizontalAlignment: unsafe extern "system" fn(Raw, i32) -> HRESULT, // 23
  _24: usize,
  SetVerticalAlignment: unsafe extern "system" fn(Raw, i32) -> HRESULT, // 25
}

define_interface!(IPanel, IPanel_Vtbl, 0x27a1b418_56f3_525e_b883_cefed905eed3);
#[repr(C)]
pub struct IPanel_Vtbl {
  base__: IInspectable_Vtbl,
  Children: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT, // 1
}

define_interface!(
  IDependencyObject,
  IDependencyObject_Vtbl,
  0xe7beaee7_160e_50f7_8789_d63463f979fa
);
#[repr(C)]
pub struct IDependencyObject_Vtbl {
  base__: IInspectable_Vtbl,
}

define_interface!(
  IXamlRoot,
  IXamlRoot_Vtbl,
  0x60cb215a_ad15_520a_8b01_4416824f0441
);
#[repr(C)]
pub struct IXamlRoot_Vtbl {
  base__: IInspectable_Vtbl,
  _1: usize,
  Size: unsafe extern "system" fn(Raw, *mut ::windows::Foundation::Size) -> HRESULT, // 2
  RasterizationScale: unsafe extern "system" fn(Raw, *mut f64) -> HRESULT,           // 3
}

define_interface!(
  IGeneralTransform,
  IGeneralTransform_Vtbl,
  0x04eedeeb_31e5_54c0_ae3f_8bd06645d339
);
#[repr(C)]
pub struct IGeneralTransform_Vtbl {
  base__: IInspectable_Vtbl,
  _1: usize,
  TransformPoint: unsafe extern "system" fn(Raw, Point, *mut Point) -> HRESULT, // 2
}

define_interface!(
  RectangleGeometry,
  IRectangleGeometry_Vtbl,
  0xb6143890_a5f5_54e0_ab42_d88bab451f04
);
#[repr(C)]
pub struct IRectangleGeometry_Vtbl {
  base__: IInspectable_Vtbl,
  _1: usize,
  SetRect: unsafe extern "system" fn(Raw, Rect) -> HRESULT, // 2
}

define_interface!(
  MatrixTransform,
  IMatrixTransform_Vtbl,
  0xf03138e1_addd_59fa_b993_3ea69b888ace
);
#[repr(C)]
pub struct IMatrixTransform_Vtbl {
  base__: IInspectable_Vtbl,
  _1: usize,
  SetMatrix: unsafe extern "system" fn(Raw, Matrix) -> HRESULT, // 2
}

define_interface!(
  RenderTargetBitmap,
  IRenderTargetBitmap_Vtbl,
  0xcf10407d_fa8b_57a3_9574_710529ae0b04
);
#[repr(C)]
pub struct IRenderTargetBitmap_Vtbl {
  base__: IInspectable_Vtbl,
  PixelWidth: unsafe extern "system" fn(Raw, *mut i32) -> HRESULT, // 1
  PixelHeight: unsafe extern "system" fn(Raw, *mut i32) -> HRESULT,
  _3: usize,
  RenderToSizeAsync: unsafe extern "system" fn(Raw, Raw, i32, i32, *mut Raw) -> HRESULT, // 4
  GetPixelsAsync: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT,
}

/// ICanvasFactory and IGridFactory: `CreateInstance(outer, inner, instance)`.
#[repr(C)]
pub struct IComposableFactory_Vtbl {
  base__: IInspectable_Vtbl,
  CreateInstance: unsafe extern "system" fn(Raw, Raw, *mut Raw, *mut Raw) -> HRESULT, // 1
}
define_interface!(
  ICanvasFactory,
  IComposableFactory_Vtbl,
  0x374c5050_3481_5557_9948_804c0b8eea89
);
define_interface!(
  IGridFactory,
  IComposableFactory_Vtbl,
  0xb16bf561_fc6c_57c6_8ebc_0b06ce4513aa
);

define_interface!(
  ICanvasStatics,
  ICanvasStatics_Vtbl,
  0xc00d5e0f_77e3_5c59_8fcd_86761f0c6607
);
#[repr(C)]
pub struct ICanvasStatics_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 8],
  SetZIndex: unsafe extern "system" fn(Raw, Raw, i32) -> HRESULT, // 9
}

define_interface!(
  IVisualTreeHelperStatics,
  IVisualTreeHelperStatics_Vtbl,
  0x5aece43c_7651_5bb5_855c_2198496e455e
);
#[repr(C)]
pub struct IVisualTreeHelperStatics_Vtbl {
  base__: IInspectable_Vtbl,
  _1: [usize; 6],
  GetParent: unsafe extern "system" fn(Raw, Raw, *mut Raw) -> HRESULT, // 7
}

interface_hierarchy!(IUIElement, IUnknown, IInspectable);
interface_hierarchy!(IUIElementStatics, IUnknown, IInspectable);
interface_hierarchy!(IPointerRoutedEventArgs, IUnknown, IInspectable);
interface_hierarchy!(IPointerPoint, IUnknown, IInspectable);
interface_hierarchy!(IFrameworkElement, IUnknown, IInspectable);
interface_hierarchy!(IPanel, IUnknown, IInspectable);
interface_hierarchy!(IDependencyObject, IUnknown, IInspectable);
interface_hierarchy!(IXamlRoot, IUnknown, IInspectable);
interface_hierarchy!(IGeneralTransform, IUnknown, IInspectable);
interface_hierarchy!(RectangleGeometry, IUnknown, IInspectable);
interface_hierarchy!(MatrixTransform, IUnknown, IInspectable);
interface_hierarchy!(RenderTargetBitmap, IUnknown, IInspectable);

macro_rules! class {
  ($name:ident, $runtime:literal) => {
    pub struct $name;
    impl windows_core::RuntimeName for $name {
      const NAME: &'static str = $runtime;
    }
  };
}
class!(CanvasClass, "Microsoft.UI.Xaml.Controls.Canvas");
class!(GridClass, "Microsoft.UI.Xaml.Controls.Grid");
class!(
  VisualTreeHelperClass,
  "Microsoft.UI.Xaml.Media.VisualTreeHelper"
);
class!(
  RenderTargetBitmapClass,
  "Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap"
);
class!(
  MatrixTransformClass,
  "Microsoft.UI.Xaml.Media.MatrixTransform"
);
class!(
  RectangleGeometryClass,
  "Microsoft.UI.Xaml.Media.RectangleGeometry"
);

/// Any element, held as its `IUIElement`; other interfaces are queried per call. It is also the
/// `Microsoft.UI.Xaml.UIElement` of `Panel.Children` (`IVector<UIElement>`).
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UIElement(IUnknown);
interface_hierarchy!(UIElement, IUnknown, IInspectable);
unsafe impl Interface for UIElement {
  type Vtable = IUIElement_Vtbl;
  const IID: GUID = <IUIElement as Interface>::IID;
}
impl windows_core::RuntimeName for UIElement {
  const NAME: &'static str = "Microsoft.UI.Xaml.UIElement";
}
impl windows_core::RuntimeType for UIElement {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}

/// The UIElement events the host listens to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
  PointerPressed,
  PointerMoved,
  PointerReleased,
  PointerWheelChanged,
  KeyDown,
  KeyUp,
}

impl Event {
  pub const ALL: [Event; 6] = [
    Event::PointerPressed,
    Event::PointerMoved,
    Event::PointerReleased,
    Event::PointerWheelChanged,
    Event::KeyDown,
    Event::KeyUp,
  ];

  /// The delegate type the event takes (PointerEventHandler or KeyEventHandler).
  fn delegate(self) -> GUID {
    match self {
      Event::KeyDown | Event::KeyUp => GUID::from_u128(0xdb68e7cc_9a2b_527d_9989_25284daccc03),
      _ => POINTER_EVENT_HANDLER,
    }
  }
}

const POINTER_EVENT_HANDLER: GUID = GUID::from_u128(0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb);

/// `UIElement.PointerEnteredEvent` and `PointerMovedEvent`.
fn pointer_events() -> Result<[IInspectable; 2]> {
  static STATICS: FactoryCache<UIElement, IUIElementStatics> = FactoryCache::new();
  STATICS.call(|s| unsafe {
    let (mut entered, mut moved) = (std::ptr::null_mut(), std::ptr::null_mut());
    let entered = object(
      (s.vtable().PointerEnteredEvent)(s.as_raw(), &mut entered),
      entered,
    )?;
    let moved = object(
      (s.vtable().PointerMovedEvent)(s.as_raw(), &mut moved),
      moved,
    )?;
    Ok([entered, moved])
  })
}

/// Pointer handlers added by `UIElement::watch_pointer`; dropping it removes them.
pub struct PointerWatch {
  element: UIElement,
  /// The boxed delegate.
  handler: IInspectable,
}

impl Drop for PointerWatch {
  fn drop(&mut self) {
    if let Ok(events) = pointer_events() {
      for event in events {
        let v = self.element.vtable();
        let _ = unsafe {
          (v.RemoveHandler)(self.element.as_raw(), event.as_raw(), self.handler.as_raw())
        };
      }
    }
  }
}

fn object<T: Type<T, Abi = Raw>>(result: HRESULT, raw: Raw) -> Result<T> {
  result.and_then(|| unsafe { Type::from_abi(raw) })
}

fn raw_of(element: Option<&impl Interface>) -> Raw {
  element.map_or(std::ptr::null_mut(), |e| e.as_raw())
}

impl UIElement {
  fn composed(factory: &impl Interface<Vtable = IComposableFactory_Vtbl>) -> Result<UIElement> {
    let (mut inner, mut instance) = (std::ptr::null_mut(), std::ptr::null_mut());
    let result = unsafe {
      (factory.vtable().CreateInstance)(
        factory.as_raw(),
        std::ptr::null_mut(),
        &mut inner,
        &mut instance,
      )
    };
    object::<IInspectable>(result, instance)?.cast()
  }

  pub fn new_canvas() -> Result<UIElement> {
    static FACTORY: FactoryCache<CanvasClass, ICanvasFactory> = FactoryCache::new();
    FACTORY.call(|f| Self::composed(f))
  }

  pub fn new_grid() -> Result<UIElement> {
    static FACTORY: FactoryCache<GridClass, IGridFactory> = FactoryCache::new();
    FACTORY.call(|f| Self::composed(f))
  }

  /// `Canvas.SetZIndex(this, z)`.
  pub fn set_z_index(&self, z: i32) -> Result<()> {
    static STATICS: FactoryCache<CanvasClass, ICanvasStatics> = FactoryCache::new();
    STATICS.call(|s| unsafe { (s.vtable().SetZIndex)(s.as_raw(), self.as_raw(), z).ok() })
  }

  /// `VisualTreeHelper.GetParent`.
  pub fn parent_of(node: &IDependencyObject) -> Result<IDependencyObject> {
    static STATICS: FactoryCache<VisualTreeHelperClass, IVisualTreeHelperStatics> =
      FactoryCache::new();
    STATICS.call(|s| unsafe {
      let mut parent = std::ptr::null_mut();
      object(
        (s.vtable().GetParent)(s.as_raw(), node.as_raw(), &mut parent),
        parent,
      )
    })
  }

  pub fn SetOpacity(&self, value: f64) -> Result<()> {
    unsafe { (self.vtable().SetOpacity)(self.as_raw(), value).ok() }
  }

  pub fn SetClip(&self, clip: Option<&RectangleGeometry>) -> Result<()> {
    unsafe { (self.vtable().SetClip)(self.as_raw(), raw_of(clip)).ok() }
  }

  pub fn SetRenderTransform(&self, transform: Option<&MatrixTransform>) -> Result<()> {
    unsafe { (self.vtable().SetRenderTransform)(self.as_raw(), raw_of(transform)).ok() }
  }

  pub fn SetIsHitTestVisible(&self, value: bool) -> Result<()> {
    unsafe { (self.vtable().SetIsHitTestVisible)(self.as_raw(), value).ok() }
  }

  pub fn UseLayoutRounding(&self) -> Result<bool> {
    let mut value = false;
    unsafe { (self.vtable().UseLayoutRounding)(self.as_raw(), &mut value).map(|| value) }
  }

  pub fn SetUseLayoutRounding(&self, value: bool) -> Result<()> {
    unsafe { (self.vtable().SetUseLayoutRounding)(self.as_raw(), value).ok() }
  }

  /// `XamlRoot.RasterizationScale` of the element's root.
  pub fn root_scale(&self) -> Result<f64> {
    let root = self.root()?;
    let mut scale = 0.;
    unsafe { (root.vtable().RasterizationScale)(root.as_raw(), &mut scale).map(|| scale) }
  }

  fn root(&self) -> Result<IXamlRoot> {
    let mut root = std::ptr::null_mut();
    object(
      unsafe { (self.vtable().XamlRoot)(self.as_raw(), &mut root) },
      root,
    )
  }

  /// `XamlRoot.Size` (DIP) of the element's root.
  pub fn root_size(&self) -> Result<::windows::Foundation::Size> {
    let root = self.root()?;
    let mut size = ::windows::Foundation::Size::default();
    unsafe { (root.vtable().Size)(root.as_raw(), &mut size).map(|| size) }
  }

  /// The element's top left in its root's coordinates (DIP).
  pub fn root_origin(&self) -> Result<Point> {
    let mut transform = std::ptr::null_mut();
    let transform: IGeneralTransform = object(
      unsafe {
        (self.vtable().TransformToVisual)(self.as_raw(), std::ptr::null_mut(), &mut transform)
      },
      transform,
    )?;
    let mut out = Point::default();
    unsafe {
      (transform.vtable().TransformPoint)(transform.as_raw(), Point::default(), &mut out)
        .map(|| out)
    }
  }

  pub fn RasterizationScale(&self) -> Result<f64> {
    let mut value = 0.;
    unsafe { (self.vtable().RasterizationScale)(self.as_raw(), &mut value).map(|| value) }
  }

  pub fn SetRasterizationScale(&self, value: f64) -> Result<()> {
    unsafe { (self.vtable().SetRasterizationScale)(self.as_raw(), value).ok() }
  }

  /// Where this element's (x, y) lands in `visual`.
  pub fn point_in(&self, visual: &UIElement, point: Point) -> Result<Point> {
    let mut transform = std::ptr::null_mut();
    let transform: IGeneralTransform = object(
      unsafe { (self.vtable().TransformToVisual)(self.as_raw(), visual.as_raw(), &mut transform) },
      transform,
    )?;
    let mut out = Point::default();
    unsafe { (transform.vtable().TransformPoint)(transform.as_raw(), point, &mut out).map(|| out) }
  }

  /// Calls `callback` (on the UI thread) on each `event`; returns the registration token.
  pub fn add_handler(&self, event: Event, callback: impl Fn() + Send + 'static) -> Result<i64> {
    let handler = Handler::new(event.delegate(), move |_, _| callback());
    let v = self.vtable();
    let add = match event {
      Event::PointerPressed => v.PointerPressed,
      Event::PointerMoved => v.PointerMoved,
      Event::PointerReleased => v.PointerReleased,
      Event::PointerWheelChanged => v.PointerWheelChanged,
      Event::KeyDown => v.KeyDown,
      Event::KeyUp => v.KeyUp,
    };
    let mut token = 0;
    unsafe { add(self.as_raw(), handler.as_raw(), &mut token).map(|| token) }
  }

  /// Calls `callback` with the pointer's position in this element (DIP) on every PointerEntered
  /// and PointerMoved that reaches it, handled or not. XAML has hit-tested the event by then.
  pub fn watch_pointer(&self, callback: impl Fn(Point) + 'static) -> Result<PointerWatch> {
    let handler = Handler::new(POINTER_EVENT_HANDLER, move |sender, args| {
      if let Some(point) = position(sender, args) {
        callback(point);
      }
    })
    .cast::<IInspectable>()?;
    for event in pointer_events()? {
      let v = self.vtable();
      unsafe { (v.AddHandler)(self.as_raw(), event.as_raw(), handler.as_raw(), true).ok()? };
    }
    Ok(PointerWatch {
      element: self.clone(),
      handler,
    })
  }

  pub fn remove_handler(&self, event: Event, token: i64) -> Result<()> {
    let v = self.vtable();
    let remove = match event {
      Event::PointerPressed => v.RemovePointerPressed,
      Event::PointerMoved => v.RemovePointerMoved,
      Event::PointerReleased => v.RemovePointerReleased,
      Event::PointerWheelChanged => v.RemovePointerWheelChanged,
      Event::KeyDown => v.RemoveKeyDown,
      Event::KeyUp => v.RemoveKeyUp,
    };
    unsafe { remove(self.as_raw(), token).ok() }
  }

  fn framework(&self) -> Result<IFrameworkElement> {
    self.cast()
  }

  pub fn ActualWidth(&self) -> Result<f64> {
    let e = self.framework()?;
    let mut value = 0.;
    unsafe { (e.vtable().ActualWidth)(e.as_raw(), &mut value).map(|| value) }
  }

  pub fn ActualHeight(&self) -> Result<f64> {
    let e = self.framework()?;
    let mut value = 0.;
    unsafe { (e.vtable().ActualHeight)(e.as_raw(), &mut value).map(|| value) }
  }

  pub fn SetWidth(&self, value: f64) -> Result<()> {
    let e = self.framework()?;
    unsafe { (e.vtable().SetWidth)(e.as_raw(), value).ok() }
  }

  /// Aligns the element to its parent's top left.
  pub fn align_start(&self) -> Result<()> {
    let e = self.framework()?;
    unsafe {
      (e.vtable().SetHorizontalAlignment)(e.as_raw(), ALIGN_START).ok()?;
      (e.vtable().SetVerticalAlignment)(e.as_raw(), ALIGN_START).ok()
    }
  }

  /// `Panel.Children`.
  pub fn Children(&self) -> Result<IVector<UIElement>> {
    let panel: IPanel = self.cast()?;
    let mut children = std::ptr::null_mut();
    object(
      unsafe { (panel.vtable().Children)(panel.as_raw(), &mut children) },
      children,
    )
  }
}

/// `args.GetCurrentPoint(sender).Position`.
fn position(sender: Raw, args: Raw) -> Option<Point> {
  let sender = unsafe { IInspectable::from_raw_borrowed(&sender) }?
    .cast::<UIElement>()
    .ok()?;
  let args = unsafe { IInspectable::from_raw_borrowed(&args) }?
    .cast::<IPointerRoutedEventArgs>()
    .ok()?;
  let mut point = std::ptr::null_mut();
  let point: IPointerPoint = object(
    unsafe { (args.vtable().GetCurrentPoint)(args.as_raw(), sender.as_raw(), &mut point) },
    point,
  )
  .ok()?;
  let mut out = Point::default();
  unsafe { (point.vtable().Position)(point.as_raw(), &mut out) }
    .ok()
    .ok()?;
  Some(out)
}

impl RectangleGeometry {
  pub fn new() -> Result<Self> {
    static FACTORY: FactoryCache<RectangleGeometryClass, IGenericFactory> = FactoryCache::new();
    FACTORY.call(|f| f.ActivateInstance())
  }

  pub fn SetRect(&self, rect: Rect) -> Result<()> {
    unsafe { (self.vtable().SetRect)(self.as_raw(), rect).ok() }
  }
}

impl MatrixTransform {
  pub fn new() -> Result<Self> {
    static FACTORY: FactoryCache<MatrixTransformClass, IGenericFactory> = FactoryCache::new();
    FACTORY.call(|f| f.ActivateInstance())
  }

  pub fn SetMatrix(&self, matrix: Matrix) -> Result<()> {
    unsafe { (self.vtable().SetMatrix)(self.as_raw(), matrix).ok() }
  }
}

impl RenderTargetBitmap {
  pub fn new() -> Result<Self> {
    static FACTORY: FactoryCache<RenderTargetBitmapClass, IGenericFactory> = FactoryCache::new();
    FACTORY.call(|f| f.ActivateInstance())
  }

  pub fn PixelWidth(&self) -> Result<i32> {
    let mut value = 0;
    unsafe { (self.vtable().PixelWidth)(self.as_raw(), &mut value).map(|| value) }
  }

  pub fn PixelHeight(&self) -> Result<i32> {
    let mut value = 0;
    unsafe { (self.vtable().PixelHeight)(self.as_raw(), &mut value).map(|| value) }
  }

  pub fn RenderToSizeAsync(
    &self,
    element: &UIElement,
    width: i32,
    height: i32,
  ) -> Result<IAsyncAction> {
    let mut op = std::ptr::null_mut();
    object(
      unsafe {
        (self.vtable().RenderToSizeAsync)(self.as_raw(), element.as_raw(), width, height, &mut op)
      },
      op,
    )
  }

  pub fn GetPixelsAsync(&self) -> Result<IAsyncOperation<IBuffer>> {
    let mut op = std::ptr::null_mut();
    object(
      unsafe { (self.vtable().GetPixelsAsync)(self.as_raw(), &mut op) },
      op,
    )
  }
}

/// A PointerEventHandler or KeyEventHandler delegate. XAML invokes it on the UI thread only.
/// `UIElement.AddHandler` takes the delegate boxed, so it is also its own
/// `IReference<PointerEventHandler>`.
#[repr(C)]
struct Handler<F: Fn(Raw, Raw) + 'static> {
  vtable: *const HandlerVtbl,
  reference: *const ReferenceVtbl,
  iid: GUID,
  count: windows_core::imp::RefCount,
  callback: F,
}

#[repr(C)]
struct HandlerVtbl {
  base__: windows_core::IUnknown_Vtbl,
  Invoke: unsafe extern "system" fn(Raw, Raw, Raw) -> HRESULT,
}

#[repr(C)]
struct ReferenceVtbl {
  base__: IInspectable_Vtbl,
  Value: unsafe extern "system" fn(Raw, *mut Raw) -> HRESULT,
}

/// `IReference<PointerEventHandler>`.
const POINTER_EVENT_HANDLER_REFERENCE: GUID = GUID::from_signature(
  windows_core::imp::ConstBuffer::from_slice(
    b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472};delegate({a48a71e1-8bb4-5597-9e31-903a3f6a04fb}))",
  ),
);

impl<F: Fn(Raw, Raw) + 'static> Handler<F> {
  const VTABLE: HandlerVtbl = HandlerVtbl {
    base__: windows_core::IUnknown_Vtbl {
      QueryInterface: Self::QueryInterface,
      AddRef: Self::AddRef,
      Release: Self::Release,
    },
    Invoke: Self::Invoke,
  };

  const REFERENCE: ReferenceVtbl = ReferenceVtbl {
    base__: IInspectable_Vtbl {
      base: windows_core::IUnknown_Vtbl {
        QueryInterface: Self::ReferenceQueryInterface,
        AddRef: Self::ReferenceAddRef,
        Release: Self::ReferenceRelease,
      },
      GetIids: Self::GetIids,
      GetRuntimeClassName: Self::GetRuntimeClassName,
      GetTrustLevel: Self::GetTrustLevel,
    },
    Value: Self::Value,
  };

  fn new(iid: GUID, callback: F) -> IUnknown {
    let this = Box::new(Self {
      vtable: &Self::VTABLE,
      reference: &Self::REFERENCE,
      iid,
      count: windows_core::imp::RefCount::new(1),
      callback,
    });
    unsafe { IUnknown::from_raw(Box::into_raw(this) as Raw) }
  }

  /// The handler of its `IReference` interface pointer.
  fn of_reference(this: Raw) -> Raw {
    unsafe { (this as *mut *const ReferenceVtbl).sub(1) as Raw }
  }

  unsafe extern "system" fn QueryInterface(this: Raw, iid: *const GUID, out: *mut Raw) -> HRESULT {
    unsafe {
      let this = this as *mut Self;
      if iid.is_null() || out.is_null() {
        return HRESULT(-2147467261); // E_POINTER
      }
      let iid = *iid;
      if iid == (*this).iid
        || iid == <IUnknown as Interface>::IID
        || iid == <windows_core::imp::IAgileObject as Interface>::IID
      {
        *out = this as Raw;
      } else if iid == <windows_core::imp::IMarshal as Interface>::IID {
        (*this).count.add_ref();
        return windows_core::imp::marshaler(IUnknown::from_raw(this as Raw), out);
      } else if (*this).iid == POINTER_EVENT_HANDLER
        && (iid == POINTER_EVENT_HANDLER_REFERENCE || iid == <IInspectable as Interface>::IID)
      {
        *out = &mut (*this).reference as *mut _ as Raw;
      } else {
        *out = std::ptr::null_mut();
        return HRESULT(-2147467262); // E_NOINTERFACE
      }
      (*this).count.add_ref();
      HRESULT(0)
    }
  }

  unsafe extern "system" fn AddRef(this: Raw) -> u32 {
    unsafe { (*(this as *mut Self)).count.add_ref() }
  }

  unsafe extern "system" fn Release(this: Raw) -> u32 {
    unsafe {
      let this = this as *mut Self;
      let remaining = (*this).count.release();
      if remaining == 0 {
        drop(Box::from_raw(this));
      }
      remaining
    }
  }

  unsafe extern "system" fn Invoke(this: Raw, sender: Raw, args: Raw) -> HRESULT {
    unsafe { ((*(this as *mut Self)).callback)(sender, args) };
    HRESULT(0)
  }

  unsafe extern "system" fn ReferenceQueryInterface(
    this: Raw,
    iid: *const GUID,
    out: *mut Raw,
  ) -> HRESULT {
    unsafe { Self::QueryInterface(Self::of_reference(this), iid, out) }
  }

  unsafe extern "system" fn ReferenceAddRef(this: Raw) -> u32 {
    unsafe { Self::AddRef(Self::of_reference(this)) }
  }

  unsafe extern "system" fn ReferenceRelease(this: Raw) -> u32 {
    unsafe { Self::Release(Self::of_reference(this)) }
  }

  unsafe extern "system" fn GetIids(_: Raw, count: *mut u32, values: *mut *mut GUID) -> HRESULT {
    unsafe {
      *count = 0;
      *values = std::ptr::null_mut();
    }
    HRESULT(0)
  }

  unsafe extern "system" fn GetRuntimeClassName(_: Raw, value: *mut Raw) -> HRESULT {
    unsafe { *value = std::ptr::null_mut() };
    HRESULT(0)
  }

  unsafe extern "system" fn GetTrustLevel(_: Raw, value: *mut i32) -> HRESULT {
    unsafe { *value = 0 };
    HRESULT(0)
  }

  unsafe extern "system" fn Value(this: Raw, out: *mut Raw) -> HRESULT {
    unsafe {
      let this = Self::of_reference(this);
      Self::AddRef(this);
      *out = this;
    }
    HRESULT(0)
  }
}

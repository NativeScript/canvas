//! SMIL animation, which Skia's SVG parser drops. Animation elements are lifted out of the
//! source and replayed each frame as ordinary attribute writes.

mod color;
mod css;
mod motion;
mod parse;
mod timing;
mod value;

pub use parse::extract;
pub(crate) use css::extract_from_css;

use skia_safe::svg::{Dom, Node};

use timing::Timing;
use value::Value;

#[derive(Clone, Debug)]
pub enum Kind {
    /// `<animate>` / `<set>` on a named attribute.
    Attribute(String),
    Transform(TransformKind),
    Motion(motion::Motion),
}

impl Kind {
    /// Transform and motion animations share one `transform` slot: they compose, not overwrite.
    fn slot(&self) -> &str {
        match self {
            Kind::Attribute(name) => name,
            Kind::Transform(_) | Kind::Motion(_) => "transform",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransformKind {
    Translate,
    Scale,
    Rotate,
    SkewX,
    SkewY,
}

impl TransformKind {
    fn parse(value: &str) -> Self {
        match value.trim() {
            "scale" => TransformKind::Scale,
            "rotate" => TransformKind::Rotate,
            "skewX" => TransformKind::SkewX,
            "skewY" => TransformKind::SkewY,
            _ => TransformKind::Translate,
        }
    }

    fn function(self) -> &'static str {
        match self {
            TransformKind::Translate => "translate",
            TransformKind::Scale => "scale",
            TransformKind::Rotate => "rotate",
            TransformKind::SkewX => "skewX",
            TransformKind::SkewY => "skewY",
        }
    }

    /// Identity: the start value for a transform animation with only `to` or `by`.
    fn neutral(self) -> Value {
        Value::Numbers {
            values: match self {
                TransformKind::Scale => vec![1.0],
                _ => vec![0.0],
            },
            unit: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Animation {
    /// Target element id; assigned during extraction if the source had none.
    pub target: String,
    /// Ids of enclosing elements, nearest first. Only id-carrying ancestors appear, which is
    /// enough since a promoted layer is named by its id.
    pub ancestors: Vec<String>,
    pub kind: Kind,
    pub timing: Timing,
    /// Document order, the tie-break when several animations drive one attribute.
    pub order: usize,
    frames: Frames,
}

/// `to`/`by` without `from` start from the attribute's current value, only readable once the
/// document exists.
#[derive(Clone, Debug)]
enum Frames {
    Values(Vec<Value>),
    ToOnly(Value),
    ByOnly(Value),
}

struct Contribution {
    animation: usize,
    value: Value,
    additive: bool,
}

/// Every animation driving one attribute of one element.
struct Group {
    target: String,
    ancestors: Vec<String>,
    attribute: String,
    /// Indices into `Timeline::animations`, in document order.
    members: Vec<usize>,
    node: Option<Node>,
    looked_up: bool,
    /// Pre-animation value for `fill="remove"`. Outer `None` means not captured yet.
    baseline: Option<Option<String>>,
    base: Option<Value>,
    /// Whether the last frame wrote anything, so a group that goes quiet is reverted once.
    written: bool,
    /// Skips unchanged writes; otherwise a frozen animation writes every frame and a promoted
    /// layer drops its cached backdrop forever.
    last_written: String,
    buffer: Vec<Contribution>,
}

#[derive(Default)]
pub struct Timeline {
    animations: Vec<Animation>,
    groups: Vec<Group>,
    time: f64,
}

impl Timeline {
    pub fn new(animations: Vec<Animation>) -> Self {
        let mut groups: Vec<Group> = Vec::new();
        for (index, animation) in animations.iter().enumerate() {
            let slot = animation.kind.slot();
            match groups
                .iter_mut()
                .find(|g| g.target == animation.target && g.attribute == slot)
            {
                Some(group) => group.members.push(index),
                None => groups.push(Group {
                    target: animation.target.clone(),
                    ancestors: animation.ancestors.clone(),
                    attribute: slot.to_owned(),
                    members: vec![index],
                    node: None,
                    looked_up: false,
                    baseline: None,
                    base: None,
                    written: false,
                    last_written: String::new(),
                    buffer: Vec::new(),
                }),
            }
        }
        Self {
            animations,
            groups,
            time: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }

    /// Merges animations added after construction into their (target, attribute) groups. Order
    /// is renumbered across the combined set; it is only a tie-break.
    pub fn extend(&mut self, added: Vec<Animation>) {
        if added.is_empty() {
            return;
        }
        let start = self.animations.len();
        self.animations.extend(added);
        for index in start..self.animations.len() {
            let animation = &self.animations[index];
            let slot = animation.kind.slot().to_owned();
            match self
                .groups
                .iter_mut()
                .find(|g| g.target == animation.target && g.attribute == slot)
            {
                Some(group) => group.members.push(index),
                None => self.groups.push(Group {
                    target: animation.target.clone(),
                    ancestors: animation.ancestors.clone(),
                    attribute: slot,
                    members: vec![index],
                    node: None,
                    looked_up: false,
                    baseline: None,
                    base: None,
                    written: false,
                    last_written: String::new(),
                    buffer: Vec::new(),
                }),
            }
        }
        for (order, animation) in self.animations.iter_mut().enumerate() {
            animation.order = order;
        }
    }

    pub fn len(&self) -> usize {
        self.animations.len()
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    /// False once every animation is past its active end, so the caller can stop scheduling frames.
    pub fn is_running(&self) -> bool {
        self.animations
            .iter()
            .any(|animation| animation.timing.is_running(self.time))
    }

    /// The moment past which nothing changes again, or `None` when something repeats forever.
    pub fn duration(&self) -> Option<f64> {
        self.animations
            .iter()
            .try_fold(0.0f64, |longest, animation| {
                animation.timing.end().map(|end| longest.max(end))
            })
    }

    /// Moves the clock to `seconds` and writes every animated attribute. `layer` is the promoted
    /// node's id; writes inside it leave a cached backdrop valid.
    pub fn apply(&mut self, dom: &mut Dom, seconds: f64, layer: Option<&str>) -> Applied {
        self.time = seconds.max(0.0);
        let mut changed = false;
        let mut changed_outside_layer = false;
        let mut changed_layer_content = false;

        let Self {
            animations,
            groups,
            time,
            ..
        } = self;

        for group in groups.iter_mut() {
            group.buffer.clear();

            // Capture the baseline before the first write overwrites it.
            let node = group.resolve(dom);
            if let (Some(node), None) = (&node, &group.baseline) {
                let raw = crate::get_attribute(&node.clone().typed(), &group.attribute);
                group.base = raw.as_deref().map(Value::parse);
                group.baseline = Some(raw);
            }
            let baseline = group.baseline.as_ref().and_then(|b| b.as_deref());

            for &index in &group.members {
                if let Some(contribution) =
                    animations[index].contribution(*time, index, group.base.as_ref())
                {
                    group.buffer.push(contribution);
                }
            }

            if group.buffer.is_empty() {
                // `fill="remove"`: restore the baseline once.
                if group.written {
                    group.written = false;
                    group.last_written.clear();
                    changed = true;
                    if group.outside_layer(layer) {
                        changed_outside_layer = true;
                    } else if !group.is_layer_transform(layer) {
                        changed_layer_content = true;
                    }
                    // Skia cannot unset an attribute (an empty string is rejected), so with no
                    // baseline the last value stays.
                    if let (Some(node), Some(previous)) = (&node, baseline) {
                        let previous = previous.to_owned();
                        crate::set_attribute(&mut node.clone().typed(), &group.attribute, &previous);
                    }
                }
                continue;
            }

            let Some(node) = node else {
                continue;
            };
            let mut typed = node.typed();

            let written = if group.attribute == "transform" {
                compose_transform(baseline, &group.buffer, animations)
            } else {
                compose_value(group.base.as_ref(), &group.buffer)
            };
            if written != group.last_written {
                crate::set_attribute(&mut typed, &group.attribute, &written);
                group.last_written = written;
                changed = true;
                if group.outside_layer(layer) {
                    changed_outside_layer = true;
                } else if !group.is_layer_transform(layer) {
                    changed_layer_content = true;
                }
            }
            group.written = true;
        }

        Applied {
            changed,
            changed_outside_layer,
            changed_layer_content,
            running: self.is_running(),
        }
    }
}

pub struct Applied {
    pub changed: bool,
    /// A write landed outside the promoted subtree, so the cached backdrop is stale. Always
    /// true when nothing is promoted.
    pub changed_outside_layer: bool,
    /// A write inside the promoted subtree other than its root's `transform`, so the layer
    /// raster cannot be reused.
    pub changed_layer_content: bool,
    pub running: bool,
}

impl Group {
    fn outside_layer(&self, layer: Option<&str>) -> bool {
        match layer {
            None => true,
            Some(layer) => self.target != layer && !self.ancestors.iter().any(|a| a == layer),
        }
    }

    /// The promoted node's own `transform`: the one write a cached layer raster absorbs by
    /// being drawn elsewhere.
    fn is_layer_transform(&self, layer: Option<&str>) -> bool {
        layer.is_some_and(|layer| self.target == layer && self.attribute == "transform")
    }

    fn resolve(&mut self, dom: &mut Dom) -> Option<Node> {
        // Cached even when missing; an unresolved id never starts resolving.
        if !self.looked_up {
            self.looked_up = true;
            self.node = dom.find_node_by_id(&self.target);
        }
        self.node.clone()
    }
}

fn compose_value(base: Option<&Value>, contributions: &[Contribution]) -> String {
    let mut current: Option<Value> = base.cloned();
    for contribution in contributions {
        current = Some(match (contribution.additive, &current) {
            (true, Some(under)) => under.add(&contribution.value),
            _ => contribution.value.clone(),
        });
    }
    current.map(|value| value.to_attribute()).unwrap_or_default()
}

/// Contributions concatenate; the element's own transform is kept as a prefix when the first
/// is additive.
fn compose_transform(
    baseline: Option<&str>,
    contributions: &[Contribution],
    animations: &[Animation],
) -> String {
    let mut parts: Vec<String> = Vec::new();

    if contributions.first().is_some_and(|c| c.additive) {
        if let Some(existing) = baseline.map(str::trim).filter(|t| !t.is_empty()) {
            parts.push(existing.to_owned());
        }
    }

    for contribution in contributions {
        let value = contribution.value.to_attribute();
        match &animations[contribution.animation].kind {
            Kind::Transform(kind) => parts.push(format!("{}({})", kind.function(), value)),
            // Motion already produces complete transform functions.
            _ => parts.push(value),
        }
    }

    parts.join(" ")
}

impl Animation {
    /// `None` before it begins or after it ends with `fill="remove"`.
    fn contribution(&self, time: f64, index: usize, base: Option<&Value>) -> Option<Contribution> {
        let sample = self.timing.sample(time)?;

        let value = match &self.kind {
            Kind::Motion(motion) => motion.sample(self.timing.ease(sample.fraction)),

            // A transform's base is the identity; `compose_transform` keeps the element's own
            // `transform` as a prefix instead.
            kind => {
                let base = match kind {
                    Kind::Transform(transform) => Some(transform.neutral()),
                    _ => base.cloned(),
                };
                match &self.frames {
                    Frames::Values(values) => self.evaluate(values, &sample),
                    Frames::ToOnly(to) => {
                        let start = base.unwrap_or_else(|| to.clone());
                        self.evaluate(&[start, to.clone()], &sample)
                    }
                    Frames::ByOnly(by) => {
                        let start = base?;
                        let end = start.add(by);
                        self.evaluate(&[start, end], &sample)
                    }
                }
            }
        };

        Some(Contribution {
            animation: index,
            value,
            additive: self.timing.additive,
        })
    }

    fn evaluate(&self, frames: &[Value], sample: &timing::Sample) -> Value {
        let mut value = self.timing.interpolate(frames, sample.fraction);
        if self.timing.accumulate && sample.iteration > 0.0 {
            if let Some(last) = frames.last() {
                value = value.add(&last.scale(sample.iteration));
            }
        }
        value
    }
}

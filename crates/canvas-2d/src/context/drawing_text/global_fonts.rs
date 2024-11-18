use std::collections::HashMap;

use parking_lot::Mutex;
use skia_safe::font_arguments::variation_position::Coordinate;
use skia_safe::font_arguments::VariationPosition;
use skia_safe::font_style::Slant;
use skia_safe::textlayout::{FontCollection, TextStyle, TypefaceFontProvider};
use skia_safe::{FontArguments, FontMgr, Typeface};
use std::sync::LazyLock;

#[derive(PartialEq, Eq, Hash)]
struct CollectionKey {
    families: String,
    weight: i32,
    slant: Slant,
}

impl CollectionKey {
    pub fn new(style: &TextStyle) -> Self {
        let families = style.font_families();
        let families = families.iter().collect::<Vec<&str>>().join(", ");
        let weight = *style.font_style().weight();
        let slant = style.font_style().slant();
        CollectionKey {
            families,
            weight,
            slant,
        }
    }
}

//
// Font collection management
//

pub static FONT_LIBRARY: LazyLock<Mutex<FontLibrary>> = LazyLock::new(|| FontLibrary::shared());

pub struct FontLibrary {
    pub fonts: Vec<(Typeface, Option<String>)>,
    pub collection: FontCollection,
    collection_cache: HashMap<CollectionKey, FontCollection>,
}

unsafe impl Send for FontLibrary {
    // famous last words: this ‘should’ be safe in practice because the singleton is behind a mutex
}

pub enum CanvasFontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub enum CanvasFontDisplay {
    Auto,
    Block,
    Fallback,
    Optional,
    Swap,
}

pub struct FontDescriptors {
    weight: CanvasFontWeight,
    family: String,
    ascent_override: String,
    descent_override: String,
    display: CanvasFontDisplay,
    style: String,
    stretch: String,
    unicode_range: String,
    feature_settings: String,
    line_gap_override: String,
    variation_settings: String,
}


impl FontLibrary {
    pub fn shared() -> Mutex<Self> {
        let fonts = vec![];
        let collection_cache = HashMap::new();
        let mut collection = FontCollection::new();
        collection.set_default_font_manager(FontMgr::new(), None);
        Mutex::new(FontLibrary {
            collection,
            collection_cache,
            fonts,
        })
    }

    fn register_fonts(&mut self) {
        let mut assets = TypefaceFontProvider::new();
        for (font, alias) in &self.fonts {
            assets.register_typeface(font.clone(), alias.as_deref());
        }

        let mut collection = FontCollection::new();
        collection.set_default_font_manager(FontMgr::new(), None);
        collection.set_asset_font_manager(Some(assets.into()));
        self.collection = collection;
        self.collection_cache.drain();
    }

    fn add_typeface(&mut self, font: Typeface, alias: Option<String>) {
        if let Some(idx) = self.fonts.iter().position(|(old_font, old_alias)|
            match alias.is_some() {
                true => old_alias.as_deref() == alias.as_deref(),
                false => old_font.family_name() == font.family_name()
            } && old_font.font_style() == font.font_style()
        ) {
            self.fonts.remove(idx);
        }

        self.fonts.push((font, alias));
        self.register_fonts();
    }

    fn remove_typeface(&mut self, font: &Typeface, alias: Option<&str>) {
        if let Some(idx) = self.fonts.iter().position(|(old_font, old_alias)|
            match alias.is_some() {
                true => old_alias.as_deref() == alias,
                false => old_font.family_name() == font.family_name()
            } && old_font.font_style() == font.font_style()
        ) {
            self.fonts.remove(idx);
        }

        self.register_fonts();
    }

    pub fn collect_fonts(&mut self, style: &TextStyle) -> FontCollection {
        let families = style.font_families();
        let families: Vec<&str> = families.iter().collect();
        let matches = self
            .collection
            .find_typefaces(&families, style.font_style());

        // if the matched typeface is a variable font, create an instance that matches
        // the current weight settings and return early with a new FontCollection that
        // contains just that single font instance
        if let Some(font) = matches.first() {
            if let Some(params) = font.variation_design_parameters() {
                // memoize the generation of single-weight FontCollections for variable fonts
                let key = CollectionKey::new(style);
                if let Some(collection) = self.collection_cache.get(&key) {
                    return collection.clone();
                }

                // reconnect to the user-specified family name (if provided)
                let alias = self.fonts.iter().find_map(|(face, alias)| {
                    if Typeface::equal(font, face) {
                        alias.clone()
                    } else {
                        None
                    }
                });

                for param in params {
                    let chars = vec![param.tag.a(), param.tag.b(), param.tag.c(), param.tag.d()];
                    let tag = String::from_utf8(chars).unwrap();
                    if tag == "wght" {
                        // NB: currently setting the value to *one less* than what was requested
                        //     to work around weird Skia behavior that returns something nonlinearly
                        //     weighted in many cases (but not for ±1 of that value). This makes it so
                        //     that n × 100 values will render correctly (and the bug will manifest at
                        //     n × 100 + 1 instead)
                        let weight = *style.font_style().weight() - 1;
                        let value = (weight as f32).max(param.min).min(param.max);
                        let coords = [Coordinate {
                            axis: param.tag,
                            value,
                        }];
                        let v_pos = VariationPosition {
                            coordinates: &coords,
                        };
                        let args = FontArguments::new().set_variation_design_position(v_pos);
                        let face = font.clone_with_arguments(&args).unwrap();

                        let mut dynamic = TypefaceFontProvider::new();
                        dynamic.register_typeface(face, alias.as_deref());

                        let mut collection = FontCollection::new();
                        collection.set_default_font_manager(FontMgr::new(), None);
                        collection.set_asset_font_manager(Some(dynamic.into()));
                        self.collection_cache.insert(key, collection.clone());
                        return collection;
                    }
                }
            }
        }

        // if the matched font wasn't variable, then just return the standard collection
        self.collection.clone()
    }

    pub fn add_family(alias: Option<&str>, filenames: &[&str]) -> Result<(), String> {
        for filename in filenames.iter() {
            let path = std::path::Path::new(&filename);
            let typeface = match std::fs::read(path) {
                Err(why) => return Err(format!("{}: \"{}\"", why, path.display())),
                Ok(bytes) => FontMgr::new().new_from_data(bytes.as_slice(), None),
            };

            match typeface {
                Some(font) => {
                    // register the typeface
                    let mut library = FONT_LIBRARY.lock();
                    let alias = alias.map(|v| v.to_owned());
                    library.add_typeface(font, alias);
                }
                None => return Err(format!("Could not decode font data in {}", path.display())),
            }
        }

        Ok(())
    }

    pub fn add_family_bytes(alias: Option<&str>, data: &[&[u8]]) -> Result<(), String> {
        let mgr = FontMgr::new();
        for bytes in data.iter() {
            match mgr.new_from_data(bytes, None) {
                Some(font) => {
                    // register the typeface
                    let mut library = FONT_LIBRARY.lock();
                    let alias = alias.map(|v| v.to_owned());
                    library.add_typeface(font, alias);
                }
                None => return Err("Could not decode font data".to_string()),
            }
        }

        Ok(())
    }

    pub fn reset() {
        let mut library = FONT_LIBRARY.lock();
        library.fonts.clear();

        let mut collection = FontCollection::new();
        collection.set_default_font_manager(FontMgr::new(), None);
        library.collection = collection;
        library.collection_cache.drain();
    }
}

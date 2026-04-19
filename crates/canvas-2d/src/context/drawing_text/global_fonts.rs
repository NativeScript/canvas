use std::collections::HashMap;

use parking_lot::Mutex;
use skia_safe::font_arguments::variation_position::Coordinate;
use skia_safe::font_arguments::VariationPosition;
use skia_safe::font_style::Slant;
use skia_safe::textlayout::{FontCollection, TextStyle, TypefaceFontProvider};
use skia_safe::{FontArguments, FontMgr, Typeface};
use std::sync::LazyLock;
use ustr::Ustr;

#[derive(PartialEq, Eq, Hash)]
struct CollectionKey {
    families: Vec<Ustr>,
    weight: i32,
    slant: Slant,
}

impl CollectionKey {
    pub fn new(style: &TextStyle) -> Self {
        let families = style.font_families();
        let families: Vec<Ustr> = families.iter().map(Ustr::from).collect();
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

pub static FONT_LIBRARY: LazyLock<Mutex<FontLibrary>> = LazyLock::new(FontLibrary::shared);

pub struct FontLibrary {
    pub fonts: Vec<(Typeface, Option<String>)>,
    pub collection: FontCollection,
    collection_cache: HashMap<CollectionKey, FontCollection>,
    font_mgr: FontMgr,
}

unsafe impl Send for FontLibrary {}

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
        let font_mgr = FontMgr::new();
        let mut collection = FontCollection::new();
        collection.set_default_font_manager(font_mgr.clone(), None);
        Mutex::new(FontLibrary {
            collection,
            collection_cache: HashMap::new(),
            fonts: vec![],
            font_mgr,
        })
    }

    fn register_fonts(&mut self) {
        let mut assets = TypefaceFontProvider::new();
        for (font, alias) in &self.fonts {
            assets.register_typeface(font.clone(), alias.as_deref());
        }

        let mut collection = FontCollection::new();
        collection.set_default_font_manager(self.font_mgr.clone(), None);
        collection.set_asset_font_manager(Some(assets.into()));
        self.collection = collection;
        self.collection_cache.clear();
    }

    /// Inserts or replaces a typeface **without** rebuilding the collection.
    /// Callers must call `register_fonts()` when all additions are done.
    fn add_typeface_no_rebuild(&mut self, font: Typeface, alias: Option<String>) {
        if let Some(idx) = self.fonts.iter().position(|(old_font, old_alias)|
            match alias.is_some() {
                true => old_alias.as_deref() == alias.as_deref(),
                false => old_font.family_name() == font.family_name()
            } && old_font.font_style() == font.font_style()
        ) {
            self.fonts[idx] = (font, alias);
        } else {
            self.fonts.push((font, alias));
        }
    }

    fn add_typeface(&mut self, font: Typeface, alias: Option<String>) {
        self.add_typeface_no_rebuild(font, alias);
        self.register_fonts();
    }

    fn remove_typeface(&mut self, font: &Typeface, alias: Option<&str>) {
        self.fonts.retain(|(old_font, old_alias)| {
            !(match alias.is_some() {
                true => old_alias.as_deref() == alias,
                false => old_font.family_name() == font.family_name()
            } && old_font.font_style() == font.font_style())
        });
        self.register_fonts();
    }

    pub fn collect_fonts(&mut self, style: &TextStyle) -> FontCollection {
        let families = style.font_families();
        let families: Vec<&str> = families.iter().collect();
        let matches = self
            .collection
            .find_typefaces(&families, style.font_style());

       
        if let Some(font) = matches.first() {
            if let Some(params) = font.variation_design_parameters() {
                let key = CollectionKey::new(style);
                if let Some(collection) = self.collection_cache.get(&key) {
                    return collection.clone();
                }

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
                        collection.set_default_font_manager(self.font_mgr.clone(), None);
                        collection.set_asset_font_manager(Some(dynamic.into()));
                        self.collection_cache.insert(key, collection.clone());
                        return collection;
                    }
                }
            }
        }

        self.collection.clone()
    }

    pub fn add_family(alias: Option<&str>, filenames: &[&str]) -> Result<(), String> {
        let decode_mgr = FontMgr::new();
        let mut typefaces = Vec::with_capacity(filenames.len());
        for filename in filenames.iter() {
            let path = std::path::Path::new(filename);
            let bytes = match std::fs::read(path) {
                Err(why) => return Err(format!("{}: \"{}\"", why, path.display())),
                Ok(b) => b,
            };
            match decode_mgr.new_from_data(bytes.as_slice(), None) {
                Some(font) => typefaces.push(font),
                None => return Err(format!("Could not decode font data in {}", path.display())),
            }
        }
        let mut library = FONT_LIBRARY.lock();
        let alias_owned = alias.map(|v| v.to_owned());
        for font in typefaces {
            library.add_typeface_no_rebuild(font, alias_owned.clone());
        }
        library.register_fonts();

        Ok(())
    }

    pub fn add_family_bytes(alias: Option<&str>, data: &[&[u8]]) -> Result<(), String> {
        let decode_mgr = FontMgr::new();
        let mut typefaces = Vec::with_capacity(data.len());
        for bytes in data.iter() {
            match decode_mgr.new_from_data(bytes, None) {
                Some(font) => typefaces.push(font),
                None => return Err("Could not decode font data".to_string()),
            }
        }

        let mut library = FONT_LIBRARY.lock();
        let alias_owned = alias.map(|v| v.to_owned());
        for font in typefaces {
            library.add_typeface_no_rebuild(font, alias_owned.clone());
        }
        library.register_fonts();

        Ok(())
    }

    pub fn reset() {
        let mut library = FONT_LIBRARY.lock();
        library.fonts.clear();

        let mut collection = FontCollection::new();
        collection.set_default_font_manager(library.font_mgr.clone(), None);
        library.collection = collection;
        library.collection_cache.clear();
    }
}

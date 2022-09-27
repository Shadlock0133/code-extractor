use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use prettyplease::unparse;
use syn::{
    File, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemMacro,
    ItemStatic, ItemStruct, ItemTrait, ItemType, ItemUnion,
};

#[derive(Subcommand)]
enum ExtractItem {
    #[clap(name = "list")]
    ListItems,
    #[clap(alias = "f")]
    Function {
        name: String,
    },
    #[clap(alias = "s")]
    Struct {
        name: String,
    },
    #[clap(alias = "e")]
    Enum {
        name: String,
    },
    #[clap(alias = "t")]
    Trait {
        name: String,
    },
    #[clap(alias = "c")]
    Const {
        name: String,
    },
    ExternCrate {
        name: String,
    },
    Static {
        name: String,
    },
    Type {
        name: String,
    },
    Union {
        name: String,
    },
    /// Note: output might be mangled
    Macro {
        name: String,
    },
}

#[derive(Parser)]
struct Opt {
    filename: PathBuf,
    #[clap(subcommand)]
    item: ExtractItem,
}

fn main() {
    let opt = Opt::parse();
    let file_content = fs::read_to_string(opt.filename).unwrap();
    let file = syn::parse_file(&file_content).unwrap();
    match opt.item {
        ExtractItem::ListItems => {
            println!("Listing items:");
            for item in file.items {
                let info = match item {
                    Item::Fn(i) => Some(("fn", i.sig.ident)),
                    Item::Struct(i) => Some(("struct", i.ident)),
                    Item::Enum(i) => Some(("enum", i.ident)),
                    Item::Trait(i) => Some(("trait", i.ident)),
                    Item::Const(i) => Some(("const", i.ident)),
                    Item::ExternCrate(i) => Some(("extern crate", i.ident)),
                    Item::Static(i) => Some(("static", i.ident)),
                    Item::Type(i) => Some(("type", i.ident)),
                    Item::Union(i) => Some(("union", i.ident)),
                    Item::Macro(ItemMacro { ident: Some(i), .. }) => {
                        Some(("macro", i))
                    }
                    _ => None,
                };
                if let Some((kind, name)) = info {
                    println!("{:>12} {}", kind.green().bold(), name.purple());
                }
            }
        }
        ExtractItem::Function { name } => {
            print!("{}", extract::<ItemFn>(&file, &name))
        }
        ExtractItem::Struct { name } => {
            print!("{}", extract::<ItemStruct>(&file, &name))
        }
        ExtractItem::Enum { name } => {
            print!("{}", extract::<ItemEnum>(&file, &name))
        }
        ExtractItem::Trait { name } => {
            print!("{}", extract::<ItemTrait>(&file, &name))
        }
        ExtractItem::Const { name } => {
            print!("{}", extract::<ItemConst>(&file, &name))
        }
        ExtractItem::ExternCrate { name } => {
            print!("{}", extract::<ItemExternCrate>(&file, &name))
        }
        ExtractItem::Static { name } => {
            print!("{}", extract::<ItemStatic>(&file, &name))
        }
        ExtractItem::Type { name } => {
            print!("{}", extract::<ItemType>(&file, &name))
        }
        ExtractItem::Union { name } => {
            print!("{}", extract::<ItemUnion>(&file, &name))
        }
        ExtractItem::Macro { name } => {
            print!("{}", extract::<ItemMacro>(&file, &name))
        }
    }
}

fn extract<T: Find + Unparse + Clone>(file: &File, name: &str) -> String {
    T::find(file, name).unwrap().clone().unparse()
}

trait Find {
    fn find<'a>(file: &'a File, name: &str) -> Option<&'a Self> {
        for item in &file.items {
            if let Some(e) = Self::find_item(item, name) {
                return Some(e);
            }
        }
        None
    }
    fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self>;
}

trait Unparse: Sized {
    fn as_item(self) -> Item;
    fn unparse(self) -> String {
        unparse(
            &(File {
                shebang: None,
                attrs: vec![],
                items: vec![self.as_item()],
            }),
        )
    }
}

macro_rules! impl_traits {
    ($t:ty : Item:: $var:ident) => {
        impl Find for $t {
            fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self> {
                if let Item::$var(i) = item {
                    if i.ident == name {
                        return Some(i);
                    }
                }
                None
            }
        }

        impl Unparse for $t {
            fn as_item(self) -> Item {
                Item::$var(self)
            }
        }
    };
}

impl Find for ItemFn {
    fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self> {
        if let Item::Fn(f) = item {
            if f.sig.ident == name {
                return Some(f);
            }
        }
        None
    }
}

impl Unparse for ItemFn {
    fn as_item(self) -> Item {
        Item::Fn(self)
    }
}

impl Find for ItemMacro {
    fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self> {
        if let Item::Macro(f) = item {
            if f.ident.as_ref()? == name {
                return Some(f);
            }
        }
        None
    }
}

impl Unparse for ItemMacro {
    fn as_item(self) -> Item {
        Item::Macro(self)
    }
}

impl_traits!(ItemStruct: Item::Struct);
impl_traits!(ItemEnum: Item::Enum);
impl_traits!(ItemTrait: Item::Trait);
impl_traits!(ItemConst: Item::Const);
impl_traits!(ItemExternCrate: Item::ExternCrate);
impl_traits!(ItemStatic: Item::Static);
impl_traits!(ItemType: Item::Type);
impl_traits!(ItemUnion: Item::Union);

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use prettyplease::unparse;
use syn::{File, Item, ItemEnum, ItemFn, ItemStruct};

#[derive(Subcommand)]
enum ExtractItem {
    #[clap(name = "list")]
    ListItems,
    #[clap(alias = "f")]
    Function { name: String },
    #[clap(alias = "s")]
    Struct { name: String },
    #[clap(alias = "e")]
    Enum { name: String },
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
                    Item::Fn(f) => Some(("fn", f.sig.ident)),
                    Item::Struct(s) => Some(("struct", s.ident)),
                    Item::Enum(e) => Some(("enum", e.ident)),
                    Item::Trait(t) => Some(("trait", t.ident)),
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

impl Find for ItemStruct {
    fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self> {
        if let Item::Struct(s) = item {
            if s.ident == name {
                return Some(s);
            }
        }
        None
    }
}

impl Find for ItemEnum {
    fn find_item<'a>(item: &'a Item, name: &str) -> Option<&'a Self> {
        if let Item::Enum(s) = item {
            if s.ident == name {
                return Some(s);
            }
        }
        None
    }
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

impl Unparse for ItemFn {
    fn as_item(self) -> Item {
        Item::Fn(self)
    }
}

impl Unparse for ItemStruct {
    fn as_item(self) -> Item {
        Item::Struct(self)
    }
}

impl Unparse for ItemEnum {
    fn as_item(self) -> Item {
        Item::Enum(self)
    }
}

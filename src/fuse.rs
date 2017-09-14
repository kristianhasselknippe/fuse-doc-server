use std::path::Path;
use std::fs::File;
use serde_json;
use std::io::Read;


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Item {
    entity: Entity,
    tableOfContents: TableOfContents,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TableOfContents {
    uxProperties: Option<Vec<PropertyCollection>>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Entity {
    comment: Option<Comment>,
    implementedInterfaces: Option<Vec<Interface>>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Interface {
    comment: Option<Comment>,
    id: Id,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Comment {
    pub brief: String,
    pub full: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Uri {
    idUri: String,
    href: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Id {
    id: String,
    parentId: String,
    #[serde(rename = "type")]_type: String,

}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Titles {
    indexTitle: String,
    fullyQualifiedIndexTitle: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct EntityRef {
    id: Id,
    uri: Uri,
    titles: Titles,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PropertyCollection {
    declaredIn: EntityRef,
    items: Vec<UXProperty>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Returns {
    pub href: String,
    pub title: String,
    pub fullyQualifiedTitle: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UXProperty {
    pub comment: Option<Comment>,
    id: Option<Id>,
    pub returns: Returns,
    uri: Uri,
    titles: Titles,
}


pub fn parse_doc_file(path: &Path) -> Item {
    let mut content = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut content);

    serde_json::from_str(&content).unwrap()
}

impl Item {
    pub fn get_property(&self, name: &str) -> Option<UXProperty> {
        if let Some(ref uxProperties) = self.tableOfContents.uxProperties {
            for col in uxProperties {
                for i in &col.items {
                    if i.uri.href.split("/").last().unwrap() == name {
                        return Some(i.clone())
                    }
                }
            }
        }
        None
    }

    pub fn print_properties(&self) {
        if let Some(ref uxProperties) = self.tableOfContents.uxProperties {
            for col in uxProperties {
                println!("Inherited from: {:?}", col.declaredIn.titles.indexTitle);
                for i in &col.items {
                    println!("    Property: {:?}", i.titles.indexTitle);
                }
            }
        }
    }
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Element, Event, Node};

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES_LEN_F64: f64 = ADJECTIVES_LEN as f64;
const ADJECTIVES: [&str; ADJECTIVES_LEN] = [
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

const COLOURS_LEN: usize = 11;
const COLOURS_LEN_F64: f64 = COLOURS_LEN as f64;
const COLOURS: [&str; COLOURS_LEN] = [
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS_LEN_F64: f64 = NOUNS_LEN as f64;
const NOUNS: [&str; NOUNS_LEN] = [
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: f64) -> usize {
    ((Math::random() * 1000.0) % max) as usize
}

struct Row {
    id: usize,
    label: String,
    el: Element,
    label_node: Node,
}

const ROW_TEMPLATE: &str = "<td class='col-md-1'></td><td class='col-md-4'><a class='lbl'></a></td><td class='col-md-1'><a class='remove'><span class='remove glyphicon glyphicon-remove' aria-hidden='true'></span></a></td><td class='col-md-6'></td>";

struct Main {
    data: Vec<Row>,
    row_template: Node,
    tbody: Node,
    last_id: usize,
    selected: Option<Element>,
}

fn get_parent_id(el: Element) -> Option<usize> {
    let mut current = Some(el);
    while let Some(e) = current {
        if e.tag_name() == "TR" {
            return match e.get_attribute("data-id") {
                Some(id) => Some(id.parse::<usize>().unwrap_throw()),
                None => None,
            };
        }
        current = e.parent_element();
    }
    None
}

impl Main {
    fn run(&mut self) -> Result<(), JsValue> {
        self.clear();
        self.append_rows(1000)
    }

    fn add(&mut self) -> Result<(), JsValue> {
        self.append_rows(1000)
    }

    fn update(&mut self) {
        let mut i = 0;
        let l = self.data.len();
        while i < l {
            let row = &mut self.data[i];
            row.label.push_str(" !!!");
            row.label_node.set_text_content(Some(row.label.as_str()));
            i += 10;
        }
    }

    fn unselect(&mut self) {
        if let Some(el) = self.selected.take() {
            el.set_class_name("");
        }
    }

    fn select(&mut self, id: usize) {
        self.unselect();
        for row in &self.data {
            if row.id == id {
                row.el.set_class_name("danger");
                self.selected = Some(row.el.clone());
                return;
            }
        }
    }

    fn delete(&mut self, id: usize) {
        let row = match self.data.iter().position(|row| row.id == id) {
            Some(i) => self.data.remove(i),
            None => return,
        };
        row.el.remove();
    }

    fn clear(&mut self) {
        self.data = Vec::new();
        self.tbody.set_text_content(None);
        self.unselect();
    }

    fn run_lots(&mut self) -> Result<(), JsValue> {
        self.clear();
        self.append_rows(10000)
    }

    fn swap_rows(&mut self) -> Result<(), JsValue> {
        if self.data.len() <= 998 {
            return Ok(());
        }
        let row1 = &self.data[1];
        let row998 = &self.data[998];
        let a = &row1.el;
        let b = a.next_sibling().unwrap_throw();
        let c = &row998.el;
        let d = c.next_sibling().unwrap_throw();
        self.tbody.insert_before(&c, Some(&b))?;
        self.tbody.insert_before(&a, Some(&d))?;
        self.data.swap(1, 998);
        Ok(())
    }

    fn append_rows(&mut self, count: usize) -> Result<(), JsValue> {
        self.data.reserve(count);
        for i in 0..count {
            let id = self.last_id + i + 1;

            let adjective = ADJECTIVES[random(ADJECTIVES_LEN_F64)];
            let colour = COLOURS[random(COLOURS_LEN_F64)];
            let noun = NOUNS[random(NOUNS_LEN_F64)];
            let capacity = adjective.len() + colour.len() + noun.len() + 2;
            let mut label = String::with_capacity(capacity);
            label.push_str(adjective);
            label.push(' ');
            label.push_str(colour);
            label.push(' ');
            label.push_str(noun);

            let node = self.row_template.clone_node_with_deep(true)?;
            let id_node = node.first_child().unwrap_throw();
            let label_node = id_node.next_sibling().unwrap_throw().first_child().unwrap_throw();
            let id_string = id.to_string();
            let id_str = id_string.as_str();
            id_node.set_text_content(Some(id_str));
            label_node.set_text_content(Some(label.as_str()));

            let el = JsCast::unchecked_into::<Element>(node);
            el.set_attribute("data-id", id_str)?;
            let row = Row {
                id,
                label,
                el,
                label_node,
            };

            self.tbody.append_child(&row.el)?;
            self.data.push(row);
        }
        self.last_id += count;
        Ok(())
    }
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap_throw().document().unwrap_throw()
}

fn on<F>(id: &str, name: &str, callback: F) where F: FnMut(web_sys::Event) + 'static {
    let target = document().get_element_by_id(id).unwrap_throw();

    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(web_sys::Event)>);

    target.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref()).unwrap_throw();

    closure.forget();
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let row_template = document().create_element("tr")?;
    row_template.set_inner_html(ROW_TEMPLATE);

    let tbody = document().get_element_by_id("tbody").unwrap_throw();

    let main = Rc::new(RefCell::new(Main {
        data: Vec::new(),
        row_template: row_template.into(),
        tbody: tbody.into(),
        last_id: 0,
        selected: None,
    }));

    on("main", "click", move |e: Event| {
        let target = e.target().unwrap_throw();
        let el: &Element = target.unchecked_ref();

        let mut main = main.borrow_mut();

        match el.id().as_str() {
            "add" => {
                e.prevent_default();
                main.add().unwrap_throw();
            }
            "run" => {
                e.prevent_default();
                main.run().unwrap_throw();
            }
            "update" => {
                e.prevent_default();
                main.update();
            }
            "runlots" => {
                e.prevent_default();
                main.run_lots().unwrap_throw();
            }
            "clear" => {
                e.prevent_default();
                main.clear();
            }
            "swaprows" => {
                e.prevent_default();
                main.swap_rows().unwrap_throw();
            }
            _ => {
                let class_list = el.class_list();

                if class_list.contains("remove") {
                    e.prevent_default();
                    let parent_id = get_parent_id(el.clone()).unwrap_throw();
                    main.delete(parent_id);

                } else if class_list.contains("lbl") {
                    e.prevent_default();
                    let parent_id = get_parent_id(el.clone()).unwrap_throw();
                    main.select(parent_id);
                }
            }
        }
    });

    Ok(())
}

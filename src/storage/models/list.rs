use glib::Sender;
use relm4::{MicroModel, MicroWidgets};
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::lists;
use gtk4 as gtk;
use gtk4::prelude::WidgetExt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name="lists"]
pub struct List {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
}

impl List {
    pub fn new(name: String) -> Self {
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: name,
            is_owner: true,
        }
    }
}

impl MicroModel for List {
    type Msg = ();
    type Widgets = ListWidgets;
    type Data = ();

    fn update(&mut self, _msg: Self::Msg, _data: &Self::Data, _sender: Sender<Self::Msg>) {
        todo!()
    }
}

#[relm4::micro_widget(pub)]
#[derive(Debug)]
impl MicroWidgets<List> for ListWidgets {
    view! {
        label = &gtk::Label {
            set_halign: gtk::Align::Start,
            set_text: model.display_name.as_str(),
            set_margin_top: 10,
            set_margin_bottom: 10,
            set_margin_start: 15,
            set_margin_end: 15,
        }
    }
}
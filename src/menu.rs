use crate::Global;
use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

pub fn cycle(globals: &mut Global) {
    /* widgets::Popup::new(hash!(), vec2(400., 200.)).ui(&mut *root_ui(), |ui|{
        ui.label(Vec2::new(10.,10.),&format!("Test"));
    });*/
    if globals.DEVELOPER_MODE {
        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
            .label("Editor")
            .ui(&mut *root_ui(), |ui| {
                for i in 0..30 {
                    Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
                        ui.label(Vec2::new(10., 10.), &format!("Item N {}", i));
                        ui.label(Vec2::new(260., 40.), "10/10");
                        ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
                        if ui.button(Vec2::new(260., 55.), "Go") {
                            println!("got em");
                        }
                    });
                }
            });
    }
}

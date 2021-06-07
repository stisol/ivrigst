use bevy::math::vec3;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::material::MyMaterial;

pub fn ui(egui_context: ResMut<EguiContext>, mut materials: ResMut<Assets<MyMaterial>>) {
    let (handle, _) = materials.iter().next().expect("No material found");
    let material = materials.get_mut(handle).expect("No material extracted");

    egui::Window::new("Settings").show(egui_context.ctx(), |ui| {
        // Colour widget.
        ui.horizontal(|ui| {
            let mut color = material.get_color().into();
            ui.label("Model base colour");
            ui.color_edit_button_rgb(&mut color);
            material.set_color(vec3(color[0], color[1], color[2]));
        });

        // Distance shading parameters widget.
        ui.vertical(|ui| {
            use crate::material::DistanceShadingChannel as DSC;
            let mut ds_dist = material.get_distance_shading();
            let mut ds_power = material.get_distance_shading_power();
            let mut ds_channel = material.get_distance_shading_channel();

            egui::ComboBox::from_label("Distance shading channel")
                .selected_text(format!("{:?}", ds_channel)) // Todo: fix
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut ds_channel, DSC::Hue, format!("{:?}", DSC::Hue));
                    ui.selectable_value(
                        &mut ds_channel,
                        DSC::Saturation,
                        format!("{:?}", DSC::Saturation),
                    );
                    ui.selectable_value(&mut ds_channel, DSC::Value, format!("{:?}", DSC::Value));
                    ui.selectable_value(&mut ds_channel, DSC::None, format!("{:?}", DSC::None));
                });

            ui.horizontal(|ui| {
                ui.label("Distance shading min");
                ui.add(egui::Slider::new(&mut ds_dist.x, 0.0..=500.0));
            });
            ui.horizontal(|ui| {
                ui.label("Distance shading max");
                ui.add(egui::Slider::new(&mut ds_dist.y, 0.0..=500.0));
            });
            ui.horizontal(|ui| {
                ui.label("Distance shading power");
                ui.add(egui::Slider::new(&mut ds_power, 0.0..=1.0));
            });
            material.set_distance_shading(ds_dist);
            material.set_distance_shading_power(ds_power);
            if ds_channel != material.get_distance_shading_channel() {
                material.set_distance_shading_channel(ds_channel);
            }
        });
    });
}

pub fn camera(
    camera_transforms: Query<&Transform, With<bevy::render::camera::Camera>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    let camera_position = camera_transforms
        .iter()
        .next()
        .expect("No camera found")
        .translation;

    let (handle, _) = materials.iter().next().expect("No material found");
    let material = materials.get_mut(handle).expect("No material extracted");

    material.set_camera_position(camera_position);
}

use std::sync::Arc;

mod generator;
pub mod versions;

pub const SUBHEADING_STYLE: &'static str = "subheading";
const GROUP_SPACING: f32 = 7.0;

#[derive(Default, PartialEq, Eq)]
pub enum ProjectType {
    #[default]
    Multiplatform,
    Forge,
}

#[derive(Default, PartialEq, Eq)]
pub enum MappingSet {
    #[default]
    Mojang,
    Yarn,
}

impl MappingSet {
    fn description(&self) -> &'static str {
        match self {
            Self::Mojang => "The official obfuscation maps published by Mojang.",
            Self::Yarn => "A libre mapping set maintained by FabricMC.",
        }
    }
}

#[derive(Default)]
pub struct Subprojects {
    fabric: bool,
    forge: bool,
    neoforge: bool,
    quilt: bool,
    fabric_likes: bool,
}

pub struct Dependencies {
    architectury_api: bool,
}

impl Default for Dependencies {
    fn default() -> Self {
        Self {
            architectury_api: true,
        }
    }
}

pub struct GeneratorApp {
    mod_name: String,
    mod_id: String,
    package_name: String,
    game_version: &'static versions::MinecraftVersion<'static>,
    project_type: ProjectType,
    subprojects: Subprojects,
    mapping_set: MappingSet,
    dependencies: Dependencies,
}

impl GeneratorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Name(Arc::from(SUBHEADING_STYLE)),
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );
        ctx.set_style(style);

        Self {
            mod_name: "Example Mod".to_owned(),
            mod_id: String::new(),
            package_name: "com.example".to_owned(),
            game_version: &versions::ALL_MINECRAFT_VERSIONS[0],
            project_type: Default::default(),
            subprojects: Default::default(),
            mapping_set: Default::default(),
            dependencies: Default::default(),
        }
    }
}

impl eframe::App for GeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut has_errors = false;

        egui::Window::new("Mod properties").show(ctx, |ui| {
            ui.heading("Mod name");
            ui.label("The human-readable name of your mod.");
            ui.text_edit_singleline(&mut self.mod_name);

            ui.add_space(GROUP_SPACING);
            ui.heading("Mod ID (optional)");
            ui.label("A unique ID for your mod.");
            let no_id = self.mod_id.is_empty();
            let mut text_edit = egui::TextEdit::singleline(&mut self.mod_id);

            if no_id {
                if let Some(mod_id) = crate::mod_ids::to_mod_id(&self.mod_name) {
                    text_edit = text_edit.hint_text(mod_id);
                } else {
                    ui.label(
                        "Could not generate mod ID from mod name, please enter it manually :)",
                    );
                    has_errors = true;
                }
            }

            ui.add(text_edit);
            ui.add_space(GROUP_SPACING);

            ui.heading("Package name");
            ui.label("A unique package name for your mod.");
            ui.text_edit_singleline(&mut self.package_name);
            ui.add_space(GROUP_SPACING);

            ui.heading("Minecraft version");
            egui::ComboBox::from_id_source(0)
                .selected_text(self.game_version.version)
                .show_ui(ui, |ui| {
                    for game_version in versions::ALL_MINECRAFT_VERSIONS {
                        ui.selectable_value(
                            &mut self.game_version,
                            game_version,
                            game_version.version,
                        );
                    }
                });

            ui.add_space(GROUP_SPACING);
            ui.heading("Mappings");
            ui.label("The set of names used for Minecraft code.");
            ui.radio_value(
                &mut self.mapping_set,
                MappingSet::Mojang,
                "Official Mojang mappings",
            );
            ui.radio_value(&mut self.mapping_set, MappingSet::Yarn, "Yarn");
            ui.add_space(2.0);
            ui.label(self.mapping_set.description());
        });

        egui::Window::new("Mod loaders").show(ctx, |ui| {
            ui.heading("Project type");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.project_type,
                    ProjectType::Multiplatform,
                    "Multiplatform",
                );
                ui.selectable_value(&mut self.project_type, ProjectType::Forge, "Forge");
            });
            ui.add_space(GROUP_SPACING);

            if self.project_type == ProjectType::Multiplatform {
                ui.heading("Subprojects");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.subprojects.fabric, "Fabric");
                        ui.checkbox(&mut self.subprojects.forge, "Forge");
                        ui.checkbox(&mut self.subprojects.neoforge, "NeoForge");
                        ui.checkbox(&mut self.subprojects.quilt, "Quilt");
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(
                            egui::WidgetText::from("Additional subprojects")
                                .text_style(egui::TextStyle::Name(Arc::from(SUBHEADING_STYLE))),
                        );
                        ui.checkbox(&mut self.subprojects.fabric_likes, "Fabric-like");
                        ui.label("Shared code between Fabric and Quilt.");
                    })
                });
                ui.add_space(GROUP_SPACING);

                ui.heading("Dependencies");
                ui.checkbox(&mut self.dependencies.architectury_api, "Architectury API");
            }
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            let button_label = egui::RichText::new("Generate!").size(20.0);
            if !has_errors && ui.button(button_label).clicked() {
                generator::generate(self).expect("bruh");
            }
        });
    }
}

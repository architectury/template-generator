// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "forge_only", false, "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "forge_only", false, "gradle.properties");
super::file_data!(SETTINGS_GRADLE settings_gradle, "forge_only", false, "settings.gradle");

// Code
super::file_data!(PACK_MCMETA pack_mcmeta, "forge_only", false, "src/main/resources/pack.mcmeta");
super::file_data!(MODS_TOML mods_toml, "forge_only", false, "src/main/resources/META-INF/mods.toml");
super::file_data!(MIXINS mixins, "forge_only", false, "src/main/resources/MOD_ID.mixins.json");
super::file_data!(MOD_CLASS mod_class, "forge_only", false, "src/main/java/PACKAGE_DIR/ExampleMod.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    settings_gradle
    pack_mcmeta
    mods_toml
    mixins
    mod_class
);

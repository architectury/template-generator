// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "forge", "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "forge", "gradle.properties");

// Code
super::file_data!(PACK_MCMETA pack_mcmeta, "forge", "src/main/resources/pack.mcmeta");
super::file_data!(MODS_TOML mods_toml, "forge", "src/main/resources/META-INF/mods.toml");
super::file_data!(MOD_CLASS mod_class, "forge", "src/main/java/PACKAGE_DIR/forge/ExampleModForge.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    pack_mcmeta
    mods_toml
    mod_class
);

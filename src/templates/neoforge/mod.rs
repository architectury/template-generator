// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "neoforge", true, "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "neoforge", true, "gradle.properties");

// Code
super::file_data!(MODS_TOML mods_toml, "neoforge", true, "src/main/resources/META-INF/mods.toml");
super::file_data!(NEOFORGE_MODS_TOML neoforge_mods_toml, "neoforge", true, "src/main/resources/META-INF/neoforge.mods.toml");
super::file_data!(MOD_CLASS mod_class, "neoforge", true, "src/main/java/PACKAGE_DIR/neoforge/ExampleModNeoForge.java");

super::file_list!(pub main_files,
    build_gradle
    gradle_properties
    mod_class
);

super::file_list!(pub mods_toml_files,
    mods_toml
);

super::file_list!(pub neoforge_mods_toml_files,
    neoforge_mods_toml
);

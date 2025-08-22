// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "quilt", true, "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "quilt", true, "gradle.properties");

// Code
super::file_data!(QUILT_MOD_JSON quilt_mod_json, "quilt", true, "src/main/resources/quilt.mod.json");
super::file_data_with_target!(MOD_CLASS mod_class, "quilt", true, "src/main/java/PACKAGE_DIR/quilt/ExampleModQuilt.java", "src/main/java/PACKAGE_DIR/quilt/MAIN_CLASS_NAMEQuilt.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    quilt_mod_json
    mod_class
);

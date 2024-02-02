// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Root files
super::file_data!(BUILD_GRADLE build_gradle, "multiplatform", false, "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "multiplatform", false, "gradle.properties");
super::file_data!(SETTINGS_GRADLE settings_gradle, "multiplatform", false, "settings.gradle");

// Common files
super::file_data!(COMMON_BUILD_GRADLE common_build_gradle, "multiplatform", false, "common/build.gradle");
super::file_data!(COMMON_MIXINS common_mixins, "multiplatform", false, "common/src/main/resources/MOD_ID.mixins.json");
super::file_data!(COMMON_INIT_CLASS common_init_class, "multiplatform", false, "common/src/main/java/PACKAGE_DIR/ExampleMod.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    settings_gradle
    common_build_gradle
    common_mixins
    common_init_class
);

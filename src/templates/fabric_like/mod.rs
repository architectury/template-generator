// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "fabric-like", true, "build.gradle");

// Code
super::file_data!(MOD_CLASS mod_class, "fabric-like", true, "src/main/java/PACKAGE_DIR/fabriclike/ExampleModFabricLike.java");

super::file_list!(pub all_files,
    build_gradle
    mod_class
);

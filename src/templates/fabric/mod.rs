// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "fabric", true, "build.gradle");

// Code
super::file_data!(FABRIC_MOD_JSON fabric_mod_json, "fabric", true, "src/main/resources/fabric.mod.json");
super::file_data_with_target!(MOD_CLASS mod_class, "fabric", true, "src/main/java/PACKAGE_DIR/fabric/ExampleModFabric.java", "src/main/java/PACKAGE_DIR/fabric/MAIN_CLASS_NAMEFabric.java");
super::file_data_with_target!(CLIENT_MOD_CLASS client_mod_class, "fabric", true, "src/main/java/PACKAGE_DIR/fabric/client/ExampleModFabricClient.java", "src/main/java/PACKAGE_DIR/fabric/client/MAIN_CLASS_NAMEFabricClient.java");

super::file_list!(pub all_files,
    build_gradle
    fabric_mod_json
    mod_class
    client_mod_class
);

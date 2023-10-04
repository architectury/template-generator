// Build files
super::file_data!(BUILD_GRADLE build_gradle, "fabric", "build.gradle");

// Code
super::file_data!(FABRIC_MOD_JSON fabric_mod_json, "fabric", "src/main/resources/fabric.mod.json");
super::file_data!(MOD_CLASS mod_class, "fabric", "src/main/java/PACKAGE_DIR/fabric/ExampleModFabric.java");
super::file_data!(CLIENT_MOD_CLASS client_mod_class, "fabric", "src/main/java/PACKAGE_DIR/fabric/client/ExampleModFabricClient.java");

super::file_list!(pub all_files,
    build_gradle
    fabric_mod_json
    mod_class
    client_mod_class
);

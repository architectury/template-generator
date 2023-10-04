// Root files
super::file_data!(BUILD_GRADLE build_gradle, "", "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "", "gradle.properties");
super::file_data!(SETTINGS_GRADLE settings_gradle, "", "settings.gradle");

// Common files
super::file_data!(COMMON_BUILD_GRADLE common_build_gradle, "", "common/build.gradle");
super::file_data!(COMMON_MIXINS common_mixins, "", "common/src/main/resources/MOD_ID.mixins.json");
super::file_data!(COMMON_INIT_CLASS common_init_class, "", "common/src/main/java/PACKAGE_DIR/ExampleMod.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    settings_gradle
    common_build_gradle
    common_mixins
    common_init_class
);

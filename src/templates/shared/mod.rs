super::file_data!(GRADLE_WRAPPER_JAR gradle_wrapper_jar, "shared", false, "gradle/wrapper/gradle-wrapper.jar");
super::file_data!(GRADLE_WRAPPER_PROPERTIES gradle_wrapper_properties, "shared", false, "gradle/wrapper/gradle-wrapper.properties");

super::file_list!(pub shared_files,
    gradle_wrapper_jar
    gradle_wrapper_properties
);

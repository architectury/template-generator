// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

super::file_data!(GRADLE_WRAPPER_JAR gradle_wrapper_jar, "shared", false, "gradle/wrapper/gradle-wrapper.jar");
super::file_data!(GRADLE_WRAPPER_PROPERTIES gradle_wrapper_properties, "shared", false, "gradle/wrapper/gradle-wrapper.properties");

super::file_list!(pub shared_files,
    gradle_wrapper_jar
    gradle_wrapper_properties
);

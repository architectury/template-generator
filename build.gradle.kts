val wasmDir = layout.buildDirectory.dir("wasm")
val outputDir = layout.buildDirectory.dir("web")
val minecraftVersions = file("src/minecraft_versions.json")
val versionIndex = layout.buildDirectory.file("version_index.json")

val compileWasm = tasks.register<Exec>("compileWasm") {
    commandLine("wasm-pack", "build", "--target", "web", "-d", wasmDir.get().asFile.absolutePath)
    inputs.dir("src")
    inputs.dir("version_resolver/src")
    outputs.dir(wasmDir)
}

val generateVersionIndex = tasks.register<Exec>("generateVersionIndex") {
    inputs.dir("version_resolver/src")
    inputs.file(minecraftVersions)
    commandLine("cargo", "run", "-p", "version_resolver", "--", "-v", minecraftVersions.absolutePath, "-o", versionIndex.get().asFile.absolutePath)
    outputs.file(versionIndex)
}

val buildWeb = tasks.register<Copy>("buildWeb") {
    dependsOn(compileWasm, generateVersionIndex)
    from(fileTree(wasmDir)) {
        include("*.wasm", "*.js")
    }
    from(fileTree("res"))
    from(fileTree("src/templates")) {
        into("templates")
        exclude("**/*.rs")
        includeEmptyDirs = false
    }
    from(minecraftVersions)
    from(versionIndex)

    into(outputDir)

    doFirst {
        delete(outputDir)
    }
}

tasks.register<Exec>("runTestServer") {
    outputs.upToDateWhen { false }
    dependsOn(buildWeb)
    commandLine("python", file("test/server.py").absolutePath, "-d", outputDir.get().asFile.absolutePath)
}

tasks.register<Sync>("refreshWebFiles") {
    dependsOn(buildWeb)
    from(fileTree(buildWeb.map { it.destinationDir }))
    into("pages") // must match the path used in the build workflow
    preserve {
        include("CNAME")
    }
}

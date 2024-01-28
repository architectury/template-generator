val wasmDir = layout.buildDirectory.dir("wasm")
val outputDir = layout.buildDirectory.dir("web")

val compileWasm = tasks.register<Exec>("compileWasm") {
    commandLine("wasm-pack", "build", "--target", "web", "-d", wasmDir.get().asFile.absolutePath)
    inputs.dir("src")
    outputs.dir(wasmDir)
}

val buildWeb = tasks.register<Copy>("buildWeb") {
    dependsOn(compileWasm)
    from(fileTree(wasmDir)) {
        include("*.wasm", "*.js")
    }
    from(fileTree("res"))
    from(fileTree("src/templates")) {
        into("templates")
        exclude("**/*.rs")
        includeEmptyDirs = false
    }

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

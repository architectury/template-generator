package %PACKAGE_NAME%.neoforge;

import net.neoforged.fml.common.Mod;

import %PACKAGE_NAME%.ExampleMod;

@Mod(ExampleMod.MOD_ID)
public final class ExampleModNeoForge {
    public ExampleModNeoForge() {
        // Run our common setup.
        ExampleMod.init();
    }
}

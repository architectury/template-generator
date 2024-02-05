package %PACKAGE_NAME%;

import net.minecraftforge.fml.common.Mod;

@Mod(ExampleMod.MOD_ID)
public final class ExampleMod {
    public static final String MOD_ID = "%MOD_ID%";

    public ExampleMod() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like registries and resources) may still be uninitialized.
        // Proceed with mild caution.
    }
}

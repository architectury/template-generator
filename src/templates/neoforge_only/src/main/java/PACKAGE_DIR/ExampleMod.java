package %PACKAGE_NAME%;

import net.neoforged.fml.common.Mod;

@Mod(%MAIN_CLASS_NAME%.MOD_ID)
public final class %MAIN_CLASS_NAME% {
    public static final String MOD_ID = "%MOD_ID%";

    public %MAIN_CLASS_NAME%() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like registries and resources) may still be uninitialized.
        // Proceed with mild caution.
    }
}

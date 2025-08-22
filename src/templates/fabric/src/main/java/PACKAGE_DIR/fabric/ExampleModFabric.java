package %PACKAGE_NAME%.fabric;

import net.fabricmc.api.ModInitializer;

//% if fabric_like
import %PACKAGE_NAME%.fabriclike.%MAIN_CLASS_NAME%FabricLike;
//% else
import %PACKAGE_NAME%.%MAIN_CLASS_NAME%;
//% end

public final class %MAIN_CLASS_NAME%Fabric implements ModInitializer {
    @Override
    public void onInitialize() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like resources) may still be uninitialized.
        // Proceed with mild caution.

//% if fabric_like
        // Run the Fabric-like setup.
        %MAIN_CLASS_NAME%FabricLike.init();
//% else
        // Run our common setup.
        %MAIN_CLASS_NAME%.init();
//% end
    }
}

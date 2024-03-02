package %PACKAGE_NAME%.fabric;

import net.fabricmc.api.ModInitializer;

//% if fabric_like
import %PACKAGE_NAME%.fabriclike.ExampleModFabricLike;
//% else
import %PACKAGE_NAME%.ExampleMod;
//% end

public final class ExampleModFabric implements ModInitializer {
    @Override
    public void onInitialize() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like resources) may still be uninitialized.
        // Proceed with mild caution.

//% if fabric_like
        // Run the Fabric-like setup.
        ExampleModFabricLike.init();
//% else
        // Run our common setup.
        ExampleMod.init();
//% end
    }
}

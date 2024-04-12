package %PACKAGE_NAME%.quilt;

import org.quiltmc.loader.api.ModContainer;
import org.quiltmc.qsl.base.api.entrypoint.ModInitializer;

//% if fabric_like
import %PACKAGE_NAME%.fabriclike.ExampleModFabricLike;
//% else
import %PACKAGE_NAME%.ExampleMod;
//% end

public final class ExampleModQuilt implements ModInitializer {
    @Override
    public void onInitialize(ModContainer mod) {
//% if fabric_like
        // Run the Fabric-like setup.
        ExampleModFabricLike.init();
//% else
        // Run our common setup.
        ExampleMod.init();
//% end
    }
}

package %PACKAGE_NAME%.quilt;

import org.quiltmc.loader.api.ModContainer;
import org.quiltmc.qsl.base.api.entrypoint.ModInitializer;

//% if fabric_like
import %PACKAGE_NAME%.fabriclike.%MAIN_CLASS_NAME%FabricLike;
//% else
import %PACKAGE_NAME%.%MAIN_CLASS_NAME%;
//% end

public final class %MAIN_CLASS_NAME%Quilt implements ModInitializer {
    @Override
    public void onInitialize(ModContainer mod) {
//% if fabric_like
        // Run the Fabric-like setup.
        %MAIN_CLASS_NAME%FabricLike.init();
//% else
        // Run our common setup.
        %MAIN_CLASS_NAME%.init();
//% end
    }
}

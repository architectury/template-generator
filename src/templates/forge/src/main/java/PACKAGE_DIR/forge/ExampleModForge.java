package %PACKAGE_NAME%.forge;

//% if architectury_api
import %ARCHITECTURY_PACKAGE%.platform.forge.EventBuses;
//% end
import net.minecraftforge.fml.common.Mod;
//% if architectury_api
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
//% end

import %PACKAGE_NAME%.%MAIN_CLASS_NAME%;

@Mod(%MAIN_CLASS_NAME%.MOD_ID)
public final class %MAIN_CLASS_NAME%Forge {
    public %MAIN_CLASS_NAME%Forge() {
//% if architectury_api
        // Submit our event bus to let Architectury API register our content on the right time.
        EventBuses.registerModEventBus(%MAIN_CLASS_NAME%.MOD_ID, FMLJavaModLoadingContext.get().getModEventBus());

//% end
        // Run our common setup.
        %MAIN_CLASS_NAME%.init();
    }
}

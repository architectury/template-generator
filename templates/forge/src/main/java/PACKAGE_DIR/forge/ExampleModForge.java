package %PACKAGE_NAME%.forge;

//% if architectury_api
import %ARCHITECTURY_PACKAGE%.platform.forge.EventBuses;
//% end
import net.minecraftforge.fml.common.Mod;
//% if architectury_api
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
//% end

import %PACKAGE_NAME%.ExampleMod;

@Mod(ExampleMod.MOD_ID)
public final class ExampleModForge {
    public ExampleModForge() {
//% if architectury_api
        // Submit our event bus to let Architectury API register our content on the right time.
        EventBuses.registerModEventBus(ExampleMod.MOD_ID, FMLJavaModLoadingContext.get().getModEventBus());

//% end
        // Run our common setup.
        ExampleMod.init();
    }
}

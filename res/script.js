import init, { create_state, list_all_minecraft_versions } from "./templateer.js";
await init();

const state = create_state();

// Set up Minecraft version dropdown with contents
const mcSelect = document.getElementById("minecraft-version-select");

for (const version of list_all_minecraft_versions()) {
    const option = document.createElement("option");
    option.textContent = version;
    mcSelect.appendChild(option);
}

mcSelect.onchange = (event) => {
    state.game_version = event.target.value;
};

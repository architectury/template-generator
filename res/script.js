import init, { create_state, is_valid_mod_id, list_all_minecraft_versions, to_mod_id, validate_mod_id } from "./templateer.js";
await init();

const state = create_state();
console.log(state);

// Set up Minecraft version dropdown with contents
const mcSelect = document.getElementById("minecraft-version-select");

for (const version of list_all_minecraft_versions().reverse()) {
    const option = document.createElement("option");
    option.textContent = version;
    mcSelect.appendChild(option);
}

mcSelect.onchange = (event) => {
    state.game_version = event.target.value;
};

// Hide multiplatform settings when deselected
const projectTypeToggles = document.getElementById("project-type-toggles").getElementsByTagName("input");
const multiplatformInput = document.getElementById("multiplatform-input");
const multiplatformSettings = document.getElementById("multiplatform-settings");

for (const input of projectTypeToggles) {
    input.onchange = () => {
        if (multiplatformInput.checked) {
            multiplatformSettings.classList.remove("hidden");
        } else {
            multiplatformSettings.classList.add("hidden");
        }
    }
};

// Add generated mod id placeholder when not specified manually
const modNameInput = document.getElementById("mod-name-input");
const modIdInput = document.getElementById("mod-id-input");

modNameInput.oninput = (event) => {
    modIdInput.placeholder = to_mod_id(event.target.value) ?? "";
    validateModId();
};

// Validate mod ids
const modIdLabel = document.getElementById("mod-id-label");
modIdInput.oninput = validateModId;

function validateModId() {
    const validation = validate_mod_id(getModId());

    if (validation[0]) {
        modIdLabel.removeAttribute("error");
    } else {
        modIdLabel.setAttribute("error", validation[1]);
    }
}

function isModIdValid() {
    return is_valid_mod_id(getModId());
}

function getModId() {
    let value = modIdInput.value;
    if (value === "") {
        value = modIdInput.placeholder;
    }
    return value;
}

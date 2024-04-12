// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import init, { create_state, generate, is_valid_mod_id, list_all_minecraft_versions, supports_neoforge, to_mod_id, validate_mod_id } from "./templateer.js";
await init();

const state = create_state();

// Set up Minecraft version dropdown with contents
const mcSelect = document.getElementById("minecraft-version-select");
mcSelect.onchange = refreshAvailablePlatforms;

for (const version of list_all_minecraft_versions().reverse()) {
    const option = document.createElement("option");
    option.textContent = version;
    mcSelect.appendChild(option);
}

// Hide multiplatform settings when deselected
const projectTypeToggles = document.getElementById("project-type-toggles").getElementsByTagName("input");
const multiplatformInput = document.getElementById("multiplatform-input");
const multiplatformSettings = document.getElementById("multiplatform-settings");

for (const input of projectTypeToggles) {
    input.onchange = refreshDisplayedProjectType;
};

// Add listeners to Fabric and Quilt checkboxes for controlling the Fabric-like checkbox,
// and refresh the Fabric-like status according to the default state.
document.getElementById("fabric-loader-input").onchange = refreshFabricLikeCheckbox;
document.getElementById("quilt-loader-input").onchange = refreshFabricLikeCheckbox;
refreshFabricLikeCheckbox();

// Add generated mod id placeholder when not specified manually
const modNameInput = document.getElementById("mod-name-input");
const modIdInput = document.getElementById("mod-id-input");

modNameInput.oninput = () => {
    refreshModIdPlaceholder();
    validateModId();
};

function refreshModIdPlaceholder() {
    modIdInput.placeholder = to_mod_id(modNameInput.value) ?? "";
}

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

function getProjectType() {
    for (const input of projectTypeToggles) {
        if (input.checked) {
            return input.getAttribute("projecttype");
        }
    }
}

function getMappingSet() {
    for (const input of document.getElementsByTagName("input")) {
        if (input.name !== "mappings") continue;
        if (input.checked) {
            return input.getAttribute("mappingset");
        }
    }
}

function updateState() {
    state.mod_name = modNameInput.value;
    state.mod_id = getModId();
    state.package_name = document.getElementById("package-input").value;
    state.game_version = mcSelect.value;
    state.project_type = getProjectType();
    state.mapping_set = getMappingSet();
    state.subprojects.fabric = document.getElementById("fabric-loader-input").checked;
    state.subprojects.forge = document.getElementById("forge-loader-input").checked;
    state.subprojects.neoforge = document.getElementById("neoforge-loader-input").checked && isNeoForgeAvailable();
    state.subprojects.quilt = document.getElementById("quilt-loader-input").checked;
    state.subprojects.fabric_likes = document.getElementById("fabric-like-input").checked && isFabricLikeAvailable();
    state.dependencies.architectury_api = document.getElementById("architectury-api-input").checked;
}

function showError(error) {
    let container = document.getElementById("error-message-container");
    container.textContent = error;
    container.classList.remove("hidden");
}

function clearError() {
    let container = document.getElementById("error-message-container");
    container.textContent = "";
    container.classList.add("hidden");
}

function refreshDisplayedProjectType() {
    if (multiplatformInput.checked) {
        multiplatformSettings.classList.remove("hidden");
    } else {
        multiplatformSettings.classList.add("hidden");
    }
}

function isFabricLikeAvailable() {
    const fabricInput = document.getElementById("fabric-loader-input");
    const quiltInput = document.getElementById("quilt-loader-input");
    return fabricInput.checked && quiltInput.checked;
}

function isNeoForgeAvailable() {
    const version = mcSelect.value;
    return supports_neoforge(version);
}

function refreshAvailablePlatforms() {
    const hasNeoForge = isNeoForgeAvailable();
    const neoForgeProjectInput = document.getElementById("neoforge-project-input");
    const neoForgeLoaderInput = document.getElementById("neoforge-loader-input");
    neoForgeProjectInput.disabled = !hasNeoForge;
    neoForgeLoaderInput.disabled = !hasNeoForge;

    // Change project type if Neo is not available.
    if (!hasNeoForge && neoForgeProjectInput.checked) {
        multiplatformInput.checked = true;
        neoForgeProjectInput.checked = false;
        refreshDisplayedProjectType();
    }
}

// Enables/disables the Fabric-like checkbox based on whether it can be selected for the current state.
function refreshFabricLikeCheckbox() {
    const hasFabricLike = isFabricLikeAvailable();
    const fabricLikeInput = document.getElementById("fabric-like-input");
    fabricLikeInput.disabled = !hasFabricLike;
}

document.getElementById("generate-button").onclick = async () => {
    updateState();

    if (state.mod_name === "") {
        showError("Mod name is empty");
        return;
    } else if (!isModIdValid()) {
        showError("Mod ID is not valid");
        return;
    } else if (state.package_name === "") {
        showError("Package name is empty");
        return;
    }

    clearError();
    await generate(state);
};

// Apply initial state
modNameInput.value = state.mod_name;
modIdInput.value = state.mod_id;
refreshModIdPlaceholder();
document.getElementById("package-input").value = state.package_name;
document.getElementById("architectury-api-input").checked = state.dependencies.architectury_api;

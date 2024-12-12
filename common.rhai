// Dioxus version. This only needs changed to update all templates.
// You can change this in development to be `path = "abc/"` or `git = "xyz"` for local development.
const DIOXUS_DEP_SRC_VAR = "dioxus_dep_src";
variable::set(DIOXUS_DEP_SRC_VAR, "version = \"0.6.0\"");

// These are variables that are used in liquid
const NEEDS_DEFAULT_PLATFORM_VAR = "needs_default_platform";
// const NEEDS_TAILWIND_PROMPT_VAR = "needs_tailwind_prompt";

const DIOXUS_DEP_FEAT_VAR = "dioxus_dep_features";
const DEFAULT_PLATFORM_VAR = "default_platform";

const IS_ROUTER_VAR = "is_router";
const IS_FULLSTACK_VAR = "is_fullstack";
// const IS_TAILWIND_VAR = "is_tailwind";

const FEATURES_VAR = "features";

let features = [
    "web = [\"dioxus/web\"]",
    "desktop = [\"dioxus/desktop\"]",
    "mobile = [\"dioxus/mobile\"]"
];
let platforms = [
    "Web",
    "Desktop",
    "Mobile"
];
let is_fullstack = variable::get(IS_FULLSTACK_VAR);

// Do we need a default platform?
if variable::get(NEEDS_DEFAULT_PLATFORM_VAR) {
    let is_default_platform_set = variable::is_set(DEFAULT_PLATFORM_VAR);

    // If set, check if valid, otherwise abort.
    // If not set, figure out if we need it.
    if is_default_platform_set {
        let value = variable::get(DEFAULT_PLATFORM_VAR);

        // Lowercase the platforms so they match the expected defined variable.
        let lowercased_platforms = [];
        for platform in platforms {
            lowercased_platforms += platform.to_lower();
        }

        // Check if provided definition is a valid choice.
        let is_valid = lowercased_platforms.some(|v| v == value);
        if !is_valid {
            abort("The `default_platform` variable is not valid based on the selected platforms.");
        }
    } else {
        // Make sure the user chose at least one platform.
        if platforms.len() == 0 {
            if is_mobile {
                variable::set(DEFAULT_PLATFORM_VAR, "mobile");
            } else {
                abort("You must choose at least one platform to build for.");
            }
        } else {
            let default_platform = variable::prompt("Which platform do you want DX to serve by default?", platforms[0], platforms);
            variable::set(DEFAULT_PLATFORM_VAR, default_platform.to_lower());
        }
    }
}


let is_router = variable::get(IS_ROUTER_VAR);
switch [is_router, is_fullstack] {
    [true, true] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"router\", \"fullstack\"]"),
    [true, false] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"router\"]"),
    [false, true] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"fullstack\"]"),
    _ => variable::set(DIOXUS_DEP_FEAT_VAR, "[]")
}

// Set features
variable::set(FEATURES_VAR, features);

// Rhai has weird returns. This ensures that the above statement isn't returned instead.
return;

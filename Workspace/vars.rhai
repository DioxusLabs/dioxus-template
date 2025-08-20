// This file sets variables based on the results of the shared-init script.

const WORKSPACE_MEMBERS_VAR = "workspace_members";
const DIOXUS_FEATURE_VAR = "dioxus_features";

const IS_FULLSTACK = "is_fullstack";
const IS_ROUTER = "is_router";

let workspace_members = ["packages/web", "packages/desktop", "packages/mobile"];

if variable::get(IS_FULLSTACK) {
    workspace_members += "packages/api"
}

variable::set(WORKSPACE_MEMBERS_VAR, workspace_members);

// Dioxus features
let features = [];
if variable::get(IS_ROUTER) {
    features.push("router");
}
if variable::get(IS_FULLSTACK) {
    features.push("fullstack");
}
let features_string = "";
for feature in features {
    if features_string.len() > 0 {
        features_string += ", ";
    }
    features_string += "\"" + feature + "\"";
}
variable::set(DIOXUS_FEATURE_VAR, features_string);

// This file handles conditional files for this specific sub-template.

const IS_FULLSTACK_VAR = "is_fullstack";
const IS_ROUTER_VAR = "is_router";
const IS_PROMPT = "is_prompt";

let is_fullstack = variable::get(IS_FULLSTACK_VAR);
if !is_fullstack {
    file::delete("packages/api");
    file::delete("packages/ui/src/echo.rs");
    file::delete("packages/ui/assets/styling/echo.css");
}

let is_router = variable::get(IS_ROUTER_VAR);
if !is_router {
    file::delete("packages/web/src/views");
    file::delete("packages/web/assets/blog.css");

    file::delete("packages/desktop/src/views");
    file::delete("packages/desktop/assets/blog.css");

    file::delete("packages/mobile/src/views");
    file::delete("packages/mobile/assets/blog.css");

    file::delete("packages/ui/src/navbar.rs");
    file::delete("packages/ui/assets/styling/navbar.css");
    file::delete("packages/ui/assets/styling/blog.css");
}

let is_prompt = variable::get(IS_PROMPT);
if !is_prompt {
    file::delete("AGENTS.md");
}

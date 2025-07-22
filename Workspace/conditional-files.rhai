// This file handles conditional files for this specific sub-template.

const IS_FULLSTACK_VAR = "is_fullstack";
const IS_ROUTER_VAR = "is_router";
const IS_PROMPT = "is_prompt";

let is_fullstack = variable::get(IS_FULLSTACK_VAR);
if !is_fullstack {
    file::delete("api");
    file::delete("ui/src/echo.rs");
    file::delete("ui/assets/styling/echo.css");
}

let is_router = variable::get(IS_ROUTER_VAR);
if !is_router {
    file::delete("web/src/views");
    file::delete("web/assets/blog.css");

    file::delete("desktop/src/views");
    file::delete("desktop/assets/blog.css");

    file::delete("mobile/src/views");
    file::delete("mobile/assets/blog.css");

    file::delete("ui/src/navbar.rs");
    file::delete("ui/assets/styling/navbar.css");
    file::delete("ui/assets/styling/blog.css");
}

let is_prompt = variable::get(IS_PROMPT);
if !is_prompt {
    file::delete("prompt.md");
    file::delete("prompt-small.md");
}

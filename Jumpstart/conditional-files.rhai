// This file handles conditional files for this specific sub-template.

const IS_TAILWIND_VAR = "is_tailwind";
const IS_ROUTER_VAR = "is_router";
const IS_FULLSTACK_VAR = "is_fullstack";
const IS_PROMPT_VAR = "is_prompt";

let is_tailwind = variable::get(IS_TAILWIND_VAR);
if !is_tailwind {
    file::delete("tailwind.css");
    file::delete("assets/tailwind.css");
}

let is_router = variable::get(IS_ROUTER_VAR);
if !is_router {
    file::delete("src/views");
    file::delete("src/components/navbar.rs");
    file::delete("assets/styling/navbar.css");
    file::delete("assets/styling/blog.css");
}

let is_fullstack = variable::get(IS_FULLSTACK_VAR);
if !is_fullstack {
    file::delete("src/components/echo.rs");
    file::delete("assets/styling/echo.css");
}

let is_prompt = variable::get(IS_PROMPT_VAR);
if !is_prompt {
    file::delete("AGENTS.md");
}

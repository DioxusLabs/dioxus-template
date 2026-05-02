// This file handles conditional files for this specific sub-template.

const IS_TAILWIND_VAR = "is_tailwind";
const IS_PROMPT = "is_prompt";

let is_tailwind = variable::get(IS_TAILWIND_VAR);
if !is_tailwind {
    file::delete("tailwind.css");
    file::delete("assets/tailwind.css");
}

let is_prompt = variable::get(IS_PROMPT);
if !is_prompt {
    file::delete("AGENTS.md");
}

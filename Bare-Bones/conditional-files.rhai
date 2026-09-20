// This file handles conditional files for this specific sub-template.

const IS_PROMPT = "is_prompt";

let is_prompt = variable::get(IS_PROMPT);
if !is_prompt {
    file::delete("prompt.md");
    file::delete("prompt-small.md");
}

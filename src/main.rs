mod hooks;

fn main() {
    let hook_id = hooks::set_hooks();
    hooks::unset_hooks(hook_id);
}
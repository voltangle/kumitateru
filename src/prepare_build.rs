use crate::utils::verify_project::verify_project;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project() {
    // Step 1: Verify project structure
    verify_project();
}
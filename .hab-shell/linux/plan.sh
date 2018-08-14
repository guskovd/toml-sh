pkg_name="toml-sh"
pkg_origin="guskovd"
pkg_version="1.0.3"

pkg_hab_shell_interpreter="bash"

pkg_deps=(
    core/grep
    core/bash
    core/coreutils
    core/gawk
    core/which
    core/git
    core/gcc
    core/make
    core/hab
    core/sudo
    core/rust
    core/gcc-libs
)

do_setup_environment() {
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/gcc-libs)/lib"
}

do_shell() {
    . ~/.bashrc
}

do_build() {
    return 0
}

do_install() {
    return 0
}

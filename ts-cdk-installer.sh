#!/bin/sh
# shellcheck shell=dash

# NOTE: This is the installer script for ts-cdk, a CLI tool for TypeScript + AWS CDK projects.
# NOTE: This script automatically downloads the appropriate binary for your system and sets up your environment.

# NOTE: KSH version check - some versions of KSH are not compatible with this installer
if [ "$KSH_VERSION" = 'Version JM 93t+ 2010-03-05' ]; then
    echo 'this installer does not work with this ksh93 version; please try bash!' >&2
    exit 1
fi

set -u

# NOTE: Configuration variables that can be overridden through environment variables
APP_NAME="ts-cdk"
APP_VERSION="0.1.0"
ARTIFACT_DOWNLOAD_URL="${INSTALLER_DOWNLOAD_URL:-https://github.com/ren-yamanashi/ts-cdk/releases/download/v0.1.0}"
PRINT_VERBOSE=${INSTALLER_PRINT_VERBOSE:-0}
PRINT_QUIET=${INSTALLER_PRINT_QUIET:-0}
NO_MODIFY_PATH=${INSTALLER_NO_MODIFY_PATH:-0}

usage() {
    cat <<EOF
ts-cdk-installer.sh

The installer for ts-cdk $APP_VERSION

This script detects what platform you're on and fetches an appropriate archive from
$ARTIFACT_DOWNLOAD_URL
then unpacks the binaries and installs them to the first of the following locations

    \$TS_CDK_INSTALL/bin
    \$HOME/.ts-cdk/bin

It will then add that dir to PATH by adding the appropriate line to your shell profiles.

USAGE:
    ts-cdk-installer.sh [OPTIONS]

OPTIONS:
    -v, --verbose
            Enable verbose output

    -q, --quiet
            Disable progress output

        --no-modify-path
            Don't configure the PATH environment variable

    -h, --help
            Print help information
EOF
}

# NOTE: Function to configure PATH in shell configuration files
configure_shell_path() {
    local _install_dir="$1"
    local _bin_name="$2"

    # NOTE: Skip PATH modification if ts-cdk is already in PATH
    if command -v ts-cdk >/dev/null 2>&1; then
        existing_ts_cdk=$(command -v ts-cdk)
        if [ "$existing_ts_cdk" != "$_install_dir/$_bin_name" ]; then
            say "ts-cdk is already installed at $existing_ts_cdk"
            say "Not modifying PATH as ts-cdk is already available"
        fi
        return 0
    fi

    # NOTE: Add to current shell's PATH immediately for instant usage
    case "$(uname -s)" in
        MINGW* | MSYS* | CYGWIN* | Windows_NT)
            export PATH="$_install_dir:$PATH"
            ;;
        *)
            export PATH="$_install_dir:$PATH"
            ;;
    esac

    # NOTE: Configure PATH in common shell configuration files
    for profile in ~/.profile ~/.bashrc ~/.zshrc; do
        if [ -w "$profile" ]; then
            say "adding $_install_dir to PATH in $profile"
            echo "" >> "$profile"
            echo "# ts-cdk PATH configuration" >> "$profile"
            echo "export PATH=\"$_install_dir:\$PATH\"" >> "$profile"
        fi
    done

    # NOTE: Special handling for Fish shell
    configure_fish_path "$_install_dir"

    # NOTE: Display the command to update PATH in the current session
    case "$(uname -s)" in
        MINGW* | MSYS* | CYGWIN* | Windows_NT)
            say "To use ts-cdk in the current session, run:"
            say "    export PATH=\"$_install_dir:\$PATH\""
            ;;
        *)
            say "To use ts-cdk in the current session, run:"
            say "    export PATH=\"$_install_dir:\$PATH\""
            ;;
    esac
}

# NOTE: Function to configure PATH for Fish shell
configure_fish_path() {
    local _install_dir="$1"
    
    if command -v fish >/dev/null 2>&1; then
        fish_config_dir="$HOME/.config/fish/conf.d"
        if [ ! -d "$fish_config_dir" ]; then
            mkdir -p "$fish_config_dir"
        fi
        fish_config_file="$fish_config_dir/ts-cdk.fish"
        say "adding $_install_dir to PATH in $fish_config_file"
        echo "# ts-cdk PATH configuration" > "$fish_config_file"
        echo "set -gx PATH \$PATH $_install_dir" >> "$fish_config_file"
    fi
}

download_binary_and_run_installer() {
    # NOTE: Check for required system commands before proceeding
    downloader --check
    need_cmd uname
    need_cmd mktemp
    need_cmd chmod
    need_cmd mkdir
    need_cmd rm
    need_cmd tar
    need_cmd unzip

    for arg in "$@"; do
        case "$arg" in
            --help)
                usage
                exit 0
                ;;
            --quiet)
                PRINT_QUIET=1
                ;;
            --verbose)
                PRINT_VERBOSE=1
                ;;
            --no-modify-path)
                NO_MODIFY_PATH=1
                ;;
            *)
                OPTIND=1
                if [ "${arg%%--*}" = "" ]; then
                    err "unknown option $arg"
                fi
                while getopts :hvq sub_arg "$arg"; do
                    case "$sub_arg" in
                        h)
                            usage
                            exit 0
                            ;;
                        v)
                            PRINT_VERBOSE=1
                            ;;
                        q)
                            PRINT_QUIET=1
                            ;;
                        *)
                            err "unknown option -$OPTARG"
                            ;;
                        esac
                done
                ;;
        esac
    done

    get_architecture || return 1
    local _arch="$RETVAL"
    assert_nz "$_arch" "arch"

    local _ext=".tar.gz"
    # Use .zip for Windows
    case "$_arch" in
        *-pc-windows-msvc)
            _ext=".zip"
            need_cmd unzip
            ;;
    esac

    local _artifact_name="${APP_NAME}-${_arch}${_ext}"
    local _url="${ARTIFACT_DOWNLOAD_URL}/${_artifact_name}"
    local _dir
    _dir="$(ensure mktemp -d)" || return 1
    local _file="${_dir}/${_artifact_name}"

    say "downloading ${APP_NAME} ${APP_VERSION} ${_arch}" 1>&2

    ensure mkdir -p "$_dir"
    downloader "$_url" "$_file"

    case "$_ext" in
        ".tar."*)
            ensure tar xf "$_file" -C "$_dir"
            ;;
        ".zip")
            ensure unzip "$_file" -d "$_dir"
            ;;
        *)
            err "unknown archive format: $_ext"
            ;;
    esac

    local _install_dir
    if [ -n "${TS_CDK_INSTALL:-}" ]; then
        _install_dir="$TS_CDK_INSTALL/bin"
    elif [ -n "${HOME:-}" ]; then
        _install_dir="$HOME/.ts-cdk/bin"
    else
        err "neither TS_CDK_INSTALL nor HOME is set"
    fi

    say "installing to $_install_dir"
    ensure mkdir -p "$_install_dir"

    local _bin_name="${APP_NAME}"
    if [ "$_arch" = "x86_64-pc-windows-msvc" ]; then
        _bin_name="${_bin_name}.exe"
    fi

    ensure mv "$_dir/$_bin_name" "$_install_dir"
    ensure chmod +x "$_install_dir/$_bin_name"

    if [ "$NO_MODIFY_PATH" = "0" ]; then
        configure_shell_path "$_install_dir" "$_bin_name"
    fi

    ignore rm -rf "$_dir"

    say "ts-cdk installation completed!"
    say "ts-cdk is now available in your current shell!"
    say "Try running: ts-cdk --help"
    say "When PATH is not passed, you need to start a new terminal session or run one of the following commands:"
    say "    source ~/.bashrc  # for bash"
    say "    source ~/.zshrc   # for zsh"
    say "    source ~/.config/fish/conf.d/ts-cdk.fish  # for fish"
}

say() {
    if [ "$PRINT_QUIET" = "0" ]; then
        echo "$1"
    fi
}

say_verbose() {
    if [ "$PRINT_VERBOSE" = "1" ]; then
        echo "$1"
    fi
}

err() {
    say "$1" >&2
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

ignore() {
    "$@"
}

downloader() {
    if check_cmd curl; then
        _dld=curl
    elif check_cmd wget; then
        _dld=wget
    else
        _dld='curl or wget'
    fi

    if [ "$1" = --check ]; then
        need_cmd "$_dld"
    elif [ "$_dld" = curl ]; then
        curl -sSfL "$1" -o "$2"
    elif [ "$_dld" = wget ]; then
        wget "$1" -O "$2"
    else
        err "Unknown downloader"
    fi
}

get_architecture() {
    local _ostype
    local _cputype

    # If INSTALLER_TARGET_TRIPLE is set, use it directly
    if [ -n "${INSTALLER_TARGET_TRIPLE:-}" ]; then
        RETVAL="$INSTALLER_TARGET_TRIPLE"
        return
    fi

    _ostype="$(uname -s)"
    _cputype="$(uname -m)"

    case "$_ostype" in
        Linux)
            _ostype=unknown-linux-gnu
            ;;
        Darwin)
            _ostype=apple-darwin
            ;;
        MINGW* | MSYS* | CYGWIN* | Windows_NT)
            _ostype=pc-windows-msvc
            ;;
        *)
            err "unsupported OS type: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64)
            _cputype=x86_64
            ;;
        aarch64 | arm64)
            _cputype=aarch64
            ;;
        *)
            err "unsupported CPU type: $_cputype"
            ;;
    esac

    RETVAL="${_cputype}-${_ostype}"
}

download_binary_and_run_installer "$@" || exit 1 
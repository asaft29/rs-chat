#!/bin/bash

detect_package_manager() {
  if command -v apt-get >/dev/null 2>&1; then echo "apt"
  elif command -v pacman >/dev/null 2>&1; then echo "pacman"
  elif command -v dnf >/dev/null 2>&1; then echo "dnf"
  elif command -v zypper >/dev/null 2>&1; then echo "zypper"
  elif command -v brew >/dev/null 2>&1; then echo "brew"
  else echo "unknown"
  fi
}

install_package() {
  local package="$1"
  local manager
  manager=$(detect_package_manager)

  case "$manager" in
    apt) sudo apt update && sudo apt install -y "$package" ;;
    pacman) sudo pacman -S "$package" ;;
    dnf) sudo dnf install -y "$package" ;;
    zypper) sudo zypper install -y "$package" ;;
    brew) brew install "$package" ;;
    *) echo "Unsupported package manager. Please install '$package' manually."; exit 1 ;;
  esac
}

check_and_install() {
  local pkg="$1"
  local cmd="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Required command '$cmd' is not installed."
    read -p "Do you want to install '$pkg'? [Y/n] " answer
    case "${answer,,}" in
      y|yes|"") install_package "$pkg" || exit 1 ;;
      *) echo "Cannot continue without '$pkg'. Exiting."; exit 1 ;;
    esac
  fi
}

check_and_install "telnet" "telnet"
check_and_install "fortune-mod" "fortune"

RAND_ID=$(shuf -i 10000-99999 -n 1)
NAME="Guest$RAND_ID"
PIPE="/tmp/telnet_input_$$"

cleanup() {
  rm -f "$PIPE"
}
trap cleanup EXIT

mkfifo "$PIPE"

telnet 127.0.0.1 8080 < "$PIPE" &
TELNET_PID=$!

{
  echo "$NAME" | tee /dev/tty
  while kill -0 "$TELNET_PID" 2>/dev/null; do
    sleep 1
    fortune -s | awk 'length < 80 && NF > 2' | head -n 1 | tee /dev/tty
  done
} > "$PIPE"

wait "$TELNET_PID"
echo "Disconnected from server."


#!/bin/sh

case "$(uname -s)" in
Darwin)
  rm -rf ~/Library/Application\ Support/Keiga
  ;;
Linux)
  data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
  rm -rf "$data_home"/keiga
  ;;
MINGW*|MSYS*|CYGWIN*)
  rm -rf "$APPDATA"/Keiga
  ;;
*)
  echo "unsupported OS: $(uname -s)" >&2
  exit 1
  ;;
esac

exit

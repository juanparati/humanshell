# HumanShell

hb2_ask() {
  local question="$BUFFER"

  # If the buffer is empty, do nothing (or restore default behavior).
  if [[ -z "$question" ]]; then
    zle beginning-of-line
    return
  fi

  echo "\nTranslating..."
  local command=$(hs "$question" 2>/dev/null | tail -n 1)

  # Clear the current command line.
  zle reset-prompt

  if [ $? -eq 0 ] && [ -n "$command" ]; then
    # Put the command into the buffer
    BUFFER="$command"
    CURSOR=${#BUFFER}
  fi
}

zle -N hb2_ask

bindkey '^H' hb2_ask
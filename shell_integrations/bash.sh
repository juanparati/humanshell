# Bash
hb2_ask() {
  local question="$READLINE_LINE"

  # If the line is empty, do nothing.
  if [[ -z "$question" ]]; then
    return
  fi

  # Optionally show a transient message
  printf '\nTranslating...\n' >&2

  # Call your translator (hs) and take the last line of output
  local command
  command=$(hs "$question" 2>/dev/null | tail -n 1)

  # If we got a command back, replace the current line and move cursor to end
  if [[ -n "$command" ]]; then
    READLINE_LINE="$command"
    READLINE_POINT=${#READLINE_LINE}
  fi
}

# Bind Ctrl+H to trigger the function
bind -x '"\C-h":hb2_ask'
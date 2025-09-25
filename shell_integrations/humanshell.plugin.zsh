# Humanshell ZSH Plugin
# 
# Installation:
# 1. Copy this file to your ZSH plugins directory
# 2. Add 'humanshell' to your plugins list in ~/.zshrc
# 3. Or source this file directly: source /path/to/humanshell.plugin.zsh
#
# Usage:
# Type a natural language command and press Ctrl+H to translate it to shell command

hs_ask() {
  # ZLE special variable $BUFFER contains the current command line content
  local question="$BUFFER"

  # If the buffer is empty, do nothing
  if [[ -z "$question" ]]; then
    return
  fi

  # Show feedback to user
  printf '\nTranslating...\n' >&2

  # Call the humanshell translator and get the last line of output
  local command
  command=$(hs "$question" 2>/dev/null | tail -n 1)

  # Clear the current command line and reset prompt
  zle reset-prompt

  # If we got a valid command back, replace the buffer content
  if [[ $? -eq 0 && -n "$command" ]]; then
    BUFFER="$command"
    CURSOR=${#BUFFER}
  else
    # If translation failed, restore original question and show error
    BUFFER="$question"
    CURSOR=${#BUFFER}
    printf 'Translation failed. Please check your input or try again.\n' >&2
  fi
}

# Register the function as a ZLE widget
zle -N hs_ask

# Bind Ctrl+H to trigger the function
bindkey '^H' hs_ask
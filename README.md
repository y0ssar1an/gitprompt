# gitprompt

This is a tiny little program that prints the current git branch in oh-my-zsh style. The 💩 means the repo is [dirty](https://stackoverflow.com/questions/20642980/does-git-dirty-mean-files-not-staged-or-not-committed-glossary-conflict).

You can use it like this:
```sh
# enable running shell commands in the prompt string
setopt prompt_subst

# fancy prompt formatting documented at http://zsh.sourceforge.net/Doc/Release/Prompt-Expansion.html
export PROMPT='%F{cyan}%B%40<..<%3~%b%f$(gitprompt) '
```

![gitprompt screenshot](https://i.imgur.com/pa62lmd.jpg)

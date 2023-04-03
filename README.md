# Tshort

## Instalacion

`cargo install tshort`


## Configuracion

Configurar los projectos en tu director `~/.config/projects.json`

con la estructura
```json

{
  "directories": [
    {
      "name": "Rust",
      "directory": "~/code/rust",
      "icon": " ",
      "color": "#cd9747"
    },
    {
      "name": "Go",
      "directory": "~/code/go",
      "icon": " ",
      "color": "#00a3cc"
    },
    {
      "name": "PHP",
      "directory": "~/code/php",
      "icon": " ",
      "color": "#5e79be"
    },
    {
      "name": "Javascript",
      "directory": "~/code/javascript/",
      "icon": " ",
      "color": "#ecb75d"
    }
  ],
  "projects": [
    {
      "name": "Neovim",
      "directory": "~/.config/nvim",
      "icon": " ",
      "color": "#509a3a"
    },
    {
      "name": "Awesome",
      "directory": "~/.config/awesome",
      "icon": " ",
      "color": "#535d6c"
    }
  ]
}

```

Con esto vas a poder rapidamente buscar en esos directorios tus projectos

Para integrarlo con tmux yo lo utilizo

```sh
bind-key u run-shell  'tmux popup -E tshort bind 1'
```

y asi con varias keybindings para un rapido acceso.


Para tener informacion de los projectos bindeados el comando `tshort list` me da esta inforamcion

Tambien estan los comandos
- `tshort` sin argumentos que va a simplemente listar y cambiar de session
- `tshort forget {key}` que va a borrar la asociacion pero sin cerrar la session.

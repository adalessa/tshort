# Tshort

## Instalacion

`cargo install tshort`


## Configuracion

Configurar los projectos en tu director `~/.config/projects.json`

con la estructura
```json
{
  "projects": [
    {
      "name": "php",
      "directory": "~/code/php",
      "icon": " ",
      "color": "#0000ff"
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

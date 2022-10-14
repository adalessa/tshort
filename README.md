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
      "icon": "system-icon"
    }
  ],
  "cli": {
    "editor": "nvim"
  },
  "gui": {
    "rofi_menu": "pathtothetheme",
    "editor": "neovide"
  }
}
```

Para correr con el menu de rofi utilizar
```sh
tshort gui
```

Para temes de rofi un buen repositorio es https://github.com/adi1090x/rofi es cuale estoy usando para launcher y para gui

Otro tema visual son los iconos. Podes usar iconos ya registrados o hacer como yo y registrar los propios
```
xdg-icon-resource install --size 128 alpha-rust.png
```
asegurate del tamanio sea cuadrado preferentemente de 128.

Importante agregar el prefijo y este mismo utilizarlo en la configuracion


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

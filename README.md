## `rsnamer`: rename sway workspaces at the push of a button

opinionated utility to rename the current workspace to give a more meaningful
label. maintains number navigation by prepending number to workspace name: on 
input `browser`, renames workspace `3` to `3:browser`.

#### install

depends on rofi for now. clone, run `cargo build --release`, and create a 
symlink to somewhere in your path if you wish.

#### config

```text
bindsym $mod+Shift+r path/to/rsnamer
```

to maintain number navigation, we ask that you modify your sway config. 
you probably have the following (at least i know i did): 

```text
bindsym $mod+<num> workspace <num>
```

changing it to 

```text
bindsym $mod+<num> workspace number <num>
```

will make workspace traversal behave as expected.

#### todos

 - [x] maintain number navigation by prepending number to new name.
 - [ ] make number behavior configurable
 - [ ] better text input dialog


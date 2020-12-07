# chmod2umask

chmod2umask is a tool for converts the `chmod` file mode to the `umask` symbolic mode.

For examples, if the `chmod` file mode is `0644`, then it converts to `u=rw,g=r,o=r`. Afterthat, you could configures `umask` by symbolic mode `umask -S u=rw,g=r,o=r`.

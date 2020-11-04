# umask converter

This repository is a simple umask converter that converts Golang os.FileMode to `umask` symbolic output, it helps you to configures `umask` by symbolic mode.

For examples, if the `os.FileMode(0644)`, it will converts to `u=rw,g=r,o=r`.Then, you could configures `umask` by symbolic mode `umask -S u=rw,g=r,o=r`.

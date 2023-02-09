# Tree Generation in Rust
This project is uses a recursive pattern where each branch is a tree in itself, to control this recursion a depth level can be set. Leaves are also genereted based on the current depth of the function.

Because of the recursion this program is insanly slow, but as it's more of a hello rust program no thought has been put into performance.

## Depends on rust
The project is written in rust and as such a rust environment is needed. By installing cargo and running 
```
    cargo run
```
dependencies should be installed automatically and the project should run.

## Below are some generated trees

### Generated with depth=10 and no leaves
![Tree](./assets/tree.png)

### Genereated with depth=8 and sakura leaves
![Tree](./assets/sakura.png)
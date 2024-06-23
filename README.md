# tmux-profiles

### Usage
Launch a single profile: `tmux-profiles launch <profile_name>`
Launch a group of profiles: `tmux-profiles group <group_name>`
List all available profiles `tmux-profiles list`

config lives in `~/tmux-profiles.toml`

### Config example
```toml
[[session]]
name = "test"
group = "personal"
windows = [
    { 
        name = "main",
        panes = [
            {
                location = "/home/test/test"
            },
            {
                location = "/home/test/test",
                command = "cargo build"
            }
        ]
    },
    {
        name = "test",
        panes = [
            {
                location = "/home/test/test/_unit_test",
                command = "cargo test"
            }
        ]
    }
]
```


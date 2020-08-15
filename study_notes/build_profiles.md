
>
    Cargo has two main profiles: the dev profile Cargo uses when you run cargo build 
    and the release profile Cargo uses when you run cargo build --release. 
    The dev profile is defined with good defaults for development, 
    and the release profile has good defaults for release builds.
    
    
~~~toml

[profile.dev]
opt-level = 1

~~~    
This code overrides the default setting of 0. Now when we run cargo build, Cargo will use the defaults for the dev profile
 plus our customization to opt-level. 
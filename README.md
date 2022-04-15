# Ungine.rs
My journey into rust programming


A Rust ECS-based game engine, inspired by Valve's Source engine

Modules:
 - [FileSystem](https://github.com/ENDERZOMBI102/Ungine.rs/tree/master/filesystem): Module with file system-related code 
    - `upkf.rs`: Manages Ungine PaK Files ( `.upkf` )
    - `layer.rs`: Manages the layered file system (NOT PRESENT)
    - `compressor.rs`: Application to compress a folder into an `.upkf` file
    - `decompressor.rs`: Application to decompress an `.upkf` file back into a folder
 - [Tier0](https://github.com/ENDERZOMBI102/Ungine.rs/tree/master/tier0): Utility and base stuff
 	- `commandline.rs`: Command line parsing
 	- `config_file.rs`: Parsing keyvalue-based custom `.cfg` files
 - [Renderer](https://github.com/ENDERZOMBI102/Ungine.rs/tree/master/renderer): Not much to say, its the WIP renderer
 - [Rich Presence](https://github.com/ENDERZOMBI102/Ungine.rs/tree/master/richpresence): Engine rich presence client for various servers
 	- `discord.rs`: Implementation of the discord richpresence client (CURRENTLY IN `lib.rs`)

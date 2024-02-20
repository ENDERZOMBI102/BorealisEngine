# Borealis Engine
A Rust game engine, based on ECS and inspired by Valve's Source engine

Modules:
 - [FileSystem](filesystem): Module with file system-related code 
    - `bpak`: Manages Borealis PAK archives ( `.bpak` )
    - `layered`: Manages the layered file system ( WiP )
 - [Tier0](tier0): Utility and base stuff
 	- `commandline.rs`: Command line parsing
 	- `config_file.rs`: Parsing KeyValue-based custom `.cfg` files
 - [Renderer](renderer): Not much to say, it's the WIP renderer
 - [Rich Presence](richpresence): Engine rich presence client for various servers
 	- `discord.rs`: Implementation of various rich-presence clients
- [Tools](tools): Various tools for development or for use
  - `fscli`: Shell to interact with various fs-related commands and utilities
  - `compressor`: Application to compress a folder into a bpak archive
  - `decompressor`: Application to decompress a bpak archive back into a folder

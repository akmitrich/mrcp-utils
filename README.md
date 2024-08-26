# MRCP utilities of Optimal City Technologies
The crate contains some utilities for integration of ASR and TTS services into UniMRCP server.

## Build
Before you start make sure that UniMRCP lib installed on your system along with its dependencies. Also environment variables `UNIMRCP_PATH`, `APR_LIB_PATH`, `APR_INCLUDE_PATH` should contain paths to them. Otherwise build script will use default paths, see source code. These bindings are fully unsafe, so be aware of that. The build depends on UniMRCP version 1.8.0 or higher. 

### Pull requests
Are welcomed!
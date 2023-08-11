# Injective protobuf generated in different programming languages

To generate the proto files for all the supported languajes run `make run-full`
The current supported languajes are:
- C++
- C#
- Python
- Rust

The script required `buf` installed to generate the proto files for C++, Python and Rust.
For the C# proto generation, the script requires `protoc` installed. It then uses the GrpTools plugin to generate the Grpc components. Please make sure that the plugin in grpc.tools/tools/<os_specific_folder> has execution permission

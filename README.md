# Injective protobuf generated in different programming languages

The current supported languajes are:
- C++ (proto files in cpp_protos.zip)
- C# (proto files in csharp_protos.zip)
- Java (proto files in java_protos.zip)
- Python (proto files in python_protos.zip)
- Rust (proto files in rust_protos.zip)

Use the components in the ZIP files for each of the different languajes.

## Original proto files
All original proto files can be found in the `all_protos` folder


## Maven dependencies for Java
If you are using the Injective proto components in Java you might want to use the following Maven dependencies (or use it as the base for your own Maven configuration):

```
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.test</groupId>
    <artifactId>injective_proto</artifactId>
    <version>1.0-SNAPSHOT</version>

    <properties>
        <maven.compiler.source>1.8</maven.compiler.source>
        <maven.compiler.target>1.8</maven.compiler.target>
    </properties>

    <dependencies>
        <dependency>
            <groupId>com.google.protobuf</groupId>
            <artifactId>protobuf-java</artifactId>
            <version>3.25.1</version>
        </dependency>

        <dependency>
            <groupId>io.grpc</groupId>
            <artifactId>grpc-protobuf</artifactId>
            <version>1.59.0</version>
        </dependency>

        <dependency>
            <groupId>io.grpc</groupId>
            <artifactId>grpc-stub</artifactId>
            <version>1.59.0</version>
        </dependency>

        <dependency>
            <groupId>javax.annotation</groupId>
            <artifactId>javax.annotation-api</artifactId>
            <version>1.3.2</version>
        </dependency>
    </dependencies>

</project>
```


## Update the proto components in the repository

To generate the proto files for all the supported languages run:

```bash
make run-full
```

### Tooling requirements

- `buf` CLI is required.
- No local `protoc`, `grpc.tools`, or `rust_plugins` binaries are required for generation.
- The generation process uses pinned Buf remote plugins (configured in `buf.gen.yaml`) to keep output reproducible.

### Generation flow

- `make generate` syncs the source protos under `proto/` and runs a single `buf generate` for all supported languages.
- `scripts/normalize_generated.py` performs compatibility normalization so C# and Rust outputs keep the expected source-relative tree shape.
- `all_protos` is refreshed from `proto/` after generation.

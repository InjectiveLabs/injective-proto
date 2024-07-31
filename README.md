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

To generate the proto files for all the supported languajes run `make run-full`

The script required `buf` installed to generate the proto files for C++, Java and Python.
For the C# and Rust proto generation, the script requires `protoc` installed. It then uses the GrpTools plugin to generate the Grpc components. Please make sure that the plugin in grpc.tools/tools/<os_specific_folder> has execution permission
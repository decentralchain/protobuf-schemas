# About
unitoken protobuf schemas repository

# How to use
## Java
Add dependency to your `pom.xml`
```xml
<dependency>
    <groupId>com.decentralchain</groupId>
    <artifactId>protobuf-schemas</artifactId>
    <version>{version}</version>
</dependency>
```
## ScalaPB
1. Add dependency to your `build.sbt`: 
```scala
libraryDependencies += ("com.decentralchain" % "protobuf-schemas" % "{version}" classifier "proto") % "protobuf"
```
2. Configure ScalaPB to compile external schemas with:
```scala
inConfig(Compile)(Seq(
   PB.protoSources in Compile := Seq(PB.externalIncludePath.value),
   includeFilter in PB.generate := new SimpleFileFilter((f: File) => f.getName.endsWith(".proto") && f.getParent.endsWith("unitoken")),
   PB.targets += scalapb.gen(flatPackage = true) -> sourceManaged.value
 ))
```
3. If you use SNAPSHOT version, add this line
```scala
resolvers += Resolver.sonatypeRepo("snapshots")
```
## JavaScript
Npm package: `@unitoken/protobuf-serialization`.

It contains generated JavaScript classes, TypeScript definitions as well as raw proto files. The default build uses CommonJS and includes all of the proto files. We used `pbjs` to build JavaScript and `pbts` to build TypeScript definitions.

You could also make your own custom build from raw `.proto` files, for example, if you want to use only a subset of proto schemas or gRPC services. They can be found in `@unitoken/protobuf-serialization/proto` directory.

`long.js` is used for 64-bit integers: `int64`, `uint64`, etc.

Example:
1. `npm install --save @unitoken/protobuf-serialization`
2. Default build usage
```javascript
import { unitoken } from '@unitoken/protobuf-serialization';

const block = new unitoken.Block();
block.header = // ... set necessary fields

const buffer = unitoken.Block.encode(block);

const blockDecoded = unitoken.Block.decode(buffer);
```

## C#
1. Add `App.config`, `packages.config` to your C# solution 
2. Add 
```
  <ItemGroup>
    <Protobuf Include="proto\unitoken\*.proto" OutputDir="unitoken\%(RelativePath)" GrpcServices="None" />
    <Protobuf Include="proto\unitoken\events\*.proto" OutputDir="unitoken\events\%(RelativePath)" GrpcServices="None" />
    <Protobuf Include="proto\unitoken\node\grpc\*.proto" OutputDir="unitoken\node\grpc\%(RelativePath)" GrpcServices="Both" />
  </ItemGroup>
```
to your `.csproj` file. After this just build your project.

or as alternative you can use util protoc, for example:
``` protoc --csharp_out=RelativePath  --proto_path=RelativePathToProtoDir RelativePathToProtoFile```

Also there is a NuGet package decentralchain.ProtobufSchema with this project.

## Rust
Add dependency to your `Cargo.toml`
```toml
[dependencies]
unitoken-protobuf-schemas = { git = "https://github.com/decentralchain/protobuf-schemas" }
```

# How to compile locally
## Java
Use `mvn package` to create JAR artifacts:
1. `protobuf-schemas-{version}-proto.jar` - raw .proto files
2. `protobuf-schemas-{version}.jar` - protoc-generated Java classes

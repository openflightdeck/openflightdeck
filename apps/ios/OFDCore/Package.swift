// swift-tools-version:5.9
// Swift Package mit dem Rust-Kern (ofd-ffi). XCFramework und ofd_ffi.swift
// werden von `cargo xtask ios` erzeugt und sind nicht eingecheckt.
import PackageDescription

let package = Package(
    name: "OFDCore",
    platforms: [.iOS(.v17)],
    products: [
        .library(name: "OFDCore", targets: ["OFDCore"]),
    ],
    targets: [
        .binaryTarget(name: "ofd_ffiFFI", path: "ofd_ffiFFI.xcframework"),
        .target(name: "OFDCore", dependencies: ["ofd_ffiFFI"], path: "Sources/OFDCore"),
    ]
)

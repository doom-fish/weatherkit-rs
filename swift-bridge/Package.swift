// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "WeatherKitBridge",
    platforms: [
        .macOS(.v13),
    ],
    products: [
        .library(name: "WeatherKitBridge", type: .static, targets: ["WeatherKitBridge"]),
    ],
    targets: [
        .target(
            name: "WeatherKitBridge",
            path: "Sources/WeatherKitBridge",
            publicHeadersPath: "include",
            cSettings: [
                .headerSearchPath("include"),
            ],
            linkerSettings: [
                .linkedFramework("WeatherKit"),
                .linkedFramework("CoreLocation"),
                .linkedFramework("Foundation"),
            ]
        ),
    ]
)

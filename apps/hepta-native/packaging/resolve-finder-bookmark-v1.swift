#!/usr/bin/swift

import Foundation

guard CommandLine.arguments.count == 2 else {
    fputs("usage: resolve-finder-bookmark-v1.swift PATH\n", stderr)
    exit(64)
}

do {
    let bookmarkPath = URL(fileURLWithPath: CommandLine.arguments[1]).standardizedFileURL
    let bookmarkData = try URL.bookmarkData(withContentsOf: bookmarkPath)
    var stale = false
    let resolved = try URL(
        resolvingBookmarkData: bookmarkData,
        options: [.withoutUI, .withoutMounting],
        relativeTo: nil,
        bookmarkDataIsStale: &stale
    )
    let result: [String: Any] = [
        "schema_version": 1,
        "kind": "hepta-finder-bookmark-resolution",
        "bookmark_path": bookmarkPath.path,
        "resolved_target": resolved.resolvingSymlinksInPath().standardizedFileURL.path,
        "bookmark_data_stale": stale
    ]
    let encoded = try JSONSerialization.data(withJSONObject: result, options: [.sortedKeys])
    FileHandle.standardOutput.write(encoded)
    FileHandle.standardOutput.write(Data([0x0a]))
} catch {
    fputs("unable to resolve Finder bookmark: \(error)\n", stderr)
    exit(1)
}

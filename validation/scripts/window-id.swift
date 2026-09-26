import CoreGraphics
import Foundation

guard CommandLine.arguments.count == 2 else {
    fputs("usage: window-id.swift <owner-or-title-fragment>\n", stderr)
    exit(2)
}

let argument = CommandLine.arguments[1].lowercased()
let searchAllSpaces = argument.hasPrefix("all:")
let needle = searchAllSpaces ? String(argument.dropFirst(4)) : argument
let listAll = needle == "--all"
let options: CGWindowListOption = (listAll || searchAllSpaces)
    ? [.optionAll, .excludeDesktopElements]
    : [.optionOnScreenOnly, .excludeDesktopElements]
guard let windows = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] else {
    exit(1)
}

for window in windows {
    let owner = (window[kCGWindowOwnerName as String] as? String) ?? ""
    let title = (window[kCGWindowName as String] as? String) ?? ""
    let layer = (window[kCGWindowLayer as String] as? Int) ?? -1
    let alpha = (window[kCGWindowAlpha as String] as? Double) ?? 0
    let bounds = (window[kCGWindowBounds as String] as? [String: Any]) ?? [:]
    let width = (bounds["Width"] as? Double) ?? 0
    let height = (bounds["Height"] as? Double) ?? 0
    if (needle == "--list" || listAll), layer == 0, alpha > 0 {
        let number = (window[kCGWindowNumber as String] as? UInt32) ?? 0
        print("\(number)\t\(owner)\t\(title)\t\(Int(width))x\(Int(height))")
        continue
    }
    if layer == 0,
       alpha > 0,
       width >= 500,
       height >= 500,
       owner.lowercased().contains(needle) || title.lowercased().contains(needle),
       let number = window[kCGWindowNumber as String] as? UInt32 {
        print(number)
        exit(0)
    }
}

if needle == "--list" || listAll {
    exit(0)
}

fputs("no visible window matched \(needle)\n", stderr)
exit(1)

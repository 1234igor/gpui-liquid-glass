import AppKit
import SwiftUI

private let variants = [
    "regular",
    "clear",
    "regular-tinted",
    "clear-tinted",
    "identity",
]
private let variant = CommandLine.arguments.dropFirst().first(where: variants.contains) ?? "regular"

private final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        true
    }
}

private struct SwiftUIWindowGlass: View {
    let variant: String

    private var material: SwiftUI.Glass {
        let coral = Color(red: 1, green: 0.22, blue: 0.28)
        switch variant {
        case "clear": return SwiftUI.Glass.clear
        case "regular-tinted": return SwiftUI.Glass.regular.tint(coral)
        case "clear-tinted": return SwiftUI.Glass.clear.tint(coral)
        case "identity": return SwiftUI.Glass.identity
        default: return SwiftUI.Glass.regular
        }
    }

    var body: some View {
        Color.clear
            .frame(width: 920, height: 620)
            .glassEffect(material, in: Rectangle())
            .ignoresSafeArea()
    }
}

let application = NSApplication.shared
private let delegate = AppDelegate()
application.delegate = delegate
application.setActivationPolicy(.regular)

let styleMask: NSWindow.StyleMask = [.titled, .closable, .fullSizeContentView]
let frame = CGRect(origin: .zero, size: CGSize(width: 920, height: 620))
let contentRect = NSWindow.contentRect(forFrameRect: frame, styleMask: styleMask)
let window = NSWindow(
    contentRect: contentRect,
    styleMask: styleMask,
    backing: .buffered,
    defer: false
)
window.title = "Native Window Glass - \(variant)"
window.titleVisibility = .hidden
window.titlebarAppearsTransparent = true
window.isOpaque = false
window.backgroundColor = NSColor(srgbRed: 0, green: 0, blue: 0, alpha: 0.0001)

let nativeContent = NSView(frame: frame)
if CommandLine.arguments.contains("swiftui") {
    let hostingView = NSHostingView(rootView: SwiftUIWindowGlass(variant: variant))
    hostingView.sizingOptions = []
    window.contentView = hostingView
} else if variant == "identity" {
    window.contentView = nativeContent
} else {
    let glass = NSGlassEffectView(frame: frame)
    glass.contentView = nativeContent
    glass.cornerRadius = 0
    glass.style = variant.hasPrefix("clear") ? .clear : .regular
    if variant.hasSuffix("tinted") {
        glass.tintColor = NSColor(srgbRed: 1, green: 0.22, blue: 0.28, alpha: 1)
    }
    window.contentView = glass
}

window.center()
window.makeKeyAndOrderFront(nil)
application.activate(ignoringOtherApps: true)

if CommandLine.arguments.contains("benchmark") {
    DispatchQueue.main.asyncAfter(deadline: .now() + 6) {
        fputs("native window elapsed: 6 s\n", stderr)
        application.terminate(nil)
    }
}

application.run()

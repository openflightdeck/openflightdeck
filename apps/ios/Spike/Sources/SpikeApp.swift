// Spike M1: lokale PMTiles-Karte in MapLibre Native mit Overlay aus dem
// Rust-Kern bei 10 bis 20 Hz. Kein Produktcode, nur Messaufbau.

import SwiftUI

@main
struct SpikeApp: App {
    @State private var engine = SpikeEngine()

    var body: some Scene {
        WindowGroup {
            ZStack(alignment: .topLeading) {
                SpikeMapView(engine: engine)
                    .ignoresSafeArea()
                StatsOverlay(engine: engine)
                    .padding()
            }
            .onAppear { engine.start() }
        }
    }
}

struct StatsOverlay: View {
    @Bindable var engine: SpikeEngine

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text("OFD Spike · Kern \(engine.coreVersion)")
                .font(.headline)
            Picker("Rate", selection: $engine.rateHz) {
                Text("10 Hz").tag(10.0)
                Text("20 Hz").tag(20.0)
                Text("60 Hz").tag(60.0)
            }
            .pickerStyle(.segmented)
            .frame(width: 260)
            Toggle("Track-up", isOn: $engine.trackUp)
                .frame(width: 260)
            Group {
                Text(String(format: "Ticks: %.1f/s  Jitter p95: %.1f ms", engine.stats.tickRate, engine.stats.tickJitterP95Ms))
                Text(String(format: "Rust frameAt: Ø %.3f ms  p95 %.3f ms", engine.stats.ffiMeanMs, engine.stats.ffiP95Ms))
                Text(String(format: "Overlay-Update: Ø %.3f ms  p95 %.3f ms", engine.stats.updateMeanMs, engine.stats.updateP95Ms))
                Text(String(format: "Karte: %.1f fps  GeoJSON %.1f KB", engine.stats.renderFps, engine.stats.geojsonKB))
                Text(String(format: "GS %.0f kt  TRK %03.0f°  %@", engine.groundSpeedKt, engine.trackDeg, engine.alertActive ? "WARNZONE" : "frei"))
                    .foregroundStyle(engine.alertActive ? .red : .primary)
            }
            .font(.system(.footnote, design: .monospaced))
        }
        .padding(12)
        .background(.regularMaterial, in: RoundedRectangle(cornerRadius: 12))
    }
}

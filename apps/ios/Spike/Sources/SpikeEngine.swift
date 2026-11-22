import Foundation
import Observation
import OFDCore
import QuartzCore
import os

/// Taktgeber: ruft den Rust-Kern mit fester Rate auf und misst dabei
/// FFI-Dauer, Overlay-Update, Jitter und Kartenframes.
@Observable
final class SpikeEngine {
    struct Stats {
        var tickRate = 0.0
        var tickJitterP95Ms = 0.0
        var ffiMeanMs = 0.0
        var ffiP95Ms = 0.0
        var updateMeanMs = 0.0
        var updateP95Ms = 0.0
        var renderFps = 0.0
        var geojsonKB = 0.0
    }

    var rateHz = 10.0 { didSet { restartTimer() } }
    var trackUp = true
    private(set) var stats = Stats()
    private(set) var groundSpeedKt = 0.0
    private(set) var trackDeg = 0.0
    private(set) var alertActive = false
    let coreVersion = OFDCore.coreVersion()

    /// Wird pro Tick mit dem neuen Frame aufgerufen (vom MapView gesetzt).
    /// Rückgabe: Dauer des Overlay-Updates in Sekunden.
    @ObservationIgnored var onFrame: ((OverlayFrame, Bool) -> Void)?

    @ObservationIgnored private let flight = SimulatedFlight(
        centerLat: 52.2036, centerLon: 13.1586, radiusM: 5000, groundSpeedKt: 100
    )
    @ObservationIgnored private var link: CADisplayLink?
    @ObservationIgnored private var startTime: CFTimeInterval = 0
    @ObservationIgnored private var lastTick: CFTimeInterval = 0
    @ObservationIgnored private var nextDue: CFTimeInterval = 0
    @ObservationIgnored private var ffiSamples: [Double] = []
    @ObservationIgnored private var updateSamples: [Double] = []
    @ObservationIgnored private var intervalSamples: [Double] = []
    @ObservationIgnored private var renderedFrames = 0
    @ObservationIgnored private var windowStart: CFTimeInterval = 0
    @ObservationIgnored private var lastGeojsonBytes = 0
    @ObservationIgnored private let log = Logger(subsystem: "org.openflightdeck.spike", category: "stats")

    func start() {
        guard link == nil else { return }
        startTime = CACurrentMediaTime()
        windowStart = startTime
        restartTimer()
    }

    func renderedFrame() {
        renderedFrames += 1
    }

    private func restartTimer() {
        link?.invalidate()
        // Display-Link mit der höchsten Bildrate; der Tick selbst wird auf
        // rateHz gedrosselt, damit die Rate exakt einstellbar bleibt.
        let link = CADisplayLink(target: self, selector: #selector(step))
        link.preferredFrameRateRange = CAFrameRateRange(minimum: 30, maximum: 120, preferred: 60)
        link.add(to: .main, forMode: .common)
        self.link = link
        nextDue = 0
    }

    @objc private func step(_ link: CADisplayLink) {
        let now = CACurrentMediaTime()
        let period = 1.0 / rateHz
        if nextDue == 0 { nextDue = now }
        guard now + 0.002 >= nextDue else { return }
        nextDue += period
        if nextDue < now { nextDue = now + period }

        if lastTick > 0 { intervalSamples.append(now - lastTick) }
        lastTick = now

        let t0 = CACurrentMediaTime()
        let frame = flight.frameAt(tS: now - startTime)
        let t1 = CACurrentMediaTime()
        onFrame?(frame, trackUp)
        let t2 = CACurrentMediaTime()

        ffiSamples.append(t1 - t0)
        updateSamples.append(t2 - t1)
        lastGeojsonBytes = frame.geojson.utf8.count
        groundSpeedKt = frame.ownship.groundSpeedKt
        trackDeg = frame.ownship.trackDeg
        alertActive = frame.alertActive

        if now - windowStart >= 2.0 { publishStats(now: now) }
    }

    private func publishStats(now: CFTimeInterval) {
        let window = now - windowStart
        var s = Stats()
        s.tickRate = Double(ffiSamples.count) / window
        s.ffiMeanMs = mean(ffiSamples) * 1000
        s.ffiP95Ms = percentile(ffiSamples, 0.95) * 1000
        s.updateMeanMs = mean(updateSamples) * 1000
        s.updateP95Ms = percentile(updateSamples, 0.95) * 1000
        let expected = 1.0 / rateHz
        s.tickJitterP95Ms = percentile(intervalSamples.map { abs($0 - expected) }, 0.95) * 1000
        s.renderFps = Double(renderedFrames) / window
        s.geojsonKB = Double(lastGeojsonBytes) / 1024
        stats = s
        log.info("rate=\(self.rateHz, privacy: .public)Hz ticks=\(s.tickRate, privacy: .public)/s jitterP95=\(s.tickJitterP95Ms, privacy: .public)ms ffi=\(s.ffiMeanMs, privacy: .public)/\(s.ffiP95Ms, privacy: .public)ms update=\(s.updateMeanMs, privacy: .public)/\(s.updateP95Ms, privacy: .public)ms fps=\(s.renderFps, privacy: .public)")

        ffiSamples.removeAll(keepingCapacity: true)
        updateSamples.removeAll(keepingCapacity: true)
        intervalSamples.removeAll(keepingCapacity: true)
        renderedFrames = 0
        windowStart = now
    }

    private func mean(_ xs: [Double]) -> Double {
        xs.isEmpty ? 0 : xs.reduce(0, +) / Double(xs.count)
    }

    private func percentile(_ xs: [Double], _ p: Double) -> Double {
        guard !xs.isEmpty else { return 0 }
        let sorted = xs.sorted()
        let index = min(sorted.count - 1, Int(Double(sorted.count - 1) * p))
        return sorted[index]
    }
}

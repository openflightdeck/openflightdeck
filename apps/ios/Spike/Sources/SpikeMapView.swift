import MapLibre
import OFDCore
import SwiftUI

/// MapLibre-Karte mit lokaler PMTiles-Datei und dynamischem Overlay aus Rust.
struct SpikeMapView: UIViewRepresentable {
    let engine: SpikeEngine

    func makeCoordinator() -> Coordinator { Coordinator(engine: engine) }

    func makeUIView(context: Context) -> MLNMapView {
        let mapView = MLNMapView(frame: .zero, styleURL: Self.styleURL())
        mapView.delegate = context.coordinator
        mapView.setCenter(CLLocationCoordinate2D(latitude: 52.2036, longitude: 13.1586), zoomLevel: 10, animated: false)
        mapView.logoView.isHidden = false
        context.coordinator.mapView = mapView
        engine.onFrame = { [weak coordinator = context.coordinator] frame, trackUp in
            coordinator?.apply(frame, trackUp: trackUp)
        }
        return mapView
    }

    func updateUIView(_ uiView: MLNMapView, context: Context) {}

    /// Style-JSON mit der gebündelten PMTiles-Datei. MapLibre Native erwartet
    /// die vollständige URL in der Form pmtiles://file:///pfad/datei.pmtiles.
    static func styleURL() -> URL {
        let pmtiles = Bundle.main.url(forResource: "spike", withExtension: "pmtiles", subdirectory: "Resources")
            ?? Bundle.main.url(forResource: "spike", withExtension: "pmtiles")
        let tilesURL = "pmtiles://" + (pmtiles?.absoluteString ?? "file:///missing.pmtiles")
        let style: [String: Any] = [
            "version": 8,
            "name": "OFD Spike",
            "sources": [
                "spike": ["type": "vector", "url": tilesURL],
            ],
            "layers": [
                ["id": "background", "type": "background", "paint": ["background-color": "#f2efe6"]],
                [
                    "id": "grid-minor", "type": "line", "source": "spike", "source-layer": "spike",
                    "filter": ["all", ["==", ["get", "kind"], "grid"], ["!", ["get", "major"]]],
                    "paint": ["line-color": "#c9c3b3", "line-width": 0.6],
                ],
                [
                    "id": "grid-major", "type": "line", "source": "spike", "source-layer": "spike",
                    "filter": ["all", ["==", ["get", "kind"], "grid"], ["get", "major"]],
                    "paint": ["line-color": "#8f8877", "line-width": 1.4],
                ],
                [
                    "id": "rings", "type": "line", "source": "spike", "source-layer": "spike",
                    "filter": ["==", ["get", "kind"], "ring"],
                    "paint": ["line-color": "#2f6db3", "line-width": 1.5, "line-dasharray": [3, 2]],
                ],
                [
                    "id": "nodes", "type": "circle", "source": "spike", "source-layer": "spike",
                    "filter": ["==", ["get", "kind"], "label"],
                    "paint": ["circle-color": "#5a5446", "circle-radius": 3],
                ],
            ],
        ]
        let url = FileManager.default.temporaryDirectory.appendingPathComponent("spike-style.json")
        if let data = try? JSONSerialization.data(withJSONObject: style) {
            try? data.write(to: url)
        }
        return url
    }

    final class Coordinator: NSObject, MLNMapViewDelegate {
        let engine: SpikeEngine
        weak var mapView: MLNMapView?
        private var overlaySource: MLNShapeSource?
        private var ownshipSource: MLNShapeSource?

        init(engine: SpikeEngine) { self.engine = engine }

        func mapView(_ mapView: MLNMapView, didFinishLoading style: MLNStyle) {
            let overlay = MLNShapeSource(identifier: "overlay", shape: nil, options: nil)
            let ownship = MLNShapeSource(identifier: "ownship", shape: nil, options: nil)
            style.addSource(overlay)
            style.addSource(ownship)

            let zone = MLNFillStyleLayer(identifier: "alert-zone", source: overlay)
            zone.predicate = NSPredicate(format: "kind == 'alert-zone'")
            zone.fillColor = NSExpression(forConditional: NSPredicate(format: "active == YES"),
                                          trueExpression: NSExpression(forConstantValue: UIColor.systemRed.withAlphaComponent(0.35)),
                                          falseExpression: NSExpression(forConstantValue: UIColor.systemOrange.withAlphaComponent(0.2)))
            style.addLayer(zone)

            let track = MLNLineStyleLayer(identifier: "track", source: overlay)
            track.predicate = NSPredicate(format: "kind == 'track'")
            track.lineColor = NSExpression(forConstantValue: UIColor.systemPurple)
            track.lineWidth = NSExpression(forConstantValue: 3)
            style.addLayer(track)

            let plane = MLNCircleStyleLayer(identifier: "ownship", source: ownship)
            plane.circleColor = NSExpression(forConstantValue: UIColor.black)
            plane.circleRadius = NSExpression(forConstantValue: 8)
            plane.circleStrokeColor = NSExpression(forConstantValue: UIColor.white)
            plane.circleStrokeWidth = NSExpression(forConstantValue: 3)
            style.addLayer(plane)

            overlaySource = overlay
            ownshipSource = ownship
        }

        func mapViewDidFinishRenderingFrame(_ mapView: MLNMapView, fullyRendered: Bool) {
            engine.renderedFrame()
        }

        func apply(_ frame: OverlayFrame, trackUp: Bool) {
            guard let mapView, let overlaySource, let ownshipSource else { return }
            if let data = frame.geojson.data(using: .utf8),
               let shape = try? MLNShape(data: data, encoding: String.Encoding.utf8.rawValue) {
                overlaySource.shape = shape
            }
            let coordinate = CLLocationCoordinate2D(latitude: frame.ownship.lat, longitude: frame.ownship.lon)
            let point = MLNPointFeature()
            point.coordinate = coordinate
            ownshipSource.shape = point

            let camera = mapView.camera
            camera.centerCoordinate = coordinate
            camera.heading = trackUp ? frame.ownship.trackDeg : 0
            mapView.setCamera(camera, animated: false)
        }
    }
}

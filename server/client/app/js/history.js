import QuakeMap from "./common.js";

class HistoryMap extends QuakeMap {

    constructor(mapId, sidebarId, listId) {
        super(mapId, sidebarId);

        this.list = document.getElementById(listId);

        this.map.spin(true);

        this.initialized = false;
    }

    static radius(magnitude, depth) {
        let size = Math.ceil(Math.exp(magnitude) / depth);
        if(size < 5) size = 5;
        return size
    }

    static historyMarkers(json) {
        return L.timeline(json, {
            pointToLayer: function(feature, latlng) {
                if (feature.properties && feature.properties.magnitude && feature.properties.depth) {
                    return HistoryMap.quakeMarker(latlng, feature.properties.magnitude, feature.properties.depth);
                }
            },
            onEachFeature: HistoryMap.markerPopup
        });
    };

    static updateList(layer, map, list) {

        const displayed = layer.getLayers().sort(function(a,b) {
            const first = moment(a.feature.properties.datetime);
            const second = moment(b.feature.properties.datetime);
            return first - second;
        });
        list.innerHTML = "";

        displayed.forEach(function(quake){
            const layerId = layer.getLayerId(quake);
            const props = quake.feature.properties;
            const inBounds = map.getBounds().contains({lat: props.latitude, lng: props.longitude});
            if( inBounds ) {
                const newItem = document.createElement('li');
                newItem.className = "quake-show";
                newItem.setAttribute("data-layer-id", layerId);
                newItem.innerHTML = HistoryMap.quakeListItemHtml(props);

                newItem.onclick = function(e) {
                    map.flyTo(quake.getLatLng(), 14);
                    map.once('moveend', function() {
                        quake.openPopup();
                    });
                };

                list.prepend(newItem);
            }
        });
    }

    load(json) {

        let timelineControl = L.timelineSliderControl({
            formatOutput: function(date){
                return moment(date).format("YYYY-MM-DD HH:MM:SS");
            },
            steps: 4000,
            duration: 80000,
            position: "bottomright"
        });

        let timeline = HistoryMap.historyMarkers(json);

        const map = this.map;
        const list = this.list;

        timeline.on('change', function(e){
            HistoryMap.updateList(e.target, map, list);
        });

        timelineControl.addTo(map);
        timelineControl.addTimelines(timeline);

        timeline.addTo(map);

        this.layer = timeline;
        this.control = timelineControl;

        map.spin(false);
        this.initialized = true;
    }

    clear() {
        if(this.initialized) {
            this.control.removeTimelines();
            this.map.removeControl(this.control);
            this.control = {};
            this.layer.clearLayers();
            this.list.innerHTML = "";
        }
    }
}

export default HistoryMap;

import QuakeMap from "./common.js";

class CurrentMap extends QuakeMap {

    #initialized = false;
    #layer;

    constructor() {
        super();

        this.map.spin(true);
    }

    static markerIcon(size, text) {
        const style = "width: " + size + "px; height: " + size + "px; line-height: " + size + "px;";
        const html = '<div class="quakes-marker-icon" style="'+ style +'"><b>' + text + '</b></div>';
        return L.divIcon({ html: html, className: 'quakes-cluster', iconSize: L.point(size, size) });
    }

    static currentMarkers(json) {
        return L.geoJSON(json, {
            pointToLayer: function(feature, latlng) {
                if (feature.properties) {
                    return new L.marker(latlng, {
                        icon: CurrentMap.markerIcon(20,1),
                        zIndexOffset: 1000
                    });
                }
            },
            onEachFeature: CurrentMap.markerPopup
        });
    }

    static filterOld(json) {
        const horizon = moment().utc().subtract(24, 'hours');
        const filtered = json.features.filter(function(item) {
            return moment.utc(item.properties.datetime).isAfter(horizon);
        });
        return {
            type: "FeatureCollection",
            features: filtered
        };
    }

    static updateList(layer, map) {

        const displayed = layer.getLayers().sort(function(a,b) {
            const first = moment(a.feature.properties.datetime);
            const second = moment(b.feature.properties.datetime);
            return second - first;
        });
        const list = document.getElementById('displayed-list');
        displayed.forEach(function(quake){
            const layerId = layer.getLayerId(quake);
            const props = quake.feature.properties;
            const newItem = document.createElement('li');
            newItem.setAttribute("data-layer-id", layerId);
            newItem.innerHTML = CurrentMap.quakeListItemHtml(props);

            newItem.onclick = function(e) {
                $('.list-view > li').removeClass('quake-selected');
                $(this).addClass('quake-selected');
                map.flyTo(quake.getLatLng(), 14);
                map.once('moveend', function() {
                    let parent = quake.__parent;
                    if(parent) {
                        let group = parent._group;
                        if(group && typeof group.zoomToShowLayer == 'function') {
                            console.log("Got group!");
                            group.zoomToShowLayer(quake, function() {
                                quake.openPopup();
                            });
                        } else {
                            quake.openPopup();
                        }
                    } else {
                        quake.openPopup();
                    }
                });
            };

            list.prepend(newItem);

            setTimeout(function() {
                newItem.className = newItem.className + "quake-show";
            }, 10);

        });
    }

    add(json) {
        let markers = CurrentMap.currentMarkers(json);

        if(!this.#initialized) {
            let cluster = L.markerClusterGroup({
                iconCreateFunction: function(cluster) {
                    const size = Math.log(cluster.getChildCount())*40;
                    return CurrentMap.markerIcon(size, cluster.getChildCount());
                }
            });
            cluster.addLayer(markers);
            this.map.addLayer(cluster);

            CurrentMap.updateList(cluster, this.map);

            this.layer = cluster;
            this.map.spin(false);
            this.#initialized = true;
        } else {
            this.layer.addLayers(markers);
            this.layer.refreshClusters(markers);
            CurrentMap.updateList(markers, this.map);
        }
    }
}

export default CurrentMap;



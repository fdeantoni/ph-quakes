"use strict";

import QuakeMap from "./common.js";

class CurrentMap extends QuakeMap {

    #initialized = false;
    #list;
    #layer;

    constructor(mapId, sidebarId, listId) {
        super(mapId, sidebarId);

        this.#list = document.getElementById(listId);
    }

    static currentMarkers(json) {
        return L.geoJSON(json, {
            pointToLayer: function(feature, latlng) {
                if (feature.properties) {
                    return CurrentMap.quakeMarker(latlng, feature.properties.magnitude, feature.properties.depth);
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

    static updateList(layer, map, list) {

        const displayed = layer.getLayers().sort(function(a,b) {
            const first = moment(a.feature.properties.datetime);
            const second = moment(b.feature.properties.datetime);
            return first - second;
        });
        displayed.forEach(function(quake){
            const layerId = layer.getLayerId(quake);
            const props = quake.feature.properties;
            const newItem = document.createElement('li');
            newItem.setAttribute("data-layer-id", layerId);
            newItem.innerHTML = CurrentMap.quakeListItemHtml(props);

            newItem.onclick = function(e) {
                map.flyTo(quake.getLatLng(), 10);
                map.once('moveend', function() {
                    quake.openPopup();
                });
            };

            list.prepend(newItem);

            setTimeout(function() {
                newItem.className = newItem.className + "quake-show";
            }, 10);

        });
    }

    static clusterIcon(size, text) {
        const style = "width: " + size + "px; height: " + size + "px; line-height: " + size + "px;";
        const html = '<div class="quakes-marker-icon" style="'+ style +'"><b>' + text + '</b></div>';
        return L.divIcon({ html: html, className: 'quakes-cluster', iconSize: L.point(size, size) });
    }

    add(json) {
        let latest = CurrentMap.filterOld(json);
        let markers = CurrentMap.currentMarkers(latest);

        if(!this.#initialized) {
            let cluster = L.markerClusterGroup({
                maxClusterRadius: function (zoom) {
                    return (zoom <= 6) ? 80 : 1; // radius in pixels
                },
                iconCreateFunction: function(cluster) {
                    const size = Math.log(cluster.getChildCount())*40;
                    return CurrentMap.clusterIcon(size, cluster.getChildCount());
                }
            });
            cluster.addLayer(markers);
            this.map.addLayer(cluster);

            CurrentMap.updateList(cluster, this.map, this.#list);

            this.#layer = cluster;
            this.#initialized = true;
        } else {
            this.#layer.addLayers(markers);
            this.#layer.refreshClusters(markers);

            CurrentMap.updateList(markers, this.map, this.#list);
        }
    }

    clear() {
        this.#layer.clearLayers();
        this.#list.innerHTML = "";
    }
}

export default CurrentMap;



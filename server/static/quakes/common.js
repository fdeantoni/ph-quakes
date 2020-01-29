"use strict";

class QuakeMap {

    map;

    constructor(mapId, sidebarId) {
        this.map = L.map(mapId, {
            center: [15.5, 120.91],
            zoom: 7,
            maxZoom: 18
        });

        const mapboxUrl = 'https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}';

        const mapboxConfig = {
            attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
            id: 'mapbox/outdoors-v11',
            accessToken: 'pk.eyJ1IjoiZmRlYW50b25pIiwiYSI6ImNrNWhhOHlueTAxcHAzZHA3Nnd1MDhveWkifQ.kTW32UkDDmHFl9MGhnNrbw',
            tileSize: 512,
            zoomOffset: -1
        };

        L.tileLayer(mapboxUrl, mapboxConfig).addTo(this.map);

        L.control.sidebar(sidebarId).addTo(this.map);
    }

    static radius(magnitude, depth) {
        let size = Math.ceil(Math.exp(magnitude) / depth);
        if(size < 5) size = 5;
        return size
    }

    static quakeMarker(latlng, magnitude, depth) {
        return new L.circleMarker(latlng, {
            className: "fade-in",
            radius: QuakeMap.radius(magnitude, depth),
            fillColor: "#ff3b33",
            color: "#ff3b33",
            weight: 1,
            fillOpacity: 0.6
        });
    }

    static markerPopup(feature, layer) {
        if (feature.properties) {
            const props = feature.properties;
            const header = '<h3>' + props.province + ' ' + props.longitude + ', ' + props.latitude + '</h3>';
            const details = '<ul style="list-style-type:none;padding-left: 0;">' +
                '<li><b>Magnitude: </b>' + props.magnitude + '</li>' +
                '<li><b>Depth:     </b>' + props.depth + '</li>' +
                '<li><b>Location:  </b>' + props.location + '</li>' +
                '<li><b>Timestamp:  </b>' + props.datetime + '</li>' +
                '<li><b>Source:    </b><a href="'+ props.url +'" target="_blank">phivolcs</li>' +
                '</ul>';
            layer.bindPopup(header + details);
        }
    }

    static quakeListItemHtml(props) {
        return '<div class="quake-container">' +
            '<span class="quake-magnitude">' + props.magnitude + '</span>' +
            '<h1 class="quake-location">' + props.province + '</h1>' +
            '<h2 class="quake-timestamp">' + moment(props.start).tz('Asia/Manila').format() + '</h2>' +
            '<aside class="quake-aside">' + props.depth + ' km</aside>' +
            '</div>';
    }

    get leafletMap() {
        return this.map;
    }
}

export default QuakeMap;
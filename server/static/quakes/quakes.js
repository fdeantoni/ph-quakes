(function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = void 0;

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } }

function _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); return Constructor; }

var QuakeMap =
/*#__PURE__*/
function () {
  function QuakeMap(mapId, sidebarId) {
    _classCallCheck(this, QuakeMap);

    this.map = L.map(mapId, {
      center: [15.5, 120.91],
      zoom: 7,
      maxZoom: 18
    });
    var mapboxUrl = 'https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}';
    var mapboxConfig = {
      attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
      id: 'mapbox/outdoors-v11',
      accessToken: 'pk.eyJ1IjoiZmRlYW50b25pIiwiYSI6ImNrNWhhOHlueTAxcHAzZHA3Nnd1MDhveWkifQ.kTW32UkDDmHFl9MGhnNrbw',
      tileSize: 512,
      zoomOffset: -1
    };
    L.tileLayer(mapboxUrl, mapboxConfig).addTo(this.map);
    L.control.sidebar(sidebarId).addTo(this.map);
  }

  _createClass(QuakeMap, [{
    key: "leafletMap",
    get: function get() {
      return this.map;
    }
  }], [{
    key: "radius",
    value: function radius(magnitude, depth) {
      var size = Math.ceil(Math.exp(magnitude) / depth);
      if (size < 5) size = 5;
      return size;
    }
  }, {
    key: "quakeMarker",
    value: function quakeMarker(latlng, magnitude, depth) {
      return new L.circleMarker(latlng, {
        className: "fade-in",
        radius: QuakeMap.radius(magnitude, depth),
        fillColor: "#ff3b33",
        color: "#ff3b33",
        weight: 1,
        fillOpacity: 0.6
      });
    }
  }, {
    key: "markerPopup",
    value: function markerPopup(feature, layer) {
      if (feature.properties) {
        var props = feature.properties;
        var header = '<h3>' + props.province + ' ' + props.longitude + ', ' + props.latitude + '</h3>';
        var details = '<ul style="list-style-type:none;padding-left: 0;">' + '<li><b>Magnitude: </b>' + props.magnitude + '</li>' + '<li><b>Depth:     </b>' + props.depth + '</li>' + '<li><b>Location:  </b>' + props.location + '</li>' + '<li><b>Timestamp:  </b>' + props.datetime + '</li>' + '<li><b>Source:    </b><a href="' + props.url + '" target="_blank">phivolcs</li>' + '</ul>';
        layer.bindPopup(header + details);
      }
    }
  }, {
    key: "quakeListItemHtml",
    value: function quakeListItemHtml(props) {
      return '<div class="quake-container">' + '<span class="quake-magnitude">' + props.magnitude + '</span>' + '<h1 class="quake-location">' + props.province + '</h1>' + '<h2 class="quake-timestamp">' + moment(props.start).tz('Asia/Manila').format() + '</h2>' + '<aside class="quake-aside">' + props.depth + ' km</aside>' + '</div>';
    }
  }]);

  return QuakeMap;
}();

var _default = QuakeMap;
exports["default"] = _default;

},{}],2:[function(require,module,exports){
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = void 0;

var _common = _interopRequireDefault(require("./common.js"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }

function _typeof(obj) { "@babel/helpers - typeof"; if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj; }; } return _typeof(obj); }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } }

function _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); return Constructor; }

function _possibleConstructorReturn(self, call) { if (call && (_typeof(call) === "object" || typeof call === "function")) { return call; } return _assertThisInitialized(self); }

function _assertThisInitialized(self) { if (self === void 0) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return self; }

function _getPrototypeOf(o) { _getPrototypeOf = Object.setPrototypeOf ? Object.getPrototypeOf : function _getPrototypeOf(o) { return o.__proto__ || Object.getPrototypeOf(o); }; return _getPrototypeOf(o); }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function"); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, writable: true, configurable: true } }); if (superClass) _setPrototypeOf(subClass, superClass); }

function _setPrototypeOf(o, p) { _setPrototypeOf = Object.setPrototypeOf || function _setPrototypeOf(o, p) { o.__proto__ = p; return o; }; return _setPrototypeOf(o, p); }

var CurrentMap =
/*#__PURE__*/
function (_QuakeMap) {
  _inherits(CurrentMap, _QuakeMap);

  function CurrentMap(mapId, sidebarId, listId) {
    var _this;

    _classCallCheck(this, CurrentMap);

    _this = _possibleConstructorReturn(this, _getPrototypeOf(CurrentMap).call(this, mapId, sidebarId));
    _this.list = document.getElementById(listId);
    _this.initialized = false;
    return _this;
  }

  _createClass(CurrentMap, [{
    key: "add",
    value: function add(json) {
      var latest = CurrentMap.filterOld(json);
      var markers = CurrentMap.currentMarkers(latest);

      if (!this.initialized) {
        var cluster = L.markerClusterGroup({
          maxClusterRadius: function maxClusterRadius(zoom) {
            return zoom <= 6 ? 80 : 1; // radius in pixels
          },
          iconCreateFunction: function iconCreateFunction(cluster) {
            var size = Math.log(cluster.getChildCount()) * 40;
            return CurrentMap.clusterIcon(size, cluster.getChildCount());
          }
        });
        cluster.addLayer(markers);
        this.map.addLayer(cluster);
        CurrentMap.updateList(cluster, this.map, this.list);
        this.layer = cluster;
        this.initialized = true;
      } else {
        this.layer.addLayers(markers);
        this.layer.refreshClusters(markers);
        CurrentMap.updateList(markers, this.map, this.list);
      }
    }
  }, {
    key: "clear",
    value: function clear() {
      this.layer.clearLayers();
      this.list.innerHTML = "";
    }
  }], [{
    key: "currentMarkers",
    value: function currentMarkers(json) {
      return L.geoJSON(json, {
        pointToLayer: function pointToLayer(feature, latlng) {
          if (feature.properties) {
            return CurrentMap.quakeMarker(latlng, feature.properties.magnitude, feature.properties.depth);
          }
        },
        onEachFeature: CurrentMap.markerPopup
      });
    }
  }, {
    key: "filterOld",
    value: function filterOld(json) {
      var horizon = moment().utc().subtract(24, 'hours');
      var filtered = json.features.filter(function (item) {
        return moment.utc(item.properties.datetime).isAfter(horizon);
      });
      return {
        type: "FeatureCollection",
        features: filtered
      };
    }
  }, {
    key: "updateList",
    value: function updateList(layer, map, list) {
      var displayed = layer.getLayers().sort(function (a, b) {
        var first = moment(a.feature.properties.datetime);
        var second = moment(b.feature.properties.datetime);
        return first - second;
      });
      displayed.forEach(function (quake) {
        var layerId = layer.getLayerId(quake);
        var props = quake.feature.properties;
        var newItem = document.createElement('li');
        newItem.setAttribute("data-layer-id", layerId);
        newItem.innerHTML = CurrentMap.quakeListItemHtml(props);

        newItem.onclick = function (e) {
          map.flyTo(quake.getLatLng(), 10);
          map.once('moveend', function () {
            quake.openPopup();
          });
        };

        list.prepend(newItem);
        setTimeout(function () {
          newItem.className = newItem.className + "quake-show";
        }, 10);
      });
    }
  }, {
    key: "clusterIcon",
    value: function clusterIcon(size, text) {
      var style = "width: " + size + "px; height: " + size + "px; line-height: " + size + "px;";
      var html = '<div class="quakes-marker-icon" style="' + style + '"><b>' + text + '</b></div>';
      return L.divIcon({
        html: html,
        className: 'quakes-cluster',
        iconSize: L.point(size, size)
      });
    }
  }]);

  return CurrentMap;
}(_common["default"]);

var _default = CurrentMap;
exports["default"] = _default;

},{"./common.js":1}],3:[function(require,module,exports){
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = void 0;

var _common = _interopRequireDefault(require("./common.js"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }

function _typeof(obj) { "@babel/helpers - typeof"; if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj; }; } return _typeof(obj); }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } }

function _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); return Constructor; }

function _possibleConstructorReturn(self, call) { if (call && (_typeof(call) === "object" || typeof call === "function")) { return call; } return _assertThisInitialized(self); }

function _assertThisInitialized(self) { if (self === void 0) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return self; }

function _getPrototypeOf(o) { _getPrototypeOf = Object.setPrototypeOf ? Object.getPrototypeOf : function _getPrototypeOf(o) { return o.__proto__ || Object.getPrototypeOf(o); }; return _getPrototypeOf(o); }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function"); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, writable: true, configurable: true } }); if (superClass) _setPrototypeOf(subClass, superClass); }

function _setPrototypeOf(o, p) { _setPrototypeOf = Object.setPrototypeOf || function _setPrototypeOf(o, p) { o.__proto__ = p; return o; }; return _setPrototypeOf(o, p); }

var HistoryMap =
/*#__PURE__*/
function (_QuakeMap) {
  _inherits(HistoryMap, _QuakeMap);

  function HistoryMap(mapId, sidebarId, listId) {
    var _this;

    _classCallCheck(this, HistoryMap);

    _this = _possibleConstructorReturn(this, _getPrototypeOf(HistoryMap).call(this, mapId, sidebarId));
    _this.list = document.getElementById(listId);

    _this.map.spin(true);

    _this.initialized = false;
    return _this;
  }

  _createClass(HistoryMap, [{
    key: "load",
    value: function load(json) {
      var timelineControl = L.timelineSliderControl({
        formatOutput: function formatOutput(date) {
          return moment(date).format("YYYY-MM-DD HH:MM:SS");
        },
        steps: 4000,
        duration: 80000,
        position: "bottomright"
      });
      var timeline = HistoryMap.historyMarkers(json);
      var map = this.map;
      var list = this.list;
      timeline.on('change', function (e) {
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
  }, {
    key: "clear",
    value: function clear() {
      if (this.initialized) {
        this.control.removeTimelines();
        this.map.removeControl(this.control);
        this.control = {};
        this.layer.clearLayers();
        this.list.innerHTML = "";
      }
    }
  }], [{
    key: "radius",
    value: function radius(magnitude, depth) {
      var size = Math.ceil(Math.exp(magnitude) / depth);
      if (size < 5) size = 5;
      return size;
    }
  }, {
    key: "historyMarkers",
    value: function historyMarkers(json) {
      return L.timeline(json, {
        pointToLayer: function pointToLayer(feature, latlng) {
          if (feature.properties && feature.properties.magnitude && feature.properties.depth) {
            return HistoryMap.quakeMarker(latlng, feature.properties.magnitude, feature.properties.depth);
          }
        },
        onEachFeature: HistoryMap.markerPopup
      });
    }
  }, {
    key: "updateList",
    value: function updateList(layer, map, list) {
      var displayed = layer.getLayers().sort(function (a, b) {
        var first = moment(a.feature.properties.datetime);
        var second = moment(b.feature.properties.datetime);
        return first - second;
      });
      list.innerHTML = "";
      displayed.forEach(function (quake) {
        var layerId = layer.getLayerId(quake);
        var props = quake.feature.properties;
        var inBounds = map.getBounds().contains({
          lat: props.latitude,
          lng: props.longitude
        });

        if (inBounds) {
          var newItem = document.createElement('li');
          newItem.className = "quake-show";
          newItem.setAttribute("data-layer-id", layerId);
          newItem.innerHTML = HistoryMap.quakeListItemHtml(props);

          newItem.onclick = function (e) {
            map.flyTo(quake.getLatLng(), 14);
            map.once('moveend', function () {
              quake.openPopup();
            });
          };

          list.prepend(newItem);
        }
      });
    }
  }]);

  return HistoryMap;
}(_common["default"]);

var _default = HistoryMap;
exports["default"] = _default;

},{"./common.js":1}],4:[function(require,module,exports){
"use strict";

var _current = _interopRequireDefault(require("./current"));

var _history = _interopRequireDefault(require("./history"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }

var current = new _current["default"]("current-map", "sidebar", "current-list");
var history = new _history["default"]("history-map", "sidebar", "history-list");
var reconnected = false;
var geojson = {
  features: [],
  type: "FeatureCollection"
};

function add(json) {
  geojson.features = geojson.features.concat(json.features);
  current.add(json);
}

function display(show, hide) {
  document.getElementById(show + "-map").classList.remove("hide");
  document.getElementById(show + "-list").classList.remove("hide");
  document.getElementById(hide + "-map").classList.add("hide");
  document.getElementById(hide + "-list").classList.add("hide");
  var header = document.getElementById("list-header");

  if (show === "history") {
    header.innerHTML = header.innerHTML.replace("24h", "History");
    history.clear();

    history.leafletMap._onResize();

    history.load(geojson);
  } else {
    header.innerHTML = header.innerHTML.replace("History", "24h");
  }
}

document.getElementById("history-toggle").onclick = function () {
  this.classList.add("hide");
  display("history", "current");
  document.getElementById("current-toggle").classList.remove("hide");
};

document.getElementById("current-toggle").onclick = function () {
  this.classList.add("hide");
  display("current", "history");
  document.getElementById("history-toggle").classList.remove("hide");
};

function connect() {
  var wsUri = (window.location.protocol === 'https:' && 'wss://' || 'ws://') + window.location.host + '/ws/';
  var ws = new WebSocket(wsUri);

  ws.onopen = function () {// TODO: set config here.
  };

  ws.onmessage = function (event) {
    var json = JSON.parse(event.data);
    console.log('New quakes ', json);
    if (reconnected) current.clear();
    add(json);
    reconnected = false;
  };

  ws.onclose = function (e) {
    console.log('Socket is closed. Reconnect will be attempted in 10 seconds.', e.reason);
    setTimeout(function () {
      reconnected = true;
      connect();
    }, 10000);
  };

  ws.onerror = function (err) {
    console.error('Socket encountered error: ', err.message, 'Closing socket');
    ws.close();
  };
}

connect(); // let counter = 0;
//
// function createDummy() {
//     return { "features":[
//             {
//                 "geometry":{"coordinates":[120.93-counter,13.77],"type":"Point"},
//                 "properties":{"datetime":moment().utc().toISOString(),"depth":134,"end":moment().utc().add(8, 'hours').toISOString(),"latitude":13.77,"location":"Some location","longitude":120.93-counter,"magnitude":2.4,"province":"Some province " + counter,"start":moment().utc().toISOString(),"url":"http://example.com"},"type":"Feature"}
//         ],
//         "type":"FeatureCollection"
//     };
// }
//
// document.getElementById('add-to-list').onclick = function() {
//     add(createDummy());
//     counter = counter + 1;
// };

},{"./current":2,"./history":3}]},{},[4]);

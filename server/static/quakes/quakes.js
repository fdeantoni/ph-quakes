!function i(a,u,s){function l(t,e){if(!u[t]){if(!a[t]){var n="function"==typeof require&&require;if(!e&&n)return n(t,!0);if(c)return c(t,!0);var r=new Error("Cannot find module '"+t+"'");throw r.code="MODULE_NOT_FOUND",r}var o=u[t]={exports:{}};a[t][0].call(o.exports,function(e){return l(a[t][1][e]||e)},o,o.exports,i,a,u,s)}return u[t].exports}for(var c="function"==typeof require&&require,e=0;e<s.length;e++)l(s[e]);return l}({1:[function(e,t,n){"use strict";function o(e,t){for(var n=0;n<t.length;n++){var r=t[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(e,r.key,r)}}Object.defineProperty(n,"__esModule",{value:!0}),n.default=void 0;var r=function(){function r(e,t){!function(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")}(this,r),this.map=L.map(e,{center:[15.5,120.91],zoom:7,maxZoom:18,zoomControl:!1});L.tileLayer("https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}",{attribution:'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',id:"mapbox/outdoors-v11",accessToken:"pk.eyJ1IjoiZmRlYW50b25pIiwiYSI6ImNrNWhhOHlueTAxcHAzZHA3Nnd1MDhveWkifQ.kTW32UkDDmHFl9MGhnNrbw",tileSize:512,zoomOffset:-1}).addTo(this.map),L.control.sidebar(t).addTo(this.map),L.control.zoom({position:"topright"}).addTo(this.map)}var e,t,n;return e=r,n=[{key:"radius",value:function(e,t){var n=Math.ceil(Math.exp(e)/t);return n<5&&(n=5),t<2&&(8<e?n=140:7<e?n=120:6<e?n=100:5<e&&(n=80)),n}},{key:"quakeMarker",value:function(e,t,n){return new L.circleMarker(e,{className:"marker-fade-in",radius:r.radius(t,n),fillColor:"#ff3b33",color:"#ff3b33",weight:1,fillOpacity:.6})}},{key:"markerPopup",value:function(e,t){if(e.properties){var n=e.properties,r="<h3>"+n.province+" "+n.longitude+", "+n.latitude+"</h3>",o='<ul style="list-style-type:none;padding-left: 0;"><li><b>Magnitude: </b>'+n.magnitude+"</li><li><b>Depth:     </b>"+n.depth+"</li><li><b>Location:  </b>"+n.location+"</li><li><b>Timestamp:  </b>"+n.datetime+'</li><li><b>Source:    </b><a href="'+n.url+'" target="_blank">phivolcs</li></ul>';t.bindPopup(r+o)}}},{key:"quakeListItemHtml",value:function(e){return'<div class="quake-container"><span class="quake-magnitude">'+e.magnitude+'</span><h1 class="quake-location">'+e.province+'</h1><h2 class="quake-timestamp">'+moment(e.start).tz("Asia/Manila").format()+'</h2><aside class="quake-aside">'+e.depth+" km</aside></div>"}}],(t=[{key:"leafletMap",get:function(){return this.map}}])&&o(e.prototype,t),n&&o(e,n),r}();n.default=r},{}],2:[function(e,t,n){"use strict";Object.defineProperty(n,"__esModule",{value:!0}),n.default=void 0;var r,o=(r=e("./common.js"))&&r.__esModule?r:{default:r};function i(e){return(i="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e})(e)}function a(e,t){for(var n=0;n<t.length;n++){var r=t[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(e,r.key,r)}}function s(e,t){return!t||"object"!==i(t)&&"function"!=typeof t?function(e){if(void 0!==e)return e;throw new ReferenceError("this hasn't been initialised - super() hasn't been called")}(e):t}function l(e){return(l=Object.setPrototypeOf?Object.getPrototypeOf:function(e){return e.__proto__||Object.getPrototypeOf(e)})(e)}function c(e,t){return(c=Object.setPrototypeOf||function(e,t){return e.__proto__=t,e})(e,t)}var u=function(){function u(e,t,n){var r;return function(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")}(this,u),(r=s(this,l(u).call(this,e,t))).list=document.getElementById(n),r.map.spin(!0),r.initialized=!1,r}var e,t,n;return function(e,t){if("function"!=typeof t&&null!==t)throw new TypeError("Super expression must either be null or a function");e.prototype=Object.create(t&&t.prototype,{constructor:{value:e,writable:!0,configurable:!0}}),t&&c(e,t)}(u,o["default"]),e=u,n=[{key:"currentMarkers",value:function(e){return L.geoJSON(e,{pointToLayer:function(e,t){if(e.properties)return u.quakeMarker(t,e.properties.magnitude,e.properties.depth)},onEachFeature:u.markerPopup})}},{key:"filterOld",value:function(e){var t=moment().utc().subtract(24,"hours");return{type:"FeatureCollection",features:e.features.filter(function(e){return moment.utc(e.properties.datetime).isAfter(t)})}}},{key:"updateList",value:function(o,i,a){o.getLayers().sort(function(e,t){return moment(e.feature.properties.datetime)-moment(t.feature.properties.datetime)}).forEach(function(t){var e=o.getLayerId(t),n=t.feature.properties,r=document.createElement("li");r.setAttribute("data-layer-id",e),r.innerHTML=u.quakeListItemHtml(n),r.onclick=function(e){i.flyTo(t.getLatLng(),10),i.once("moveend",function(){t.openPopup()})},a.prepend(r),setTimeout(function(){r.className=r.className+"quake-show"},50)})}},{key:"clusterIcon",value:function(e,t){var n='<div class="quakes-marker-icon" style="'+("width: "+e+"px; height: "+e+"px; line-height: "+e+"px;")+'"><b>'+t+"</b></div>";return L.divIcon({html:n,className:"quakes-cluster",iconSize:L.point(e,e)})}}],(t=[{key:"add",value:function(e){var t=u.filterOld(e),n=u.currentMarkers(t);if(this.initialized)this.layer.addLayers(n),this.layer.refreshClusters(n),u.updateList(n,this.map,this.list);else{var r=L.markerClusterGroup({maxClusterRadius:function(e){return e<=6?80:1},iconCreateFunction:function(e){var t=40*Math.log(e.getChildCount());return u.clusterIcon(t,e.getChildCount())}});r.addLayer(n),this.map.addLayer(r),u.updateList(r,this.map,this.list),this.layer=r,this.map.spin(!1),this.initialized=!0}}},{key:"clear",value:function(){this.layer.clearLayers(),this.list.innerHTML=""}}])&&a(e.prototype,t),n&&a(e,n),u}();n.default=u},{"./common.js":1}],3:[function(e,t,n){"use strict";Object.defineProperty(n,"__esModule",{value:!0}),n.default=void 0;var r,o=(r=e("./common.js"))&&r.__esModule?r:{default:r};function i(e){return(i="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e})(e)}function a(e,t){for(var n=0;n<t.length;n++){var r=t[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(e,r.key,r)}}function s(e,t){return!t||"object"!==i(t)&&"function"!=typeof t?function(e){if(void 0!==e)return e;throw new ReferenceError("this hasn't been initialised - super() hasn't been called")}(e):t}function l(e){return(l=Object.setPrototypeOf?Object.getPrototypeOf:function(e){return e.__proto__||Object.getPrototypeOf(e)})(e)}function c(e,t){return(c=Object.setPrototypeOf||function(e,t){return e.__proto__=t,e})(e,t)}var u=function(){function u(e,t,n){var r;return function(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")}(this,u),(r=s(this,l(u).call(this,e,t))).list=document.getElementById(n),r.map.spin(!0),r.initialized=!1,r}var e,t,n;return function(e,t){if("function"!=typeof t&&null!==t)throw new TypeError("Super expression must either be null or a function");e.prototype=Object.create(t&&t.prototype,{constructor:{value:e,writable:!0,configurable:!0}}),t&&c(e,t)}(u,o["default"]),e=u,n=[{key:"radius",value:function(e,t){var n=Math.ceil(Math.exp(e)/t);return n<5&&(n=5),n}},{key:"historyMarkers",value:function(e){return L.timeline(e,{pointToLayer:function(e,t){if(e.properties&&e.properties.magnitude&&e.properties.depth)return u.quakeMarker(t,e.properties.magnitude,e.properties.depth)},onEachFeature:u.markerPopup})}},{key:"updateList",value:function(o,i,a){var e=o.getLayers().sort(function(e,t){return moment(e.feature.properties.datetime)-moment(t.feature.properties.datetime)});a.innerHTML="",e.forEach(function(t){var e=o.getLayerId(t),n=t.feature.properties;if(i.getBounds().contains({lat:n.latitude,lng:n.longitude})){var r=document.createElement("li");r.className="quake-show",r.setAttribute("data-layer-id",e),r.innerHTML=u.quakeListItemHtml(n),r.onclick=function(e){i.flyTo(t.getLatLng(),14),i.once("moveend",function(){t.openPopup()})},a.prepend(r)}})}}],(t=[{key:"load",value:function(e){var t=L.timelineSliderControl({formatOutput:function(e){return moment(e).format("YYYY-MM-DD HH:MM:SS")},steps:4e3,duration:8e4,position:"bottomright"}),n=u.historyMarkers(e),r=this.map,o=this.list;n.on("change",function(e){u.updateList(e.target,r,o)}),r.on("moveend",function(e){console.log("Event: ",e),u.updateList(n,r,o)}),t.addTo(r),t.addTimelines(n),n.addTo(r),this.layer=n,this.control=t,this.map.spin(!1),this.initialized=!0}},{key:"clear",value:function(){this.initialized&&(this.control.removeTimelines(),this.map.removeControl(this.control),this.control={},this.layer.clearLayers(),this.list.innerHTML="")}}])&&a(e.prototype,t),n&&a(e,n),u}();n.default=u},{"./common.js":1}],4:[function(e,t,n){"use strict";var r=i(e("./current")),o=i(e("./history"));function i(e){return e&&e.__esModule?e:{default:e}}var a=new r.default("current-map","sidebar","current-list"),u=new o.default("history-map","sidebar","history-list"),s=!1,l={features:[],type:"FeatureCollection"},c=document.getElementsByClassName("quake-alert")[0],f=document.getElementById("quake-tweet");function p(e){var t,n=e.properties.source.split("/").pop();t=n,f.innerHTML="",c.style.display="none",twttr.widgets.createTweet(t,f,{conversation:"none"}),setTimeout(function(){c.style.display="inline",c.classList.add("quake-alert-show"),setTimeout(function(){c.classList.remove("quake-alert-show"),setTimeout(function(){c.style.display="none"},1e3)},4e3)},1e3)}function d(e,t){document.getElementById(e+"-map").classList.remove("hide"),document.getElementById(e+"-list").classList.remove("hide"),document.getElementById(t+"-map").classList.add("hide"),document.getElementById(t+"-list").classList.add("hide");var n=document.getElementById("list-header");"history"===e?(n.innerHTML=n.innerHTML.replace("24h","History"),u.clear(),u.leafletMap._onResize(),u.load(l)):(n.innerHTML=n.innerHTML.replace("History","24h"),a.leafletMap._onResize())}document.getElementById("history-toggle").onclick=function(){this.classList.add("hide"),d("history","current"),document.getElementById("current-toggle").classList.remove("hide")},document.getElementById("current-toggle").onclick=function(){this.classList.add("hide"),d("current","history"),document.getElementById("history-toggle").classList.remove("hide")},function t(){var e=("https:"===window.location.protocol?"wss://":"ws://")+window.location.host+"/ws/",n=new WebSocket(e);n.onopen=function(){},n.onmessage=function(e){var t,n=JSON.parse(e.data);console.log("New quakes ",n),s&&a.clear(),t=n,l.features=l.features.concat(t.features).sort(function(e,t){return moment(e.properties.datetime)-moment(t.properties.datetime)}),p(l.features.slice(-1)[0]),a.add(t),s=!1},n.onclose=function(e){console.log("Socket is closed. Reconnect will be attempted in 10 seconds.",e.reason),setTimeout(function(){s=!0,t()},1e4)},n.onerror=function(e){console.error("Socket encountered error: ",e.message,"Closing socket"),n.close()}}()},{"./current":2,"./history":3}]},{},[4]);
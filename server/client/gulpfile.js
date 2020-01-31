const browserify = require('browserify');
const gulp = require('gulp');
const log = require('gulplog');
const plumber = require('gulp-plumber');
const source = require('vinyl-source-stream');
const buffer = require("vinyl-buffer");
const uglify = require("gulp-uglify");
const del = require('del');
const watchify = require('watchify');

const config = {
    entries: [
        './app/js/main.js'
    ]
};

function bundle(bundler) {
    bundler
        .transform('babelify', { presets: ["@babel/preset-env"] } )
        .bundle()
        .on('error', log.error)
        .pipe(source('quakes.js'))
        .pipe(plumber())
        .pipe(buffer())
        .pipe(uglify())
        .pipe(gulp.dest('../static/quakes'));
}

function build(cb) {
    const bundler = browserify(config);
    bundle(bundler);

    cb();
}

exports.build = build;

function watch(cb) {
    const watcher = watchify(browserify(config, watchify.args));
    bundle(watcher);

    watcher.on('update', function() {
        bundle(watcher);
    });

    watcher.on('log', log.info);

    cb();
}

exports.watch = watch;

function clean(cb) {
    del.sync('../static/quakes/quakes.js');
    cb();
}

exports.clean = clean;
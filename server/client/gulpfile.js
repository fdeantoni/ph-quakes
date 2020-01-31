const browserify = require('browserify');
const gulp = require('gulp');
const log = require('gulplog');
const plumber = require('gulp-plumber');
const source = require('vinyl-source-stream');
const del = require('del');

function build(cb) {
    browserify({
        entries: [
            './app/js/main.js'
        ]
    })
        .transform('babelify', { presets: ["@babel/preset-env"] } )
        .bundle()
        .on('error', log.error)
        .pipe(source('quakes.js'))
        .pipe(plumber())
        .pipe(gulp.dest('../static/quakes'));

    cb();
}

exports.build = build;

function clean(cb) {
    del.sync('../static/quakes/quakes.js');
    cb();
}

exports.clean = clean;
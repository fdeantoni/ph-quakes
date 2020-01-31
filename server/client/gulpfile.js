const browserify = require('browserify');
const gulp = require('gulp');
const log = require('gulplog');
const plumber = require('gulp-plumber');
const source = require('vinyl-source-stream');
const buffer = require("vinyl-buffer");
const uglify = require("gulp-uglify");
const postcss = require("gulp-postcss");
const cssnano = require("cssnano");
const imagemin = require('gulp-imagemin');
const del = require('del');

const config = {
    entries: [
        './app/js/main.js'
    ]
};

function javascript(cb) {
    browserify(config)
        .transform('babelify', { presets: ["@babel/preset-env"] } )
        .bundle()
        .on('error', log.error)
        .pipe(source('quakes.js'))
        .pipe(plumber())
        .pipe(buffer())
        .pipe(uglify())
        .pipe(gulp.dest('../static/quakes'));

    cb();
}

exports.javascript = javascript;

function css(cb) {
    gulp.src("app/css/*")
        .pipe(plumber())
        .pipe( postcss([cssnano()] ) )
        .pipe(gulp.dest("../static/quakes"))
        .on('error', log.error);

    cb();
}

exports.css = css;

function image(cb) {
    gulp.src('app/images/*')
        .pipe(plumber())
        .pipe(imagemin())
        .pipe(gulp.dest('../static/quakes'))
        .on('error', log.error);

    cb();
}

exports.build = gulp.parallel(javascript, css, image);

exports.watch = function() {
    gulp.watch('app/css/*.css', css);
    gulp.watch('app/images/*', image);
    gulp.watch('app/js/*.js', gulp.series(clean, javascript));
};

function clean(cb) {
    del.sync('../static/quakes/*');
    cb();
}

exports.clean = clean;
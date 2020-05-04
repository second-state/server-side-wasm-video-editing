var http = require('http');
var formidable = require('formidable');
var fs = require('fs');
const url = require('url');
const cv = require('opencv4nodejs');

http.createServer(function(req, res) {
    var req_url = url.parse(req.url, true);
    console.log("Request URL: " + req_url.pathname);
    if (req_url.pathname == '/process.html') {
        var form = new formidable.IncomingForm();
        form.parse(req, function(err, fields, files) {
            const temp_path = files.filetoupload.path;
            console.log(temp_path);
            const filepath = '/media/nvme/image_processing/image_files/';
            original_video_location = filepath + files.filetoupload.name;
            console.log(original_video_location);
            fs.readFile(temp_path, function(rf_err, data) {
                if (rf_err) throw rf_err;
                console.log('File read!');
                fs.writeFile(original_video_location, data, function(wf_err) {
                    if (wf_err) throw wf_err;
                    console.log('File written!');
                    fs.unlink(temp_path, function(u_err) {
                        if (u_err) throw u_err;
                        console.log('File deleted!');
                    });
                });
            });
            setTimeout(function(){
            fs.readFile('process.html', 'utf8', function(err, p_data) {
                if (err) throw err;
                res.writeHead(200, {
                    'Content-Type': 'text/html'
                });
                //var source = p_data.toString();
                var regexp_original_video = `<source id="original_video_src" src="change_me.mp4" type="video/mp4">`;
                var regexp_processed_video = `<source id="processed_video_src" src="change_me.mp4" type="video/mp4">`;
                //var match = source.match(new RegExp(regexp, 'g');
                p_data = p_data.replace(new RegExp(regexp_original_video), '<source id="original_video" src="image_files/' + files.filetoupload.name + '" type="video/mp4">');
                p_data = p_data.replace(new RegExp(regexp_processed_video), '<source id="processed_video" src="modified_image_files/' + 'mod_' + files.filetoupload.name + '" type="video/mp4">');

                res.write(p_data);
                res.end();
            });
            // This timeout section is not really needed if just writing a copy of unedited vodeo to disk
            // This proves that the bottle neck is not disk io but the intense frame-by-frame, pixel-by-pixel CPU computation
            // For example one image with 300px by 200px with 3 values (RGB) will require 180, 000 separate calculations
            // 300px x 200px = 60,000px 
            // Each pixel has 3 values (Red, Green, Blue) which results in 180, 000 values to evaluate
          }, 50000);
        });

    } else if (req_url.pathname.endsWith('png')) {
        console.log("Accessing image files");
        const path = "." + req_url.pathname;
        const stat = fs.statSync(path)
        const fileSize = stat.size
        const head = {
            'Content-Length': fileSize,
            'Content-Type': 'image/png',
        }
        res.writeHead(200, head)
        fs.createReadStream(path).pipe(res)
    } else if (req_url.pathname.endsWith('svg')) {
        console.log("Accessing image files");
        const path = "." + req_url.pathname;
        const stat = fs.statSync(path)
        const fileSize = stat.size
        const head = {
            'Content-Length': fileSize,
            'Content-Type': 'image/svg+xml',
        }
        res.writeHead(200, head)
        fs.createReadStream(path).pipe(res)
    } else if (req_url.pathname.endsWith('mp4')) {
        console.log("Accessing video files");
        const path = "." + req_url.pathname;
        const stat = fs.statSync(path)
        const fileSize = stat.size
        const head = {
            'Content-Length': fileSize,
            'Content-Type': 'video/mp4',
        }
        res.writeHead(200, head)
        fs.createReadStream(path).pipe(res)
    } else {
        fs.readFile('index.html', function(rf_err2, data2) {
            res.writeHead(200, {
                'Content-Type': 'text/html'
            });
            res.write(data2);
            //res.end();
        });
    }

}).listen(8080);
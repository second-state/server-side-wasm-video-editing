extern crate notify;
extern crate num_cpus;
extern crate threadpool;
use notify::{watcher, DebouncedEvent, RawEvent, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};
// Multithreading
use std::thread;

use threadpool::ThreadPool;

use std::{io, path::PathBuf};

use std::any::type_name;

extern crate image;

use image::GenericImageView;


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn list_of_paths(root: String) -> io::Result<i32> {


    let mut dim_set: bool = false;
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    // Multithreading
    //let handle = thread::spawn(move || {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    let tx = tx.clone();
    pool.execute(move || {
    for entry in fs::read_dir(root).expect("asdf") {

        println!("Spawned thread");
        //println!("Processing a single image");
        let entry = entry.expect("asdf");
        let img = image::open(entry.path()).unwrap();
        if dim_set == false {
            let dim = img.dimensions();
            //println!("Width data type: {:?}", type_of(dim.0));
            width = dim.0;
            height = dim.1;
            dim_set = true;
        }

        // This part is where we will implement the SecondState server-side Wasm VM
        // We will have a new thread for each of these image files (entry in fs::read_dir)
        // We will pass the enture array of pixels to SSVM; each array as a separate Rust thread
        // If SSVM can process each array in a second or two then this whole process will be performed in mere seconds
        // At present using Rust and CPU in this fashion the whole process takes about 50 seconds for a 10 second video (25 fps)
        for h in 0..height {
            for w in 0..width {
                //println!("Processing width: {:?} and height: {:?}", w, h);
                let mut pix = img.get_pixel(w, h);
                // consider setting pixel here i.e. img.get_pixel once and then test performance difference
                for _x in 0..=2 {
                    /*
                    if 200 as i32 - pix[_x] as i32 > 0 {
                        pix[_x] = 200 - pix[_x];
                    }
                    */
                    pix[_x] = 1;
                }
            }
        }
        // Save
        println!("Saving image");
        img.save(entry.path()).unwrap();
    }
    });
    //handle.join().unwrap();
    Ok(1)
}

fn f<P>(paths: &[P])
where
    P: AsRef<Path>,
{
}

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_millis(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            "/media/nvme/image_processing/image_files",
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    loop {
        let event = rx.recv();
        match event {
            Ok(DebouncedEvent::Create(path)) => {
                //Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                println!("Path to uploaded video file: {:?}", path);
                let time_as_string = String::from(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        .to_string(),
                );
                let path_to_original_video = path.clone();
                let path_to_processing_images = Path::new("./processing_images")
                    .join(time_as_string)
                    .join("frame%04d.png");
                let clone_path_to_processing_images = path_to_processing_images.clone();
                let var_for_fun: String = clone_path_to_processing_images.parent().unwrap().as_os_str().to_str().unwrap().to_string();
                let clone_path_to_processing_images_2 = path_to_processing_images.clone();
                let clone_path_to_processing_images_3 = path_to_processing_images.clone();
                fs::create_dir(path_to_processing_images.parent().unwrap());
                println!("Path to processing images: {:?}", path_to_processing_images);
                let output = Command::new("ffmpeg")
                    .arg("-i")
                    .arg(path)
                    .arg(path_to_processing_images)
                    .output();
                //list_of_csv_paths(clone_path_to_processing_images.parent().unwrap().as_os_str().to_str().unwrap());
                list_of_paths(var_for_fun);
                let file_stem = path_to_original_video.file_stem();
                let extension = path_to_original_video.extension();
                let old_stem = file_stem.clone();
                let mut prefix = String::from("");
                //let mut prefix = String::from("mod_");
                prefix.push_str(
                    path_to_original_video
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                );
                prefix.push_str("/modified_image_files");
                prefix.push_str("/mod_");
                prefix.push_str(old_stem.unwrap().to_str().unwrap());
                prefix.push_str(".");
                prefix.push_str(extension.unwrap().to_str().unwrap());
                println!("Writing new file to: {:?}", prefix);
                let output_mod = Command::new("ffmpeg")
                    .arg("-i")
                    .arg(clone_path_to_processing_images_2)
                    .arg(prefix)
                    .output();
                //fs::remove_dir_all(clone_path_to_processing_images_3.parent().unwrap());
            }
            _ => {}
        }
    }
}

//! An example of opening an image.
extern crate image;
extern crate clap;
use clap::{App, Arg};

use std::fs;

use std::path::Path;

use image::{
    GenericImage
};

mod tilt_shift_module;

fn create_single_image(matches: clap::ArgMatches) {
    let file = matches.value_of("filename").unwrap();
    let blur = matches.value_of("blur_level").unwrap().parse::<f32>().unwrap();
    let saturation =  matches.value_of("saturation_level").unwrap().parse::<f32>().unwrap();

    let img = image::open(&Path::new(&file)).unwrap();
    let (_width, height) = img.dimensions();

    let output_file = matches.value_of("output_file_name").unwrap();


    let y_point_of_interest = if matches.is_present("yPointOfInterest") {
        matches.value_of("yPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };

    let height_point_of_interest = if matches.is_present("heightPointOfInterest") {
        matches.value_of("heightPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };

    tilt_shift_module::create_image(file, output_file, blur, saturation, y_point_of_interest, height_point_of_interest);
}

fn create_several_images(matches: clap::ArgMatches) {
    let file = matches.value_of("filename").unwrap();
    let output_file_folder = "output";
    fs::create_dir(output_file_folder);
    let saturation =  matches.value_of("saturation_level").unwrap().parse::<f32>().unwrap();

    let img = image::open(&Path::new(&file)).unwrap();
    let (_width, height) = img.dimensions();

    let y_point_of_interest = if matches.is_present("yPointOfInterest") {
        matches.value_of("yPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };

    let height_point_of_interest = if matches.is_present("heightPointOfInterest") {
        matches.value_of("heightPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };
    let range =  parse_params(matches.value_of("blur_level"));
    let blur_min = range[0];
    let blur_max = range[1];
    let mut current_blur = blur_min;
    let step = 1.0;

    while current_blur < blur_max {
        let output_file = format!("{}/{}_{}", output_file_folder, current_blur, matches.value_of("output_file_name").unwrap());
        tilt_shift_module::create_image(file, &output_file, current_blur, saturation, y_point_of_interest, height_point_of_interest);
        current_blur = current_blur + step;
    } 
}

fn parse_params(param: Option<&str>) -> Vec<f32> {
  let has_loop = param.unwrap().contains("..");  
  if has_loop  {
    let range: Vec<&str> = param.unwrap().split("..").collect();
    let min = range[0].parse::<f32>().unwrap();
    let max = range[1].parse::<f32>().unwrap();
     vec![ min, max]
  } else {
    return vec![ param.unwrap().parse::<f32>().unwrap() ];
  }
}



fn main() {
    let matches = App::new("Tilt Shift")
        .about("Compute the tilt-shift")
        .version("1.0")
        .author("Guillaume Gomez. <guillaume.gomez846@gmail.com>")
        .arg(Arg::with_name("filename")
                    .help("filename of the source image")
                    .short("f")
                    .long("filename")
                    .takes_value(true)
                    .required(true)
                    )
        .arg(Arg::with_name("blur_level")
                    .help("the amont of blur in the image")
                    .short("b")
                    .long("blur_level")
                    .takes_value(true)
                    .required(true))
        .arg(Arg::with_name("saturation_level")
                    .help("the level of saturation use in the image")
                    .short("s")
                    .long("saturation_level")
                    .takes_value(true)
                    .required(true))
        .arg(Arg::with_name("output_file_name")
                    .help("filename of the target image")
                    .short("o")
                    .long("output_file_name")
                    .takes_value(true)
                    .default_value("result.png"))
        .arg(Arg::with_name("yPointOfInterest")
                    .help("start of the focus zone")
                    .short("y")
                    .takes_value(true)
                    .long("yOrigin"))
        .arg(Arg::with_name("heightPointOfInterest")
                    .help("height of the focus zone")
                    .short("h")
                    .takes_value(true)
                    .long("height"))
        .get_matches();

    let has_loop = matches.value_of("blur_level").unwrap().contains("..");
    if has_loop {
        create_several_images(matches)
    } else {
        create_single_image(matches);  
    }

}
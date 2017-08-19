#[macro_use]
extern crate rosrust;
extern crate env_logger;

use rosrust::Ros;
use std::{thread, time};
use std::io;

rosmsg_include!();

fn main() {
    env_logger::init().unwrap();
    let mut ros = Ros::new("vel_publisher").unwrap();

    let mut publisher = ros.publish("cmd_vel").unwrap();

    // Really want to use below, but errors out for the moment, so using a
    // python script to map from "cmd_vel" to
    // "raspimouse/diff_drive_controller/cmd_vel" 
    // let mut publisher = ros.publish("/raspimouse/diff_drive_controller/cmd_vel").unwrap();

    println!("w: forward, s: backward, a: left, d: right >");

    loop {
        let mut vel_cmd = msg::geometry_msgs::Twist::new();
        thread::sleep(time::Duration::from_secs(1));

        let mut command = String::new();
        io::stdin().read_line(&mut command);

        match command.as_str() {
            "w\n" => vel_cmd.linear.x = 0.35,
            "s\n" => vel_cmd.linear.x = -0.35,
            "a\n" => vel_cmd.angular.z = 3.21,
            "d\n" => vel_cmd.angular.z = -3.21,
            "q\n" => break,
            _ => println!("Command not recognized"),
        }

        publisher.send(vel_cmd).unwrap();
    }
}

Teleop example written in Rust for the raspimouse developed by RT Corporation
---

This package was originally inspired by Takashi Ogura package found
[here](https://github.com/OTL/rosrust_tutorial)

First follow the instructions [here](https://github.com/rt-net/raspimouse_sim)
to get the packages that will enable to you simulate the raspimouse in Gazebo.


First clone the project:
```
$ git clone https://github.com/surfertas/rustymouse.git
```

The build the project:

```
$ cargo build
```


We first need to launch the simulation in Gazebo. (Make sure not to forget
`source devel/setup.bash`) Go to `~/raspiouse_gazebo/launch`

```
$ roscore&
$ roslaunch raspimouse_with_samplemaze.launch 
```

For the moment, ros.publish() doesnt accept the topic
`raspimouse/diff_drive_controller/cmd_vel` so we need to use a python script to
map between `cmd_vel` and the `raspimouse/diff_drive_controller/cmd_vel` topic.

Open a new window and go to `rustymouse/script` and run:
```
$ python topic_to_topic.py
```

Finally, run
```
$ cargo run --bin vel_publisher
```

You should be prompted to enter w,s,a,d. Use q to quit.



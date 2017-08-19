#!/usr/bin/env python
# license removed for brevity
import rospy
from geometry_msgs.msg import Twist

def callback(msg):
    pub = rospy.Publisher('/raspimouse/diff_drive_controller/cmd_vel', Twist, queue_size=10)
    pub.publish(msg)

def topic_mapper():
    rospy.init_node('mapper', anonymous=True)
    rospy.Subscriber("/cmd_vel", Twist, callback)
    rospy.spin()

if __name__ == '__main__':
    topic_mapper()
    

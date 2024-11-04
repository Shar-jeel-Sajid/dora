#!/usr/bin/env python
# -*- coding: utf-8 -*-
import dora
from dora import Node

ros2_context = dora.Ros2Context()
ros2_node = ros2_context.new_node(
    "dora",  # name
    "/piper",  # namespace
    dora.Ros2NodeOptions(rosout=True),
)

# Define a ROS2 QOS
topic_qos = dora.Ros2QosPolicies(reliable=True, max_blocking_time=0.1)

# Create a listener to pose topic
turtle_pose_topic = ros2_node.create_topic(
    "/puppet/joint_left", "sensor_msgs/JointState", topic_qos
)
pose_reader = ros2_node.create_subscription(turtle_pose_topic)

# Create a dora node
dora_node = Node()

# Listen for both stream on the same loop as Python does not handle well multiprocessing
dora_node.merge_external_events(pose_reader)

while True:
    event = dora_node.next()
    if event is None:
        break
    match event["kind"]:

        # In this case ROS2 event
        case "external":
            pose = event["value"][0].as_py()
            dora_node.send_output("/puppet/joint_left", event["value"])

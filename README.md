# hdmi_rest_server
A very simplistic REST API to wake up or shutdown the monitor using HDMI CEC

Requires cec-utils (sudo apt get install cec-utils)

Example of usage:
`curl -X POST http://<target_ip>:8181/screen/on` wakes up the monitor on the target device
`curl -X POST http://<target_ip>:8181/screen/off` shuts down the monitor on the target device
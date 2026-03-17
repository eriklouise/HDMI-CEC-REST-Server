# HDMI-CEC REST Server
A very simplistic REST API to wake up or shutdown the monitor using HDMI CEC


Requires cec-utils (sudo apt get install cec-utils)


Example of usage:\
`curl -X POST http://<target_ip>:8181/screen/on` wakes up the monitor on the target device\
`curl -X POST http://<target_ip>:8181/screen/off` shuts down the monitor on the target device


More advanced example using presence detection under Home Assistant:\
*Note: This is has been developped to light on/off a TV with a Raspberry Pi depending on the Occupancy attribute of motion sensor which is controlled by Home Assistant. The HDMI-CEC-REST-Server runs on the rasperry pi while Home Assistant sends POSTs ON/OFF when the Occupancy attribute of the motion sensor changes so that the TV screen is on only when there's someone in the room.*
1. Modify the Home Assistant principal configuration.yaml file. Add the following to the bottom of the file (or append an exisitng rest_command section):
```
rest_command:
  remotedevice_hdmi_on:
    url: "http://<target_ip>:8181/screen/on"
    method: POST
    headers:
      accept: "application/json, text/html"
      user-agent: "HAOS"
    verify_ssl: false
  remotedevice_hdmi_off:
    url: "http://<target_ip>:8181/screen/off"
    method: POST
    headers:
      accept: "application/json, text/html"
      user-agent: "HAOS"
    verify_ssl: false
```
2. Save and validate the configuration.yaml file uder developer-tools of Home Assistant then reboot.
3. Create the "ON" automation:
- Set the trigger to "occupancy becomes occupied" attribute of your motion sensor
- Set the action to `rest_command.magicmirror_hdmi_on`
4. Create the "OFF" automation:
- Set the trigger to "occupancy becomes unoccupied" attribute of your motion sensor
- Set the action to `rest_command.magicmirror_hdmi_off`
5. That's it ! The monitor of your target should wake up upon motion detection and shut down after 3mn of no motion detection.

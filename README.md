# Battery-charge
This util get total battery cap and charging status
and sends a notification when the battery is critically low
(if not one of the batteries is not charging)

## In The notification.toml can be configured:
  - title: title of the notification
  - text: text of the notification
  - time_period: farrowing interval of the charge
  - min_lifetime_percentage: total capacity which is considered low


1. Clone repo:
```git clone git@github.com:Mabarik667f/battery_charge.git```
  1.1 *Optional. Change notification.toml
2. Create daemon service and past battery_charge.service contents:
```sudo touch /etc/systemd/system/battery_charge.service```
3. Compile util:
```cargo build --release```
4. Change paths in battery_charge.service from binary file
5. Start daemon:
```systemctl daemon-reload```
```systemctl start battery_charge.service```
```systemctl enable battery_charge.service```
